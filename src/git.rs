//! The git status of files (QUERY.md#git), read by the `git` program once per repository.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn porcelain_v2_records() {
        let out = "1 .M N... 100644 100644 100644 a a modified.txt\0\
                   1 M. N... 100644 100644 100644 a b staged file.txt\0\
                   1 MM N... 100644 100644 100644 a b sub/both.txt\0\
                   u UU N... 100644 100644 100644 100644 a b c conflict.txt\0\
                   ? untracked.txt\0\
                   ! ignored.txt\0";
        let statuses = parse_status(out.as_bytes());
        assert_eq!(
            statuses["modified.txt"],
            GitStatus {
                modified: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["staged file.txt"],
            GitStatus {
                staged: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["sub/both.txt"],
            GitStatus {
                modified: true,
                staged: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["conflict.txt"],
            GitStatus {
                conflicted: true,
                ..GitStatus::default()
            }
        );
        assert_eq!(
            statuses["untracked.txt"],
            GitStatus {
                untracked: true,
                ..GitStatus::default()
            }
        );
        assert!(!statuses.contains_key("ignored.txt"));
    }

    #[test]
    fn values() {
        let both = GitStatus {
            modified: true,
            staged: true,
            ..GitStatus::default()
        };
        assert!(
            both.has(GitValue::Modified)
                && both.has(GitValue::Staged)
                && both.has(GitValue::Changed)
        );
        assert!(!both.has(GitValue::Clean) && !both.has(GitValue::Untracked));
        let clean = GitStatus::default();
        assert!(clean.has(GitValue::Clean) && !clean.has(GitValue::Changed));
        assert_eq!(GitValue::from_keyword("CHANGED"), Some(GitValue::Changed));
        assert_eq!(GitValue::from_keyword("dirty"), None);
    }

    #[test]
    fn repositories_are_found_per_file() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("outer/.git")).unwrap();
        std::fs::create_dir_all(root.join("outer/inner/src")).unwrap();
        std::fs::write(root.join("outer/inner/.git"), "gitdir: x\n").unwrap();
        assert_eq!(find_root(&root.join("outer/a")), Some(root.join("outer")));
        assert_eq!(
            find_root(&root.join("outer/inner/src/b")),
            Some(root.join("outer/inner"))
        );
        assert_eq!(
            find_root(&root.join("c")).filter(|r| r.starts_with(root)),
            None
        );
    }
}
