# Plan for task 22: Regex operator `=~`

## Design

- `query/mod.rs`:
  - validation: every `=~` value (also outside of the subset) is compiled with the `regex` crate; an error is
    `invalid_regex`, message `invalid regex`, the string marked, hint: `look-arounds are not supported`,
    `backreferences are not supported`, or the problem named by the crate (`unclosed group`, ...)
  - `name` / `path` `=~`: the regex finds a match anywhere in the value (unanchored), case-insensitive with `/i` or an
    insensitive `path_case`
  - `text` `=~` without `/m`: `Filter::Content { per_line: true }` — the regex runs on every line separately, `^` and
    `$` are the line start and end, a match never crosses lines
- `query/eval.rs`: per-line evaluation and matches (the byte offsets of the lines in the normalized text).
- `query/subset.rs`: `=~` on `text`, `name`, `path`.
- `description.rs`: `=~` with a raw string.

## Tests

- examples E22.1–E22.10 (fixture `content`)
- unit: per-line matching (anchors, no match across lines), `name` / `path` regexes with `/i`, the hints of
  `invalid_regex`

## Steps

1. failing unit tests
2. validation, compile, per-line evaluation, subset
3. description

## Risks

None known: the regex crate runs in linear time.
