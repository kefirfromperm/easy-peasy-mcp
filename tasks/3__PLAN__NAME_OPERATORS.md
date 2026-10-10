# Plan for task 3: `name` with `has`, `starts`, `ends`

## Design

- `query/subset.rs`: `has`, `starts`, `ends` join `=` and `~` as the operators of `name`.
- `query/mod.rs`: `TextMatcher` compiles every text operator to one anchored or unanchored regex over the NFC value:
  `=` → `^v$`, `has` → `v`, `starts` → `^v`, `ends` → `v$`, with `(?i)` when `path_case` is insensitive. The regex
  crate gives Unicode simple case folding, which `/i` (task 5) needs too. `~` stays the wildcard regex.
- the parser already knows the operators and lists the full set in `unknown_operator`.
- `description.rs`: the cheat sheet gets `has`, `starts`, `ends`.

Rejected: plain `str::contains` / `starts_with` for the case-sensitive path — faster, but two code paths for the
same thing; the regex literal optimizations make the difference negligible for file names.

## Tests

- examples E3.1–E3.6
- unit: compile `has`, `starts`, `ends`, `=` with special regex characters in the value, case-insensitive variants;
  subset check accepts the three operators and still rejects `~~` and `=~`

## Steps

1. failing unit tests
2. subset and matcher
3. description

## Risks

None known.
