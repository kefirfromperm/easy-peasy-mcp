# Summary of task 24: `/m` multiline regex

Server version: `0.24.0`.

## What was built

- `/m` with `=~` on `text` in file scope (QUERY.md#regex): the regex runs on the whole content, `\n` matches a line
  break, `^` / `$` match at line starts and ends, `.` doesn't match `\n` unless `(?s)`.
- Multiline fragments open on their first line and close on their last; every line of the match is a matched line.
- `modifier_not_allowed` for `/m` elsewhere; the `find` description shows `/m`.

## Architecture

`compile_text` builds `=~` regexes with `multi_line` for `/m` and turns `per_line` off.

## Deviations from the plan

None.

## Tests

- examples E24.1–E24.4 (fixture `tests_attr`, the spec example)
- unit: line breaks, anchors, `.` with and without `(?s)`

## Limitations and known gaps

- `to` of multiline fragments in JSON comes with the JSON format (task 78)

## Try it

``{ "query": "text =~ `#\\[test\\]\\s*\\n\\s*fn \\w+_slow` /m" }``
