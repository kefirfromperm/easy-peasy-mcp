# Plan for task 120: Windows specifics

## Design

- already there: output paths with the native separator (`MAIN_SEPARATOR` in `output`, `search`, `warning`,
  `hints`), `path_case = native` case-insensitive (task 119's option), the hidden attribute (the walker of the
  `ignore` crate reads it on Windows), junctions as links (`FileType::is_symlink` of the standard library counts
  every name-surrogate reparse point, junctions included), the `%APPDATA%` config directory and `%USERPROFILE%` for
  `~` (task 34), TOML strings in single quotes (`toml_edit`).
- `src/paths.rs` (new): `canonicalize` — `std::fs::canonicalize` without the `\\?\` prefix on Windows where the
  path means the same without it (a drive letter or a UNC share, no reserved names, no trailing dots or spaces, no
  `.`/`..`, shorter than 260 characters), like the `dunce` crate; every `canonicalize` of `src` goes through it, so
  absolute paths, warnings, errors and `get_config` never show the prefix.
- `src/query/mod.rs`: `ParseOptions::backslash_separator` (on Windows): in the values of `path` conditions — except
  regexes — `\` is a separator, read as `/`; `traversal.exclude` and the wildcards of `kinds` too
  (`query::slashed`).
- `src/search/mod.rs`: `file_id` on Windows — the volume serial number and the file index
  (`winapi_util::file::information` of a handle opened with `Handle::from_path_any`), a Windows-only dependency
  (`winapi-util`, already in the tree through `ignore`); `None` for a symlink which isn't followed, so a link isn't
  a duplicate of its target. Only the matched entries get an id.
- tests (AGENTS.md, in its own commit): on Windows an example which isn't marked with a platform is compared with
  every `\` read as `/`, `<fixture>` has no `\\?\` prefix; the setup action `junction`; the targets of `symlink`
  with `\` on Windows; the integration tests which spell out a directory header use `MAIN_SEPARATOR`.

Rejected: an option for the separator of the output in every function that writes a path (the conventions of
Linux in the examples on Windows) — a lot of plumbing for the tests only; reading `\` as `/` in the comparison costs
the examples nothing on Linux and macOS.

## Tests

- examples E120.1–E120.8 (fixtures `windows`, `macos`, `ignore-paths`), Windows only
- unit: `paths::tests` (the prefix rules, on every platform), `query::tests::backslash_is_a_separator_where_asked`
- `tests/search.rs`: `hard_links_are_returned_once_under_the_first_path` on every platform

## Steps

1. failing tests
2. `paths::canonicalize`
3. `\` in query paths
4. the file ids of Windows

## Risks

- nothing of Windows can run here (Linux, no Windows target): the code behind `cfg(windows)` is first compiled in CI
  (task 118).
