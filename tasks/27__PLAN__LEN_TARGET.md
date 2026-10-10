# Plan for task 27: `len` target

## Design

- `query/eval.rs`: `Filter::Len { id, order, bounds }`; in `eval_line` it compares the number of characters of the
  line (without the line break, a tab is one character); it is a contributing leaf without fragments, so a group of
  only `len` conditions gives whole-line fragments (task 26).
- `query/mod.rs`: the comparison operators and `in a..b` with non-negative integers (`value::bounds`); a value
  which is not an integer is `unexpected_token` with the hint `len takes a number of characters, e.g. len > 120`;
  `invalid_range` for `in 10..1`; `line_target_outside` outside of `line( ... )` / `near( ... )`
  (`'len' outside of line( ... )`, hint `'len' works only inside line( ... ) or near( ... )`).
- `query/subset.rs`: `len` is supported.
- `description.rs`: `len > 120 works inside line()`.

## Tests

- examples E27.1–E27.6 (fixture `lens`, the spec `app.conf` example)
- unit: `len` per line with tabs and non-ASCII, the errors

## Steps

1. failing unit tests
2. filter, validation, subset, description

## Risks

None known.
