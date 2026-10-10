# Plan for task 40: `server.timeout`

## Design

- `src/config.rs`: `parse_duration` (`ms`, `s`, `m`, a whole number); `server.timeout` in the file values, in
  `Settings` (as written, with its source) and as a `Mutable` key; `--timeout` in `Settings::resolve`; `get_config`
  shows it as a TOML string.
- `src/main.rs`: `--timeout <duration>`; an invalid duration prints the usage.
- `src/request.rs`: the field `timeout`.
- `src/engine.rs`: the deadline is the arrival of the request plus the timeout of the request or of the settings;
  none for zero. The working directories are searched until the deadline; then the response gets
  `truncated: yes (timeout)` in its header and a `timeout` warning.
- `src/search/mod.rs`: `SearchOptions.deadline`; the walk stops (`WalkState::Quit`) when it is passed, a file read
  after it is dropped; `SearchResult.timed_out`. The test hook `EngineOptions.read_delay` makes every read of a
  file's content take that long, cut short at the deadline.
- `src/warning.rs`: `timeout`, a warning without paths (`warning: timeout`); `src/output/mod.rs`: the header.
- harness: `` Slow reads: `2s` `` sets the hook.

## Tests

- examples E40.1–E40.11 (fixture `timeout`)
- unit: `parse_duration`; the header and the warning line

## Steps

1. failing tests
2. key, request field, deadline, output

## Risks

- timing: the examples use a read delay ten times the timeout, and the file which is found before the timeout
  doesn't need to be read
