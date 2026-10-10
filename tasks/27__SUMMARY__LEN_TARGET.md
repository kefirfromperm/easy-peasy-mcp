# Summary of task 27: `len` target

Server version: `0.27.0`.

## What was built

- `len` inside `line( ... )` (QUERY.md#targets): the length of the line in Unicode characters without the line
  break, a tab is one character; comparison operators and `in a..b` with non-negative integers.
- `line_target_outside` outside of a group; a value which is not an integer is `unexpected_token` with the hint
  `len takes a number of characters, e.g. len > 120`; `invalid_range` for a reversed range.
- `len` never produces fragments: a group of only `len` gives whole-line fragments, also empty ones for empty lines.
- The spec `app.conf` example works (E27.1); the `find` description shows `len`.

## Architecture

`Filter::Len { bounds }` evaluated in `eval_line`; `Needs.content` also for `len`; `validate_lens`; the value word
checks moved into `validate_words`; the renderer shows empty parts at the start of a line.

## Deviations from the plan

- the line renderer needed a fix for empty fragments of empty lines.

## Tests

- examples E27.1–E27.6 (fixture `lens`)
- unit: tabs, non-ASCII, empty lines, the errors, rendering an empty fragment

## Limitations and known gaps

- `len` inside `near( ... )` (task 28)

## Try it

`{ "query": "name ~ '*.py' and not line(len > 120)" }`
