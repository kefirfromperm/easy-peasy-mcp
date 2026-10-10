# Plan for task 119: macOS specifics

## Design

- already there: `path_case = native` is case-insensitive on macOS (`engine::NATIVE_CASE_INSENSITIVE`), names are
  compared in NFC form (task 47), the macOS config directory (`config::locate`, task 34), `created` from the
  metadata of the standard library (task 52).
- `src/search/mod.rs`: the `hidden` flag (`UF_HIDDEN` of `st_flags`) hides an entry on macOS unless
  `traversal.hidden`: a `filter_entry` of the walker which reads the flag (`flagged_hidden`, `false` on the other
  platforms); the walker of the `ignore` crate only knows the leading `.` there.
- `src/engine.rs`: `EngineOptions::native_case_insensitive` — what `path_case = native` means, the platform's
  convention by default; `main.rs` reads the test hook `EASY_PEASY_TEST_NATIVE_CASE=sensitive`.
- the examples run with the Linux path case unless marked with a platform (AGENTS.md, in its own commit):
  `tests/examples.rs` sets the option, `tests/startup.rs` the variable; the setup action `hide <path>` runs
  `chflags hidden` on macOS and `attrib +h` on Windows.

Rejected: marking every example which depends on the case of `native` with `Platform: linux` — they can't be found
without a Mac, and they test other things.

## Tests

- examples E119.1–E119.5 (fixture `macos`); E119.3 runs on every platform
- unit: `flagged_hidden` on macOS (`#[cfg(target_os = "macos")]`); `native_case_insensitive` in
  `tests/search.rs`: `native_path_case_follows_the_option`

## Steps

1. failing tests
2. the option of the native path case and its test hook
3. the hidden flag

## Risks

- the macOS examples and tests can't run here (Linux): they run in CI (task 118).
