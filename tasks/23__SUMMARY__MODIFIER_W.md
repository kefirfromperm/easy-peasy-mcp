# Summary of task 23: `/w` whole word modifier

Server version: `0.23.0`.

## What was built

- `/w` with `=`, `has`, `starts`, `ends`, `=~` on `text` (QUERY.md#modifiers): no Unicode letter, digit or `_`
  right before and after the match; also with `/i`.
- `modifier_not_allowed` for `/w` elsewhere (since task 5).
- The `find` description shows `/w`.

## Architecture

`Filter::Content.whole_word`; `find_all` checks the characters around each match and, after a rejected match, goes
on one character after its start with `find_at`, so an overlapping whole-word match is still found.

## Deviations from the plan

None.

## Tests

- examples E23.1–E23.6 (fixtures `words`, `content`)
- unit: line and text borders, `_`, digits, Unicode letters, punctuation, overlapping matches, regexes, `starts`

## Limitations and known gaps

None new.

## Try it

`{ "query": "text has 'id' /w" }`
