# Summary of task 119: macOS specifics

Server version: `0.119.0`.

## What was built

- the `hidden` flag of macOS (`chflags hidden`) hides an entry like a name starting with `.`; `traversal.hidden =
  true` includes both
- `path_case = native` is case-insensitive on macOS and Windows (it was), now an option of the engine
  (`EngineOptions::native_case_insensitive`) with the platform's convention as the default
- already there and now shown by examples: decomposed (NFD) names match composed (NFC) queries, the config file in
  `~/Library/Application Support/easy-peasy-mcp/`, the creation date
- the examples are written with the Linux path case: the test server compares paths case-sensitively for `native`
  unless an example is marked with a platform (AGENTS.md), so the examples of all tasks hold on every platform

## Architecture

- `src/search/mod.rs`: `flagged_hidden` (`UF_HIDDEN` of `st_flags`, `false` off macOS) in a `filter_entry` of the
  walker when `traversal.hidden` is off; `HIDDEN_FLAG` says whether the platform has such a flag.
- `src/engine.rs`: `EngineOptions::native_case_insensitive`; `src/main.rs`: the test hook
  `EASY_PEASY_TEST_NATIVE_CASE=sensitive` of the startup examples.
- tests: the setup action `hide <path>` (`chflags hidden` on macOS, `attrib +h` on Windows, nothing on Linux); the
  library runner skips the examples of other platforms, as the startup runner did.

## Deviations from the plan

- `tests/examples.rs` ran the examples marked with another platform (only the startup runner skipped them): fixed in
  its own commit.

## Tests

- examples E119.1–E119.5 (fixture `macos`); E119.3 runs on every platform, the others on macOS only — their expected
  texts were checked on Linux with the same requests and `path_case = "insensitive"` / `hidden = true`
- `tests/search.rs`: `native_path_case_follows_the_option`, `native_path_case_is_the_platform_convention_by_default`
- unit (macOS only): `search::tests::the_hidden_flag_is_read`

## Limitations and known gaps

- the macOS code and examples were not compiled or run here (Linux, no macOS target): they first run in CI
  (task 118).

## Try it

On a Mac: `chflags hidden notes.txt`, then `find` with `name = 'notes.txt'` finds nothing until
`"traversal": {"hidden": true}`.
