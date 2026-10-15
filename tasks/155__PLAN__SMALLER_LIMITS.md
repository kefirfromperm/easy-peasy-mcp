# Plan for task 155: Smaller default output limits

## Design

- `config/keys.rs`: the built-in defaults of spec 2 in `Key::default_value`: `max_files = 20`, `max_lines = 100`,
  `max_matches = 200`, `max_matches_per_file = 10`, `max_output_per_file = 800`, `max_output = 8000`
  (`max_matches_per_file` no longer shares its value with `result_cache`). `Key::default_for(1)` keeps the limits
  of spec 1, as it keeps `omit = []`. The engine already applies `default_for` to every key the configuration
  doesn't set: at the start of a session (the current spec version) and per request (its `version`), so
  `"version": 1`, the examples files written for spec 1 and the write tools (`output.max_lines` of `replace`) follow
  without more code.
- `engine.rs` `file_limits` (OUTPUT.md#limits-for-a-file): `max_files`, `max_lines`, `max_matches` and
  `max_matches_per_file` which neither the request nor the configuration sets (source `default`) take
  `default_for(1)` in a file; `max_output`, `max_output_per_file` and `budget` stay request only.
- `cursor.rs` `request_hash`: found while writing E155.1 — the next page of a request without `output` is
  `"output": { "cursor": ... }`, which hashes as `"output": {}` and fails with `cursor_mismatch`. With the old
  defaults a request without `output` was rarely cut; with the new ones it is the common case. An `output` left
  empty after removing `cursor` and `file` is removed too.
- no new response part: the header already names the limits (the options to raise) and the cursor
  (OUTPUT.md#how-to-get-more); the funnel of a huge result follows `max_files` (now from 201 files).

## Decisions

- the new limits are a part of the defaults of spec 2, like the `omit` default of task 141, rather than a spec 3:
  `"version": 1` keeps the old ones, so a pinned client sees no change
- a file keeps the old count limits: `output.file` is the way to take a big result out of the context, and 20 files
  would make it useless; a configured or requested value still applies
- the batch rule stays: `max_output` is split between the queries, 2000 characters each of four

## Tests

- examples E155.1–E155.9 (fixture `smaller-limits`); E130.13 and E141.12 (`get_config` in spec 2 files) get the
  new defaults; the examples files written for spec 1 are unchanged
- unit: `default_for(1)` and `default_for(2)` of the six limits, `max_line_length` unchanged; `request_hash` of a
  request without `output` equals the one of its next page (`output.cursor`) and of `output.file`

## Steps

1. tests (red): the unit tests above; the examples of task 155 fail on the old defaults
2. the cursor hash fix
3. the defaults per spec version and the limits of a file; E130.13, E141.12 updated
4. summary, version bump `0.148.0` → `0.149.0`

## Risks

- a client which relied on 100 files per response gets a cursor sooner: that is the point of the task; the
  configuration restores the old values
