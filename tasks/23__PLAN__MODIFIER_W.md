# Plan for task 23: `/w` whole word modifier

## Design

- `query/eval.rs`: `Filter::Content.whole_word`; `content_matches` keeps only matches without a word character —
  a Unicode letter, digit or `_` — right before and after them; after a rejected match the search goes on one
  character after its start, so an overlapping whole-word match isn't missed (`find_at`).
- `query/mod.rs`: `/w` sets `whole_word` for `text` with `=`, `has`, `starts`, `ends`, `=~`; `modifier_not_allowed`
  elsewhere exists since task 5.
- `query/subset.rs`: `/w` is supported.
- `description.rs`: `/w whole word`.

Rejected: `\b` in the regex — wrong for values which start or end with punctuation (`'.unwrap('`).

## Tests

- examples E23.1–E23.6 (fixtures `words`, `content`)
- unit: whole-word filtering at the text start and end, next to `_`, digits, Unicode letters, punctuation; an
  overlapping later match after a rejected one

## Steps

1. failing unit tests
2. filtering, compile, subset, description

## Risks

None known.
