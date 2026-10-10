# Plan for task 15: `lang` target

## Design

- `languages.rs` (new): `Language { name, extensions, names, shebangs, source }` and `LanguageTable` built from the
  built-in table of CONFIG.md#built-in-languages, with lookup maps by lower case name, extension and interpreter;
  `detect(file_name, read_first_line)` by the rules written into CONFIG.md#languages: full name, extension,
  shebang (only for files without an extension; the first line only; `env` → its first non-option argument;
  exact interpreter, then without a trailing version). The comment syntax comes with `/c` (task 29).
- `query`: `Filter::Lang { id, order, values, negated }`; `FileValues.lang: &str` (`unknown` when no language);
  `Needs.lang`; `unknown_value` for a language which is not in the table nor `unknown`, hint `did you mean ...?
  expected one of: <all languages>, unknown`; the subset supports `lang`.
- `search`: the language is detected only when the query needs it; directories are `unknown`.
- `description.rs`: `lang = rust`.

## Tests

- examples E15.1–E15.9 (fixture `langs`)
- unit: detection by name, extension (case-insensitive), shebang with and without `env`, versions, `env -S`;
  a file with an extension is not read; `Filter::Lang`; `unknown_value` for languages

## Steps

1. failing unit tests
2. language table and detection, filter, validation, search
3. description

## Risks

None known.
