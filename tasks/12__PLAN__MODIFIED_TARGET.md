# Plan for task 12: `modified` target and dates

## Design

- `EngineOptions` gets `now: Option<jiff::Timestamp>` (fixed for tests; `None` — the clock when the request arrives)
  and `time_zone` (the system zone by default, UTC in tests). The engine takes `now` once per request.
- `query/value.rs`: `parse_date(text, now, tz) -> Option<DateValue>` with `DateValue { start, end }` in seconds since
  the epoch (`f64`, sub-second precision kept):
  - `YYYY-MM-DD` → the day in the local zone: `[00:00, next day 00:00)` (DST-aware through `jiff`)
  - `YYYY-MM-DDTHH:MM` / `THH:MM:SS` (optional `Z` or `±HH:MM`; without an offset the local zone) → the minute / the
    second
  - `-N` + `s` / `m` / `h` / `d` / `w` → the moment `now - N·unit` (`start == end`)
  - `date_bounds(op, a, b)`: `=` `[a.start, a.end)`, `!=` negated, `>` `[a.end, ∞)`, `>=` `[a.start, ∞)`,
    `<` `(-∞, a.start)`, `<=` `(-∞, a.end)`, `in a..b` `[a.start, b.end)`; for a moment the ends are closed
    (`<=`, `>`, `=` work on the instant itself).
- validation: `invalid_date` — relative dates: hint `units: s, m, h, d, w`; others: hint
  `ISO 8601: 2026-10-01, 2026-10-01T14:30, 2026-10-01T14:30:00Z, 2026-10-01T14:30:00+02:00`; `invalid_range` when the
  start of `a` is after the start of `b`.
- `ParseOptions` carries `now` and the zone; `Filter::Number` gets the target `Modified`; `FileValues.modified` from
  `metadata().modified()` (the symlink itself for links).
- `query/subset.rs`: `modified` is supported; `Query.needs_metadata` covers it.
- `description.rs`: `modified > -7d`, date forms.

## Tests

- examples E12.1–E12.16 (fixture `dates`, `Now: 2026-10-08 12:00`)
- unit: `parse_date` for every form, offsets, a DST zone (a day of 23 hours), invalid dates; `date_bounds` for every
  operator with days and moments

## Steps

1. failing unit tests
2. options, date parsing, bounds, filter, validation, search
3. description

## Risks

- `jiff` is a new dependency (well known, by the author of `regex`); it brings the IANA zone handling the local time
  needs
