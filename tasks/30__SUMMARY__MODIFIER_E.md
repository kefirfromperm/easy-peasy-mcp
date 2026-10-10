# Summary of task 30: `/e` match escaped text

Server version: `0.30.0`.

## What was built

- `/e(style, levels)` on `=`, `has`, `starts`, `ends` on `text`, in file and line scope (QUERY.md#escaping): the
  value is written unescaped, the server matches its escaped forms on every requested level.
- Styles (QUERY.md#styles): `backslash` (`\`, `"`, `'` also unescaped for JSON, new line, tab), `quote_doubling`,
  `xml` (`'` as `&apos;` or `&#39;`), `url` (every character except `A–Z a–z 0–9 - . _ ~` as `%XX` of its UTF-8
  bytes, space also as `+`); styles and `any` in any case.
- Levels (QUERY.md#levels): a number, a range `a..b`, `any` = `0..3`; when several levels match at a position the
  lowest is reported.
- Combines with `/i` (on the escaped forms), `/w` (around the escaped match) and `/c`.
- Output (OUTPUT.md#highlight): `/eN` after the ids of a marked part, level 0 too; a part merged from several
  levels lists them ascending; identical fragments merge only with the same level.
- Errors: `unknown_value` for the style, `invalid_escape_level` for levels which are not a number, a range or `any`
  and for a level above 3 (at that number), `invalid_range` for a reversed range.
- The `find` description shows `/e`.
- Spec clarified first: the `url` characters, the reported level, the argument errors, the markers of levels.

## Architecture

- `query/escape.rs`: `Style`, `MAX_LEVEL`; `forms(style, c, level)` applies the one-character escaping `level`
  times to every form of the previous level; `pattern(style, value, levels)` is `(?:(level a)|(level a+1)|...)`, each
  level the concatenation of `(?:form|form)` per character.
- `query/mod.rs`: `validate_escape` / `escape_levels`; `compile_text` uses the escaped pattern in place of the
  literal, with the anchors of `=`, `starts`, `ends` around it.
- `query/eval.rs`: `TextPattern.escape_levels` and `escape_level(hay, start)`, which reads the matched capture group
  at the start of a match.
- `output/lines.rs`: `Fragment.escape`; marked parts (`Part`) collect the levels.
- `search/mod.rs`: fragments of file and line scope get their level.

## Deviations from the plan

None.

## Tests

- examples E30.1–E30.15 (fixture `escaped`): every style, levels 0–2, `any`, a range, `/i`, `'` in `backslash` and
  `xml`, `+` and `%20`, level 0 markers, the argument errors
- unit: the levels table and the forms of each style (`query::escape`), the reported level, `/e` with `=`,
  `starts`, `ends`, `/i`, `/w`, `/c` and in `line( ... )` (`query::tests::escaped_text`), the argument errors
  (`query::tests::escape_errors`), markers with levels (`output::lines::tests::escape_levels_after_the_ids`)

## Limitations and known gaps

- `escape.max_level` is fixed at 3 until task 61.
- The JSON `escape` field of fragments comes with the JSON output format, task 78.

## Try it

``{ "query": "name ~ '*.json' and text has `\"type\": \"admin\"` /e(backslash, any)" }``
