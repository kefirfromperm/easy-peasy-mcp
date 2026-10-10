# Summary of task 31: `/n` any naming style

Server version: `0.31.0`.

## What was built

- `/n` on `text`, `name` and `path` with `=`, `has`, `starts`, `ends`, in file and line scope
  (QUERY.md#n--any-naming-style): the value is split into words and matches them in camelCase, PascalCase,
  snake_case, SCREAMING_SNAKE, kebab-case, dot.case, words and flat, always case-insensitive.
- Word splitting: spaces, `_`, `-`, `.` and case changes; an upper case run is one word (`HTTPServer` → `http`,
  `server`); digits stay with the preceding word (`utf8Decode` → `utf8`, `decode`).
- The words are joined by nothing or by one separator, the same between all words (spec clarified first).
- `/n/w` matches the whole identifier; `/n` with `~`, `~~`, `=~` and with `/e` is `modifier_not_allowed` (validation
  from task 1).
- The `find` description shows `/n`. With this task every modifier of the spec is implemented.

## Architecture

- `query/naming.rs`: `words(value)` and `pattern(value)` = `(?:ab|a_b|a\-b|a\.b|a b)` for the words `a`, `b`.
- `query/mod.rs`: `content_pattern` builds the `TextPattern` of a `text` condition (the regex of `=~`, or the value
  escaped by `/e`, in any naming style by `/n`, or literal, with the anchors of the operator); `compile_text` uses the
  naming pattern for `name` and `path` too and makes them case-insensitive.

## Deviations from the plan

- `compile_text` was split (`content_pattern`) to stay within the function length limit of clippy.

## Tests

- examples E31.1–E31.8 (fixture `naming`): every style, the value in another style with `=` in a line group,
  `HTTPServer`, `utf8Decode`, `/n/w`, file names, `/n` with `~` and with `/e`
- unit: `query::naming` (words of every style, upper case runs, digits, Unicode, the pattern), `/n` on `text`,
  `name`, `path` with each operator, `/n/w`, line scope (`query::tests::naming_styles`)

## Limitations and known gaps

None new.

## Try it

`{ "query": "kind = source and name has 'config loader' /n" }` — `kind` is task 16.
