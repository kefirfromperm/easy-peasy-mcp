# Summary of task 32: Fuzzy `~~` on `name` and `path`

Server version: `0.32.0`.

## What was built

- `~~` on `name` and `path` (QUERY.md#--fuzzy): the value is split at spaces into terms; the characters of every
  term must appear in the name or path in order, case-insensitive (on the NFC form).
- Every match has a score from 0 to 1; matches with a score below 0.5 are dropped (`query.fuzzy_min_score` is fixed
  until task 62). The score is not a relevance signal yet (task 93).
- `~~` on `text` stays `operator_not_allowed`; `/i` is allowed and changes nothing.
- The `find` description shows `~~`.
- Spec clarified first: "shorter" means the matched name or path; the terms are averaged.
- With `~~` the whole query language is implemented: the subset check (`query/subset.rs`), which answered
  `not_implemented` for the rest, is removed. `not_implemented` remains for request fields.

## The scoring function

For a haystack `h` (the name, or the path) of `n` characters:

- a **word start** is the first character, a character after `/`, `_`, `-`, `.` or a space, or an upper case letter
  after a lower case letter or a digit; a **word** runs from a word start to the next one
- in the **directory part** of a path (before the last `/`) every weight is multiplied by `0.8`
- a matched character weighs
  - `1.0` at a word start
  - otherwise `0.8` right after the previous matched character
  - otherwise `0.6` in the word of the previous matched character
  - otherwise `0.1` — a jump into the middle of a word (the first character of a term not at a word start too)
- the score of a term is the mean weight of its characters in the best alignment (dynamic programming, linear in `n`
  per character of the term); a term which doesn't appear in order fails the whole condition
- the score of the condition is `mean(term scores) · (0.8 + 0.2 · min(1, k / n))`, `k` the characters of all terms
- a value without terms matches every name with the score 1

Examples: `cfgldr` scores 0.70 on `config_loader.rs` (c, l, r at word starts), 0.64 on `ConfigLoader.java`, 0.40 on
`scaffold_bigolder.ts` (c and g in the middle of words); `srv hndl` scores 0.62 on `src/server/handlers.rs` and on
`services/http/handler.go`.

## Architecture

- `query/fuzzy.rs`: `Fuzzy` (terms, path or name), `score(haystack)`, `MIN_SCORE`; `Haystack` precomputes the lower
  case characters, word starts, word of each character and the directory factor.
- `query/eval.rs`: `TextMatcher` is an enum, `Regex` or `Fuzzy { fuzzy, min_score }`.
- `query/mod.rs`: `compile_text` builds the fuzzy matcher; `text_target`.

## Deviations from the plan

- The subset check was removed: after this task it rejected nothing.
- The fixture was changed twice while writing the tests: the examples keep a margin of at least 0.1 from the
  threshold and the files of one example have the same score.

## Tests

- examples E32.1–E32.6 (fixture `fuzzy`): the spec examples, a match below the threshold not returned, case, several
  terms, paths, `~~` on `text`
- unit: `query::fuzzy` (word starts, case changes, consecutive characters, jumps, directories, the length factor,
  several terms, a missing term, an empty value), `query::tests::fuzzy_names_and_paths`

## Limitations and known gaps

- The score as a relevance signal comes with task 93, `query.fuzzy_min_score` with task 62.

## Try it

`{ "query": "kind = source and name ~~ 'cfgload'" }`
