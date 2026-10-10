# Summary of task 12: `modified` target and dates

Server version: `0.12.0`.

## What was built

- `modified` with `=`, `!=`, `<`, `<=`, `>`, `>=` and `in a..b` (QUERY.md#dates).
- Absolute dates in ISO 8601: `2026-10-01` (the whole day), `2026-10-01T14:30` (the whole minute),
  `...T14:30:00` (the whole second), with `Z` or `±HH:MM`, otherwise in the server's local time zone with daylight
  saving; a fraction of a second is a moment. The precision of date-times and ranges is now in the spec.
- Relative dates `-N` with `s`, `m`, `h`, `d`, `w`: the moment `N` units before the search starts.
- The whole-day table of the spec: `=` during the day, `>` from the next day, `>=` from the start, `<` before,
  `<=` until the end; `in a..b` from the start of `a` to the end of `b`.
- `invalid_date` (hint with the units for relative dates, with the ISO forms otherwise), `invalid_range`.
- The `find` description shows dates.

## Architecture

- `EngineOptions` gets `now` (fixed in tests) and `time_zone` (the system zone, UTC in tests); `ParseOptions`
  carries them, the engine takes `now` once per request.
- `query/value.rs`: `parse_date` → `DateValue { start, end }` (seconds since the epoch, `start == end` for a moment)
  and `date_bounds(op, a, b)` → `Bounds`; `Filter::Number` with the target `Modified`; `FileValues.modified`.
- `search::visit` holds the per-entry work of the walk.
- new dependency `jiff` for time zones, DST and ISO 8601 parts.

## Deviations from the plan

- `search` was split into `search` and `visit` to stay under the function length lint.

## Tests

- examples E12.1–E12.16 (fixture `dates`, `Now: 2026-10-08 12:00`, files named so that newer files come first in
  path order as with relevance)
- unit: every date form, offsets, a day of 25 hours at the end of daylight saving in Berlin, relative units, invalid
  dates, intervals for days and moments

## Limitations and known gaps

None new.

## Try it

`{ "query": "modified > -7d and name ~ '*.rs'" }`
