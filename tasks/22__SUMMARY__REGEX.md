# Summary of task 22: Regex operator `=~`

Server version: `0.22.0`.

## What was built

- `=~` on `text`, `path` and `name` with the Rust `regex` syntax (QUERY.md#regex), linear time, `/i`; on `path` and
  `name` the regex finds a match anywhere (unanchored) and follows `path_case`.
- On `text` without `/m` the regex runs on every line separately: `^` and `$` are the line start and end, a match
  never crosses lines.
- Raw strings are the recommended form; quoted strings keep backslashes before other characters, so `'\bfoo\b'`
  works.
- `invalid_regex` (message `invalid regex`, the string marked) with the hints `look-arounds are not supported`,
  `backreferences are not supported`, or the problem named by the regex crate (`unclosed group`).
- The `find` description shows `=~`.

## Architecture

`user_regex` builds and checks query regexes; `Filter::Content.per_line`; `content_matches` gives the match ranges
of a content condition over the whole text or per line, for the evaluation and the fragments.

## Deviations from the plan

None.

## Tests

- examples E22.1–E22.10 (fixture `content`)
- unit: per-line anchors and no match across lines, `/i`, `name` / `path` regexes, the hints of `invalid_regex`

## Limitations and known gaps

- `/m` (task 24), `/w` (task 23)

## Try it

``{ "query": "text =~ `fn \\w+_test`" }``
