# Plan for task 30: `/e` match escaped text

## Design

- `query/escape.rs` (new): `Style` (`backslash`, `quote_doubling`, `xml`, `url`) and `Levels`; the escaped forms of
  one character on one level (`escape_char`), the forms of a character on level `n` (escaping applied to every form of
  level `n - 1`), and the regex of a value: per level the concatenation of `(?:form|form)` per character, the levels as
  capture groups in ascending order — leftmost-first then reports the lowest level matching at a position.
  `MAX_LEVEL = 3` until `escape.max_level` is configurable (task 61).
- `query/mod.rs`: validation of the `/e` arguments — `unknown_value` for the style, `invalid_escape_level` for levels
  which are not a number, a range or `any` and for a number above the max level (at that number), `invalid_range`;
  `compile_text` builds the escaped regex for `/e`, `=` / `starts` / `ends` anchor the whole alternation.
- `query/eval.rs`: `TextPattern.escape_levels` — the level of every capture group; `escape_level(hay, start)` runs the
  captures at the start of a match.
- `output/lines.rs`: `Fragment.escape: Option<u8>`; identical fragments merge only with the same level; a marked part
  collects the levels of its fragments and writes `/eN` after the ids, ascending.
- `search/mod.rs`: fragments of conditions with `/e` get their level (file scope over the content, line scope over
  the line).
- `query/subset.rs`: `/e` is supported; `description.rs`: `/e(style, levels)`.

No new dependencies.

## Tests

- examples E30.1–E30.15 (fixture `escaped`)
- unit: the forms of each style on levels 0–2 (the table of QUERY.md#levels), `'` in `backslash`, `url` of non-ASCII
  and `%`, the lowest level at a position, `/w` and `/c` with `/e`, the argument errors, markers with levels

## Steps

1. failing unit tests and the examples
2. escape module, validation, compile, pattern levels
3. fragments with levels, markers, search
4. description

## Risks

- the number of forms grows with the level (`'` in `backslash` and `xml`, space in `url` have two forms on every
  level); with `MAX_LEVEL = 3` it stays below 16 forms per character
