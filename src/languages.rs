//! The language table (CONFIG.md#languages) and the language of a file.

#[cfg(test)]
mod tests {
    use super::*;

    fn detect(name: &str, first_line: Option<&str>) -> Option<String> {
        let table = LanguageTable::builtin();
        let mut read = false;
        let lang = table
            .detect(name, || {
                read = true;
                first_line.map(str::to_string)
            })
            .map(|l| l.name.clone());
        if name.contains('.') && !name.starts_with('.') {
            assert!(!read, "{name}: a file with an extension is not read");
        }
        lang
    }

    #[test]
    fn by_extension_and_name() {
        assert_eq!(detect("main.rs", None).as_deref(), Some("rust"));
        assert_eq!(detect("MAIN.RS", None).as_deref(), Some("rust"));
        assert_eq!(detect("a.tar.hpp", None).as_deref(), Some("cpp"));
        assert_eq!(detect("Dockerfile", None).as_deref(), Some("dockerfile"));
        assert_eq!(detect("x.dockerfile", None).as_deref(), Some("dockerfile"));
        assert_eq!(detect("Gemfile", None).as_deref(), Some("ruby"));
        assert_eq!(detect(".env", None).as_deref(), Some("dotenv"));
        assert_eq!(detect("notes.txt", None), None);
    }

    #[test]
    fn by_shebang() {
        assert_eq!(
            detect("run", Some("#!/usr/bin/env python3")).as_deref(),
            Some("python")
        );
        assert_eq!(
            detect("run", Some("#!/usr/bin/python3.11 -u")).as_deref(),
            Some("python")
        );
        assert_eq!(detect("run", Some("#!/bin/bash")).as_deref(), Some("shell"));
        assert_eq!(
            detect("run", Some("#!/usr/bin/env -S node --flag")).as_deref(),
            Some("javascript")
        );
        assert_eq!(
            detect("run", Some("#! /usr/bin/perl -w")).as_deref(),
            Some("perl")
        );
        assert_eq!(detect("run", Some("echo no shebang")), None);
        assert_eq!(detect("run", None), None);
        assert_eq!(
            detect(".bashrc", Some("#!/bin/bash")).as_deref(),
            Some("shell")
        );
    }

    #[test]
    fn names_of_all_languages() {
        let table = LanguageTable::builtin();
        assert_eq!(table.names().len(), 41);
        assert!(table.get("RUST").is_some_and(|l| l.source));
        assert!(table.get("json").is_some_and(|l| !l.source));
    }
}
