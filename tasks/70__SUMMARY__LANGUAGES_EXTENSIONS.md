# Summary of task 70: `languages.<name>.extensions`

Server version: `0.70.0`.

## What was built

- Languages of the configuration (CONFIG.md#languages, clarified in its own commit): a section `[languages.<name>]`
  replaces the built-in language with the same name completely (its keys which are not written get the values of a
  new language) or adds a language. The key `extensions` of a language: from the config file, shown by `get_config`
  (a table per language, the built-in ones in the order of the spec, then the added ones), set by `update_config`
  with a section and reset by the name of the language, replaced for one request by the new request field
  `languages`, language by language.
- `lang`, `kind = source` and `/c` use the table with the changes; names and extensions are compared in any case.
- Errors: an extension in two languages (`the extension 'rs' is also in 'languages.rust'` at start and in
  `update_config`, `the extension 'RS' of 'languages.terraform.extensions' is also in 'languages.rust'` in a
  request), a name which is not a word (`invalid language name 'c++'`), a language which is not a section or an
  object, an unknown key (did you mean), a key not implemented yet, a value of a wrong type.

## Architecture

- `src/config/languages.rs` (new): the keys of a language, the implemented ones, `LanguageSetting`, `Problem` with its
  messages for the config file, `update_config` and the request, `from_toml`, `from_json`, `merge`, `render`,
  `changes`.
- `src/config/mod.rs`: `Settings.languages`, `FileValues.languages`; the config file check reads the language tables
  and checks the conflicts at the end (`check_language_conflicts`, with the line of the key); `get_config` shows the
  languages as the last section; `update_config` resets languages (`Resets`, `update_languages`) and sets them.
- `src/languages.rs`: `builtin_languages`.
- `src/request.rs`: `Request.languages`; `src/engine.rs`: `Settings::with` merges the request languages, the language
  table is built per request.
- `src/description.rs`: the `languages` object in the input schema.

## Deviations from the plan

- The full `get_config` examples of tasks 35–43 show the 41 language tables (separate commit, generated from the
  table of the spec).
- `update_config` was split into smaller functions for clippy (`update_arguments`, `update_languages`).

## Tests

- examples E70.1–E70.12 (fixture `languages`)
- unit: `config::languages::tests` (TOML and JSON, merging and conflicts, tables and changes), `request::tests::
  languages`, `config::tests::errors_with_lines`, `config::tests::update_config_changes_nothing_yet`

## Limitations and known gaps

- The other keys of a language come with tasks 71–77; until then a replaced built-in language loses them.

## Try it

`{ "query": "lang = terraform", "languages": { "terraform": { "extensions": ["tf", "tfvars"] } } }`
