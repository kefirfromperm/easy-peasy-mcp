# Summary of task 120: Windows specifics

Server version: `0.120.0`.

## What was built

- canonical paths never show the `\\?\` prefix where the path means the same without it (a drive or a UNC share, no
  device names, no trailing dots or spaces, at most 259 characters): absolute paths, directory headers, warnings,
  errors and `get_config`
- `\` is a separator in the values of `path` conditions on Windows (not in regexes), in `traversal.exclude` and in
  the wildcards of `kinds`; wildcard escapes use brackets on every platform (they did)
- duplicates are found by the volume serial number and the file index on Windows (a symlink which isn't followed is
  not a duplicate of its target), so a file reached through a junction, a symlink or a hard link is returned once
- already there and now shown by examples: `\` in output paths, the hidden attribute (the walker of the `ignore`
  crate reads it), junctions as `link` (the standard library counts them as symlinks), `%APPDATA%` and
  `%USERPROFILE%`, TOML paths in single quotes
- the examples of the Linux conventions run on Windows: a response is compared with every `\` read as `/` unless the
  example is marked with a platform; `<fixture>` has no `\\?\` prefix (AGENTS.md)

## Architecture

- `src/paths.rs` (new): `canonicalize` — `std::fs::canonicalize` and, on Windows, `simplified` (the rules of the
  `dunce` crate); every `canonicalize` of `src` goes through it.
- `src/query/mod.rs`: `ParseOptions::backslash_separator`, `BACKSLASH_SEPARATOR`, `slashed`, `path_wildcard`,
  `path_value`.
- `src/search/mod.rs`: `file_id(path, metadata)`; on Windows `winapi_util::file::information` of
  `Handle::from_path_any` — `winapi-util` is a Windows-only dependency, already in the tree through `ignore`.
- tests: `same_response`, `slashed_json` and `Exchange::slashed`; the setup action `junction`; symlink targets
  with `\` on Windows; the integration tests which write a directory header use `MAIN_SEPARATOR`, the hard link test
  runs on every platform.

## Deviations from the plan

None.

## Tests

- examples E120.1–E120.8 (fixtures `windows`, `macos`, `ignore-paths`), Windows only; E120.1, E120.3, E120.5 and
  E120.6 were checked on Linux with `/` for `\`
- unit: `paths::tests` (the prefix rules on every platform), `query::tests::backslash_is_a_separator_where_asked`,
  `search::tests::hard_links_have_the_same_file_id` (Windows)
- `tests/search.rs`: `hard_links_are_returned_once_under_the_first_path` on every platform

## Limitations and known gaps

- nothing of Windows was compiled or run here (Linux, no Windows target): the `cfg(windows)` code, the examples and
  the harness actions first run in CI (task 118).
- on Windows the file id costs opening every matched file once.

## Try it

On Windows: `find` with `path ~ `src\**\*.rs`` and `"output": {"path_style": "absolute"}` shows `C:\...` paths
without `\\?\`.
