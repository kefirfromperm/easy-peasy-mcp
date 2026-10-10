# Plan for task 24: `/m` multiline regex

## Design

- `query/mod.rs`: `=~` on `text` with `/m` compiles with `multi_line(true)` (`^` / `$` at line starts and ends) and
  runs on the whole normalized content (`per_line: false`), so `\n` matches a line break; `.` doesn't match `\n`
  unless the regex has `(?s)` (the default of the regex crate).
- multiline fragments are rendered since task 19 (opened on the first line, closed on the last, every line a matched
  line); `to` in JSON comes with the JSON format (task 78).
- `modifier_not_allowed` for `/m` elsewhere, also inside `line( ... )` (since task 5).
- `query/subset.rs`: `/m` is supported.
- `description.rs`: `/m multiline regex`.

## Tests

- examples E24.1–E24.4 (fixture `tests_attr`)
- unit: `/m` regexes across lines, `^` / `$` at line borders, `.` without and with `(?s)`

## Steps

1. failing unit tests
2. compile, subset, description

## Risks

None known.
