# Summary of task 3: `name` with `has`, `starts`, `ends`

Server version: `0.3.0`.

## What was built

- `name has`, `name starts`, `name ends` (QUERY.md#text-operators), in NFC, case-sensitive on Linux and
  case-insensitive on macOS and Windows (`path_case = native`); keywords in any case.
- `unknown_operator` still lists the full set `=, has, starts, ends, ~, ~~, =~`.
- The `find` description lists the new operators.

## Architecture

`TextMatcher` is now one compiled regex for every operator: `=` → `^v$`, `has` → `v`, `starts` → `^v`,
`ends` → `v$` (the value escaped), `~` → the wildcard regex; `(?i)` for case-insensitive paths. The subset check
accepts the three operators after `name`.

## Deviations from the plan

None.

## Tests

- examples E3.1–E3.6, fixture `names` (shared with tasks 4 and 5)
- unit: the operators with regex characters in the value, case-insensitive variants, non-ASCII folding; the subset
  check accepts `has`, `starts`, `ends` and rejects `=~`

## Limitations and known gaps

- the same as task 2: no funnel for empty results, no truncation, order by path

## Try it

`{ "query": "name has 'config'" }`
