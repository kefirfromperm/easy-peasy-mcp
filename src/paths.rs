//! Canonical paths in the form the agent reads: on Windows without the `\\?\` prefix where the path means the same
//! without it (OUTPUT.md#paths).

use std::path::{Component, Path, PathBuf};

/// `std::fs::canonicalize` in the form the agent reads: on Windows without the `\\?\` prefix where the path means
/// the same without it, like the `dunce` crate does.
pub fn canonicalize(path: &Path) -> std::io::Result<PathBuf> {
    let canonical = std::fs::canonicalize(path)?;
    if cfg!(windows)
        && let Some(plain) = canonical.to_str().and_then(simplified)
    {
        return Ok(PathBuf::from(plain));
    }
    Ok(canonical)
}

/// The canonical form of a path which needn't exist: `.` and `..` resolved without the file system, then its longest
/// existing part canonicalized and the rest appended.
#[must_use]
pub fn canonical_prefix(path: &Path) -> PathBuf {
    let normal = normalize(path);
    let mut existing = normal.as_path();
    let mut rest = Vec::new();
    loop {
        if let Ok(canonical) = canonicalize(existing) {
            return rest
                .iter()
                .rev()
                .fold(canonical, |path, name| path.join(name));
        }
        match (existing.parent(), existing.file_name()) {
            (Some(parent), Some(name)) => {
                rest.push(name.to_os_string());
                existing = parent;
            }
            _ => return normal,
        }
    }
}

/// A path with `.` and `..` resolved without the file system; `..` at the root stays there.
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => match out.components().next_back() {
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                Some(Component::RootDir | Component::Prefix(_)) => {}
                _ => out.push(".."),
            },
            other => out.push(other),
        }
    }
    out
}

/// The longest path Windows takes without the `\\?\` prefix: `MAX_PATH` without the terminating zero.
const MAX_PATH: usize = 259;

/// A verbatim path `\\?\C:\...` or `\\?\UNC\server\share\...` without its prefix where it means the same without
/// it: no `/` (a character of a verbatim path, a separator otherwise), no component `.` or `..`, none ending with a
/// dot or a space, no device name (`CON`, `NUL.txt`, ...), no character Windows reserves, at most `MAX_PATH`
/// characters. `None` otherwise, or without the prefix.
fn simplified(text: &str) -> Option<String> {
    let (plain, components) = if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        let mut parts = rest.split('\\');
        if parts.next()?.is_empty() || parts.next()?.is_empty() {
            return None;
        }
        (format!(r"\\{rest}"), rest)
    } else {
        let rest = text.strip_prefix(r"\\?\")?;
        let drive = rest.as_bytes();
        if drive.len() < 3
            || !drive[0].is_ascii_alphabetic()
            || drive[1] != b':'
            || drive[2] != b'\\'
        {
            return None;
        }
        (rest.to_string(), &rest[3..])
    };
    let plain_component = |c: &str| {
        let stem = c.split('.').next().unwrap_or(c).to_ascii_uppercase();
        let device = ["CON", "PRN", "AUX", "NUL"].contains(&stem.as_str())
            || (stem.len() == 4
                && (stem.starts_with("COM") || stem.starts_with("LPT"))
                && stem.ends_with(|ch: char| ch.is_ascii_digit()));
        c != "."
            && c != ".."
            && !c.ends_with(['.', ' '])
            && !c.contains(['/', '<', '>', ':', '"', '|', '?', '*'])
            && !device
    };
    let simple = components
        .split('\\')
        .filter(|c| !c.is_empty())
        .all(plain_component)
        && plain.encode_utf16().count() <= MAX_PATH;
    simple.then_some(plain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prefix_is_dropped_where_the_path_means_the_same() {
        assert_eq!(
            simplified(r"\\?\C:\Users\me\p").as_deref(),
            Some(r"C:\Users\me\p")
        );
        assert_eq!(simplified(r"\\?\c:\").as_deref(), Some(r"c:\"));
        assert_eq!(
            simplified(r"\\?\UNC\server\share\p").as_deref(),
            Some(r"\\server\share\p")
        );
    }

    #[test]
    fn the_prefix_stays_where_it_is_needed() {
        for verbatim in [
            r"C:\Users",
            r"\\?\C:\a\CON",
            r"\\?\C:\a\nul.txt",
            r"\\?\C:\a\b.",
            r"\\?\C:\a\b ",
            r"\\?\C:\a\..\b",
            r"\\?\C:\a\.\b",
            r"\\?\C:\a/b",
            r"\\?\C:",
            r"\\?\Volume{1234}\a",
            r"\\?\UNC\server",
        ] {
            assert_eq!(simplified(verbatim), None, "{verbatim}");
        }
        let long = format!(r"\\?\C:\{}", "a".repeat(300));
        assert_eq!(simplified(&long), None);
    }

    #[test]
    fn a_path_which_does_not_exist_yet() {
        let tmp = tempfile::tempdir().unwrap();
        let base = canonicalize(tmp.path()).unwrap();
        assert_eq!(
            canonical_prefix(&tmp.path().join("a/./b/../c")),
            base.join("a").join("c")
        );
        assert_eq!(
            canonical_prefix(&tmp.path().join("..")),
            base.parent().unwrap()
        );
        assert_eq!(normalize(Path::new("../a/..")), Path::new(".."));
        assert_eq!(normalize(Path::new("../..")), Path::new("../.."));
        assert_eq!(normalize(Path::new("/a/../../b")), Path::new("/b"));
    }

    #[test]
    #[cfg(not(windows))]
    fn elsewhere_the_canonical_path_of_the_standard_library() {
        let path = std::path::Path::new(".");
        assert_eq!(
            canonicalize(path).unwrap(),
            std::fs::canonicalize(path).unwrap()
        );
    }
}
