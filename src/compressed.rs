//! Compressed single files, searched as their decompressed content (CONFIG.md#archives).

use std::io::Read;

/// A compression format of a single file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    Lz4,
}

impl Compression {
    /// The compression of a file by the extension of its name, in any case: `.gz` and `.tgz`, `.bz2`, `.xz`, `.zst`,
    /// `.lz4`.
    pub fn of(name: &str) -> Option<Self> {
        let (_, extension) = name.rsplit_once('.')?;
        match extension.to_ascii_lowercase().as_str() {
            "gz" | "tgz" => Some(Self::Gzip),
            "bz2" => Some(Self::Bzip2),
            "xz" => Some(Self::Xz),
            "zst" => Some(Self::Zstd),
            "lz4" => Some(Self::Lz4),
            _ => None,
        }
    }
}

/// The decompressed content of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decompressed {
    /// All of it.
    Bytes(Vec<u8>),
    /// It is longer than the limit.
    TooBig,
}

/// A reader of the decompressed content of `source`.
pub fn reader<'a>(
    compression: Compression,
    source: impl Read + 'a,
) -> std::io::Result<Box<dyn Read + 'a>> {
    Ok(match compression {
        Compression::Gzip => Box::new(flate2::read::MultiGzDecoder::new(source)),
        Compression::Bzip2 => Box::new(bzip2::read::MultiBzDecoder::new(source)),
        Compression::Xz => Box::new(lzma_rust2::XzReader::new(source, true)),
        Compression::Zstd => Box::new(
            ruzstd::decoding::StreamingDecoder::new(source)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
        ),
        Compression::Lz4 => Box::new(lz4_flex::frame::FrameDecoder::new(source)),
    })
}

/// Decompresses `source` in memory; the content beyond `limit` bytes is not decompressed.
///
/// # Errors
///
/// The data is not valid in the format: corrupted, truncated, or of another format.
pub fn decompress(
    compression: Compression,
    source: impl Read,
    limit: u64,
) -> std::io::Result<Decompressed> {
    let mut bytes = Vec::new();
    reader(compression, source)?
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Ok(Decompressed::TooBig);
    }
    Ok(Decompressed::Bytes(bytes))
}

/// The first `n` bytes of the decompressed content, fewer when it is shorter.
///
/// # Errors
///
/// The data is not valid in the format.
pub fn head(compression: Compression, source: impl Read, n: u64) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader(compression, source)?
        .take(n)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compressions_by_the_extension() {
        assert_eq!(Compression::of("app.log.gz"), Some(Compression::Gzip));
        assert_eq!(Compression::of("BACKUP.TGZ"), Some(Compression::Gzip));
        assert_eq!(Compression::of("a.tar.bz2"), Some(Compression::Bzip2));
        assert_eq!(Compression::of("a.xz"), Some(Compression::Xz));
        assert_eq!(Compression::of("a.zst"), Some(Compression::Zstd));
        assert_eq!(Compression::of("a.lz4"), Some(Compression::Lz4));
        assert_eq!(Compression::of("a.zip"), None);
        assert_eq!(Compression::of("gz"), None);
    }

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/compressed/logs")
                .join(name),
        )
        .unwrap()
    }

    #[test]
    fn every_format_is_decompressed() {
        for format in ["gz", "bz2", "xz", "zst", "lz4"] {
            let name = format!("app.log.{format}");
            let compression = Compression::of(&name).unwrap();
            let bytes = fixture(&name);
            assert_eq!(
                decompress(compression, bytes.as_slice(), 1000).unwrap(),
                Decompressed::Bytes(
                    format!("started\n{format} request req-7f3a9c done\n").into_bytes()
                ),
                "{format}"
            );
        }
    }

    #[test]
    fn the_limit_and_broken_data() {
        let bytes = fixture("app.log.gz");
        let full = "started\ngz request req-7f3a9c done\n".len() as u64;
        assert!(matches!(
            decompress(Compression::Gzip, bytes.as_slice(), full).unwrap(),
            Decompressed::Bytes(_)
        ));
        assert_eq!(
            decompress(Compression::Gzip, bytes.as_slice(), full - 1).unwrap(),
            Decompressed::TooBig
        );
        assert_eq!(
            head(Compression::Gzip, bytes.as_slice(), 7).unwrap(),
            b"started"
        );
        let broken = fixture("broken.gz");
        assert!(decompress(Compression::Gzip, broken.as_slice(), 1000).is_err());
        for compression in [
            Compression::Bzip2,
            Compression::Xz,
            Compression::Zstd,
            Compression::Lz4,
        ] {
            assert!(
                decompress(compression, &b"not compressed"[..], 1000).is_err(),
                "{compression:?}"
            );
        }
    }
}
