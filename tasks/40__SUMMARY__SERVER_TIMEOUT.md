# Summary of task 40: `server.timeout`

Server version: `0.40.0`.

## What was built

- `server.timeout` (CONFIG.md#server, TOOLS.md#timeout), default `"30s"`: from the config file, `--timeout
  <duration>` (before the file), `update_config` (set and reset); `get_config` shows it as written with its source.
- The request field `timeout` replaces it for one request.
- Durations (spec clarified first): a whole number and `ms`, `s` or `m`; zero — no limit. Invalid: `invalid value for
  'server.timeout': a duration like "30s" is expected (units ms, s, m)` at start, the same hint for `invalid_value`,
  `error[invalid_option]: 'timeout' must be a duration like "10s" (units ms, s, m)` for a request, the usage for
  `--timeout`.
- The time counts from the arrival of the request. At the deadline the walk stops and a file read after it is
  dropped; the response has the files found so far, `truncated: yes (timeout)` in the header and the warning
  `warning: timeout` (a warning without paths, spec clarified first); no cursor. A timeout is not an error.
- The `find` description and schema have `timeout`.

## Architecture

- `src/config.rs`: `parse_duration`, `CommandLine` (`--root`, `--timeout`), `Settings.timeout` with its source and
  `Settings::timeout()`, `Mutable::Timeout`.
- `src/engine.rs`: the deadline from the arrival; working directories are searched until it passes;
  `EngineOptions.read_delay` — the test hook.
- `src/search/mod.rs`: `SearchOptions.deadline` / `read_delay`, `Walk::past_deadline` (quits the walk),
  `Walk::slow_read`, `SearchResult.timed_out`.
- `src/warning.rs`: `timeout`, `Warnings::add_global`; `src/output/mod.rs`: the header.
- harness: `` Slow reads: `2s` `` (AGENTS.md).

## Deviations from the plan

- The example found before the timeout is a directory: a file whose name decides the query is still read for its
  fragments, so with slow reads it could be found or not.

## Tests

- examples E40.1–E40.11 (fixture `timeout`)
- unit: `config::tests::durations`, the timeout in `roots_are_resolved`, `warning::tests::warnings_without_paths`

## Limitations and known gaps

- JSON `counts_exact: false` and `truncated_by` come with the JSON format (task 78).

## Try it

`{ "timeout": "2s", "query": "text has 'TODO'" }`
