//! Archives with members, searched with `archives = all` (CONFIG.md#archives).

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::compressed::Compression;

/// An archive format with members.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    /// `.zip`, `.jar`.
    Zip,
    /// `.tar`, or a compressed tar: `.tar.gz` / `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`.
    Tar(Option<Compression>),
    /// `.7z`.
    SevenZ,
}

impl ArchiveKind {
    /// The archive format of a file by the end of its name, in any case.
    pub fn of(name: &str) -> Option<Self> {
        let lower = name.to_ascii_lowercase();
        let ends = |suffix: &str| lower.len() > suffix.len() && lower.ends_with(suffix);
        if ends(".zip") || ends(".jar") {
            Some(Self::Zip)
        } else if ends(".tar") {
            Some(Self::Tar(None))
        } else if ends(".tar.gz") || ends(".tgz") {
            Some(Self::Tar(Some(Compression::Gzip)))
        } else if ends(".tar.bz2") {
            Some(Self::Tar(Some(Compression::Bzip2)))
        } else if ends(".tar.xz") {
            Some(Self::Tar(Some(Compression::Xz)))
        } else if ends(".tar.zst") {
            Some(Self::Tar(Some(Compression::Zstd)))
        } else if ends(".7z") {
            Some(Self::SevenZ)
        } else {
            None
        }
    }
}

/// A file member of an archive.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The path inside the archive as stored, never resolved against the file system.
    pub path: String,
    /// The uncompressed size.
    pub size: u64,
    /// The modification date in seconds since the Unix epoch, if the entry has one.
    pub modified: Option<f64>,
}

/// Reads the file members of an archive in their order: `visit` gets every member and a reader of its content, which
/// it may leave unread, and returns `false` to stop. Directories, links and other entries are not members.
///
/// # Errors
///
/// The archive can't be read: corrupted, encrypted, or of an unsupported method.
pub fn for_each_member(
    path: &Path,
    kind: ArchiveKind,
    time_zone: &jiff::tz::TimeZone,
    visit: &mut dyn FnMut(&Member, &mut dyn Read) -> bool,
) -> std::io::Result<()> {
    match kind {
        ArchiveKind::Zip => zip_members(path, time_zone, visit),
        ArchiveKind::Tar(compression) => {
            let file = BufReader::new(File::open(path)?);
            let source: Box<dyn Read> = match compression {
                Some(compression) => crate::compressed::reader(compression, file)?,
                None => Box::new(file),
            };
            tar_members(source, visit)
        }
        ArchiveKind::SevenZ => seven_z_members(path, visit),
    }
}

fn invalid(error: impl std::error::Error + Send + Sync + 'static) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, error)
}

fn zip_members(
    path: &Path,
    time_zone: &jiff::tz::TimeZone,
    visit: &mut dyn FnMut(&Member, &mut dyn Read) -> bool,
) -> std::io::Result<()> {
    let mut archive = zip::ZipArchive::new(BufReader::new(File::open(path)?)).map_err(invalid)?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(invalid)?;
        if !entry.is_file() {
            continue;
        }
        let utc = entry.extra_data_fields().find_map(|field| match field {
            zip::extra_fields::ExtraField::ExtendedTimestamp(ts) => ts.mod_time(),
            _ => None,
        });
        let modified = match utc {
            Some(seconds) => Some(f64::from(seconds)),
            None => entry.last_modified().and_then(|dos| {
                let local = jiff::civil::date(
                    i16::try_from(dos.year()).ok()?,
                    i8::try_from(dos.month()).ok()?,
                    i8::try_from(dos.day()).ok()?,
                )
                .at(
                    i8::try_from(dos.hour()).ok()?,
                    i8::try_from(dos.minute()).ok()?,
                    i8::try_from(dos.second()).ok()?,
                    0,
                );
                #[allow(clippy::cast_precision_loss)]
                // dates of zip files are far below 2^53 seconds
                local
                    .to_zoned(time_zone.clone())
                    .ok()
                    .map(|z| z.timestamp().as_second() as f64)
            }),
        };
        let member = Member {
            path: entry.name().map_err(invalid)?.into_owned(),
            size: entry.size(),
            modified,
        };
        if !visit(&member, &mut entry) {
            break;
        }
    }
    Ok(())
}

fn tar_members(
    source: impl Read,
    visit: &mut dyn FnMut(&Member, &mut dyn Read) -> bool,
) -> std::io::Result<()> {
    let mut archive = tar::Archive::new(source);
    for entry in archive.entries()? {
        let mut entry = entry?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        #[allow(clippy::cast_precision_loss)] // dates of tar files are far below 2^53 seconds
        let member = Member {
            path: String::from_utf8_lossy(&entry.path_bytes()).into_owned(),
            size: entry.header().size()?,
            modified: entry.header().mtime().ok().map(|m| m as f64),
        };
        if !visit(&member, &mut entry) {
            break;
        }
    }
    Ok(())
}

/// The difference between the epochs of `FILETIME` (1601) and Unix (1970), in seconds.
const FILETIME_UNIX_EPOCH: f64 = 11_644_473_600.0;

fn seven_z_members(
    path: &Path,
    visit: &mut dyn FnMut(&Member, &mut dyn Read) -> bool,
) -> std::io::Result<()> {
    let mut archive = sevenz_rust2::ArchiveReader::open(path, sevenz_rust2::Password::empty())
        .map_err(invalid)?;
    archive
        .for_each_entries(|entry, reader| {
            if entry.is_directory || entry.is_anti_item {
                return Ok(true);
            }
            #[allow(clippy::cast_precision_loss)]
            // 100 ns intervals of real dates are far below 2^63
            let modified = entry
                .has_last_modified_date
                .then(|| u64::from(entry.last_modified_date) as f64 / 1e7 - FILETIME_UNIX_EPOCH);
            let member = Member {
                path: entry.name.clone(),
                size: entry.size,
                modified,
            };
            Ok(visit(&member, reader))
        })
        .map_err(invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archives_by_the_name() {
        assert_eq!(ArchiveKind::of("logs/a.zip"), Some(ArchiveKind::Zip));
        assert_eq!(ArchiveKind::of("lib.JAR"), Some(ArchiveKind::Zip));
        assert_eq!(ArchiveKind::of("a.tar"), Some(ArchiveKind::Tar(None)));
        assert_eq!(
            ArchiveKind::of("a.tar.gz"),
            Some(ArchiveKind::Tar(Some(Compression::Gzip)))
        );
        assert_eq!(
            ArchiveKind::of("a.tgz"),
            Some(ArchiveKind::Tar(Some(Compression::Gzip)))
        );
        assert_eq!(
            ArchiveKind::of("a.tar.zst"),
            Some(ArchiveKind::Tar(Some(Compression::Zstd)))
        );
        assert_eq!(ArchiveKind::of("a.7z"), Some(ArchiveKind::SevenZ));
        assert_eq!(ArchiveKind::of("a.gz"), None);
        assert_eq!(ArchiveKind::of(".zip"), None);
    }

    /// The members of an archive of the fixture `archives` with their content.
    fn members(name: &str) -> Vec<(String, u64, Option<f64>, String)> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/archives/logs")
            .join(name);
        let kind = ArchiveKind::of(name).unwrap();
        let mut out = Vec::new();
        for_each_member(&path, kind, &jiff::tz::TimeZone::UTC, &mut |m, r| {
            let mut bytes = Vec::new();
            r.read_to_end(&mut bytes).unwrap();
            let text = String::from_utf8_lossy(&bytes).into_owned();
            out.push((m.path.clone(), m.size, m.modified, text));
            true
        })
        .unwrap();
        out
    }

    #[test]
    fn members_of_every_format() {
        // 2026-10-01 10:00 UTC
        let modified = Some(1_790_848_800.0);
        for (name, format) in [
            ("a.zip", "zip"),
            ("a.jar", "jar"),
            ("a.tar", "tar"),
            ("a.tar.gz", "tar.gz"),
            ("a.tar.bz2", "tar.bz2"),
            ("a.tar.xz", "tar.xz"),
            ("a.tar.zst", "tar.zst"),
            ("a.7z", "7z"),
        ] {
            let text = format!("started\n{format} request req-7f3a9c done\n");
            let found = members(name);
            assert_eq!(
                found[0],
                (
                    format!("inner/{}.log", format.replace('.', "-")),
                    text.len() as u64,
                    modified,
                    text
                ),
                "{name}"
            );
        }
    }

    #[test]
    fn a_broken_archive() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/archives/logs/broken.zip");
        assert!(
            for_each_member(
                &path,
                ArchiveKind::Zip,
                &jiff::tz::TimeZone::UTC,
                &mut |_, _| true
            )
            .is_err()
        );
    }
}
