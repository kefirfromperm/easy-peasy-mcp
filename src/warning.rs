//! Problems which don't make a request fail (OUTPUT.md#warnings).

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sep() -> char {
        std::path::MAIN_SEPARATOR
    }

    #[test]
    fn lines_show_three_paths_and_the_rest() {
        let mut w = Warnings::default();
        for name in ["e", "b", "a", "d", "c"] {
            w.add(WarningCode::DateUnsupported, PathBuf::from(name), false);
        }
        w.add(WarningCode::DateUnsupported, PathBuf::from("dir"), true);
        assert_eq!(
            w.to_text(),
            format!("warning: date_unsupported (6): a, b, c, … 3 more\n")
        );
        let mut one = Warnings::default();
        one.add(WarningCode::DateUnsupported, PathBuf::from("dir"), true);
        assert_eq!(
            one.to_text(),
            format!("warning: date_unsupported (1): dir{}\n", sep())
        );
        assert!(Warnings::default().is_empty());
    }
}
