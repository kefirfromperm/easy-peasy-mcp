//! The language table (CONFIG.md#languages) and the language of a file.

use std::collections::HashMap;

/// A language of the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    /// The value of `lang`, lower case.
    pub name: String,
    /// File extensions without the dot.
    pub extensions: Vec<String>,
    /// Full file names.
    pub names: Vec<String>,
    /// Interpreters in the shebang of files without an extension.
    pub shebangs: Vec<String>,
    /// Code (`kind = source`) or a data / document format.
    pub source: bool,
    /// Markers which start a comment until the end of the line.
    pub line_comments: Vec<String>,
    /// Start and end markers of block comments.
    pub block_comments: Vec<(String, String)>,
    /// Whether block comments can be nested.
    pub nested_block_comments: bool,
    /// String delimiters: comment markers inside strings are text.
    pub strings: Vec<String>,
}

/// `(line comments, block comments, nested, strings)`.
type CommentSyntax = (
    &'static [&'static str],
    &'static [(&'static str, &'static str)],
    bool,
    &'static [&'static str],
);

const C_BLOCK: &[(&str, &str)] = &[("/*", "*/")];
const XML_BLOCK: &[(&str, &str)] = &[("<!--", "-->")];
const QUOTES: &[&str] = &["\"", "'"];
const DOUBLE: &[&str] = &["\""];
const JS_STRINGS: &[&str] = &["\"", "'", "`"];

/// The comment syntax of a built-in language (CONFIG.md#built-in-languages).
fn comment_syntax(name: &str) -> CommentSyntax {
    match name {
        "c" | "cpp" | "csharp" | "java" | "groovy" | "scss" => (&["//"], C_BLOCK, false, QUOTES),
        "kotlin" | "scala" | "swift" | "dart" => (&["//"], C_BLOCK, true, QUOTES),
        "go" => (&["//"], C_BLOCK, false, &["\"", "`"]),
        "javascript" | "typescript" => (&["//"], C_BLOCK, false, JS_STRINGS),
        "rust" => (&["//"], C_BLOCK, true, DOUBLE),
        "php" => (&["//", "#"], C_BLOCK, false, QUOTES),
        "css" => (&[], C_BLOCK, false, QUOTES),
        "python" | "shell" | "perl" | "r" => (&["#"], &[], false, QUOTES),
        "ruby" => (&["#"], &[("=begin", "=end")], false, QUOTES),
        "powershell" => (&["#"], &[("<#", "#>")], false, QUOTES),
        "batch" => (&["REM", "::"], &[], false, DOUBLE),
        "sql" => (&["--"], C_BLOCK, false, QUOTES),
        "lua" => (&["--"], &[("--[[", "]]")], false, QUOTES),
        "haskell" => (&["--"], &[("{-", "-}")], true, DOUBLE),
        "clojure" | "lisp" => (&[";"], &[], false, DOUBLE),
        "html" | "xml" => (&[], XML_BLOCK, false, DOUBLE),
        "vue" => (&["//"], &[("<!--", "-->"), ("/*", "*/")], false, JS_STRINGS),
        "markdown" => (&[], XML_BLOCK, false, &[]),
        "yaml" | "toml" | "dotenv" | "dockerfile" | "makefile" => (&["#"], &[], false, DOUBLE),
        "ini" => (&["#", ";"], &[], false, DOUBLE),
        "properties" => (&["#", "!"], &[], false, DOUBLE),
        "jsonc" => (&["//"], C_BLOCK, false, DOUBLE),
        _ => (&[], &[], false, DOUBLE),
    }
}

/// `(name, extensions, names, shebangs, source)` of the built-in languages, in the order of the spec.
/// `(name, extensions, names, shebangs, source)`.
type BuiltinLanguage = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
    bool,
);

const BUILTIN: &[BuiltinLanguage] = &[
    ("c", &["c", "h"], &[], &[], true),
    (
        "cpp",
        &["cpp", "cc", "cxx", "hpp", "hh", "hxx"],
        &[],
        &[],
        true,
    ),
    ("csharp", &["cs"], &[], &[], true),
    ("java", &["java"], &[], &[], true),
    ("kotlin", &["kt", "kts"], &[], &[], true),
    ("scala", &["scala", "sc"], &[], &[], true),
    ("go", &["go"], &[], &[], true),
    ("swift", &["swift"], &[], &[], true),
    ("dart", &["dart"], &[], &[], true),
    ("groovy", &["groovy", "gradle"], &[], &[], true),
    (
        "javascript",
        &["js", "jsx", "mjs", "cjs"],
        &[],
        &["node"],
        true,
    ),
    ("typescript", &["ts", "tsx", "mts", "cts"], &[], &[], true),
    ("rust", &["rs"], &[], &[], true),
    ("php", &["php"], &[], &[], true),
    ("css", &["css"], &[], &[], true),
    ("scss", &["scss", "less"], &[], &[], true),
    ("python", &["py", "pyi"], &[], &["python", "python3"], true),
    ("ruby", &["rb"], &["Gemfile", "Rakefile"], &["ruby"], true),
    (
        "shell",
        &["sh", "bash", "zsh"],
        &[],
        &["sh", "bash", "zsh"],
        true,
    ),
    ("perl", &["pl", "pm"], &[], &["perl"], true),
    ("r", &["r"], &[], &[], true),
    ("powershell", &["ps1", "psm1"], &[], &[], true),
    ("batch", &["bat", "cmd"], &[], &[], true),
    ("sql", &["sql"], &[], &[], true),
    ("lua", &["lua"], &[], &[], true),
    ("haskell", &["hs"], &[], &[], true),
    ("clojure", &["clj", "cljs", "cljc"], &[], &[], true),
    ("lisp", &["lisp", "el", "scm"], &[], &[], true),
    ("html", &["html", "htm"], &[], &[], true),
    ("xml", &["xml", "svg", "xsd", "xsl"], &[], &[], false),
    ("vue", &["vue"], &[], &[], true),
    ("markdown", &["md", "markdown"], &[], &[], false),
    ("yaml", &["yml", "yaml"], &[], &[], false),
    ("toml", &["toml"], &[], &[], false),
    ("ini", &["ini", "cfg", "conf"], &[], &[], false),
    ("properties", &["properties"], &[], &[], false),
    ("dotenv", &["env"], &[".env"], &[], false),
    ("json", &["json"], &[], &[], false),
    ("jsonc", &["jsonc", "json5"], &[], &[], false),
    ("dockerfile", &["dockerfile"], &["Dockerfile"], &[], true),
    ("makefile", &["mk"], &["Makefile", "GNUmakefile"], &[], true),
];

/// The languages and the lookup maps by lower case name, extension and interpreter.
#[derive(Debug, Clone)]
pub struct LanguageTable {
    languages: Vec<Language>,
    by_name: HashMap<String, usize>,
    by_extension: HashMap<String, usize>,
    by_shebang: HashMap<String, usize>,
}

impl LanguageTable {
    /// The built-in table of CONFIG.md#built-in-languages.
    pub fn builtin() -> Self {
        let to_vec = |items: &[&str]| items.iter().map(ToString::to_string).collect();
        Self::new(
            BUILTIN
                .iter()
                .map(|(name, extensions, names, shebangs, source)| {
                    let (line, block, nested, strings) = comment_syntax(name);
                    Language {
                        name: (*name).to_string(),
                        extensions: to_vec(extensions),
                        names: to_vec(names),
                        shebangs: to_vec(shebangs),
                        source: *source,
                        line_comments: to_vec(line),
                        block_comments: block
                            .iter()
                            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
                            .collect(),
                        nested_block_comments: nested,
                        strings: to_vec(strings),
                    }
                })
                .collect(),
        )
    }

    /// A table of these languages.
    pub fn new(languages: Vec<Language>) -> Self {
        let mut by_name = HashMap::new();
        let mut by_extension = HashMap::new();
        let mut by_shebang = HashMap::new();
        for (i, lang) in languages.iter().enumerate() {
            for name in &lang.names {
                by_name.insert(name.to_lowercase(), i);
            }
            for ext in &lang.extensions {
                by_extension.insert(ext.to_lowercase(), i);
            }
            for interpreter in &lang.shebangs {
                by_shebang.insert(interpreter.clone(), i);
            }
        }
        Self {
            languages,
            by_name,
            by_extension,
            by_shebang,
        }
    }

    /// The names of all languages, in the order of the table.
    pub fn names(&self) -> Vec<&str> {
        self.languages.iter().map(|l| l.name.as_str()).collect()
    }

    /// The language of a file: by the full name, by the extension, or — only for a name without an extension — by
    /// the shebang of the first line, which `first_line` reads when needed.
    pub fn detect(
        &self,
        file_name: &str,
        first_line: impl FnOnce() -> Option<String>,
    ) -> Option<&Language> {
        let lower = file_name.to_lowercase();
        if let Some(&i) = self.by_name.get(&lower) {
            return Some(&self.languages[i]);
        }
        if let Some(dot) = lower.rfind('.').filter(|&i| i > 0) {
            return self
                .by_extension
                .get(&lower[dot + 1..])
                .map(|&i| &self.languages[i]);
        }
        let line = first_line()?;
        let interpreter = shebang_interpreter(&line)?;
        let i = self.by_shebang.get(interpreter).or_else(|| {
            let unversioned =
                interpreter.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
            self.by_shebang.get(unversioned)
        })?;
        Some(&self.languages[*i])
    }
}

/// The interpreter of a shebang line: the program, or the first argument of `env` which is not an option.
fn shebang_interpreter(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("#!")?.trim();
    let mut words = rest.split_whitespace();
    let program = words.next()?.rsplit('/').next()?;
    if program == "env" {
        words.find(|w| !w.starts_with('-') && !w.contains('='))
    } else {
        Some(program)
    }
}

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
        assert!(table.detect("a.rs", || None).is_some_and(|l| l.source));
        assert!(table.detect("a.json", || None).is_some_and(|l| !l.source));
    }
}
