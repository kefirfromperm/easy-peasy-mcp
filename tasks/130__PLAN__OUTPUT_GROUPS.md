# Plan for task 130: Output limited to groups

## Design

The shown matches are a filter over the fragments of the found files, applied by `find` after the search and before
the response, so the counts, the limits, the budget, the cursor, the repeated lines and the result id all see only
the shown matches without changes of their own. `only_match` is a way of writing the lines of `lines` detail.

### Modules

- `config::keys`: the keys `Groups` (a new `Type::GroupRefs`: a list of strings, `a list of group references`) and
  `OnlyMatch` (`Type::Bool`) after `PathFragments`, in the order of CONFIG.md; defaults `[]` and `false`
- `query::ast`: `CondId::Sub(String)`, the id of a sub-group of a regex (`@def.fname`, `#3.1`) as it is written, so a
  match of a sub-group carries its reference like any other id; `GroupRef::reference()` writes the canonical
  reference (`@def.01` → `@def.1`)
- `shown` (new, `src/shown.rs`): `Shown::of(query, list)` checks the references with `Query::group` (the error
  `unknown_group` as `into_field_report("output.groups", reference)`), `None` for an empty list; `Shown::apply(files)`
  replaces the fragments of every file by the shown ones:
  - a fragment whose ids have a whole group of the list: the fragment with only those ids, its sub-groups kept
  - a sub-group of a fragment with a reference of the list: a fragment of its range with the ids
    `(order of the condition, CondId::Sub(reference))`
  - `merge_identical`, `matched_lines`; a file without a shown match gets `matches: None`, so it is listed without
    counts and lines; the path fragments are filtered the same way
- `engine`:
  - `try_find`: `Shown::of` before the search (an error costs no search), `Shown::apply` after it; the check of the
    totals of a cursor (`results_changed`) moves from `run` to after the filter, because the totals of a cursor are
    the shown ones. `run` keeps returning the full matches for the write tools
  - `only_match`: `block_options` without context and dedent, `group_repeated = 0`, `Response::only_match`
  - `omit(...).effective(...)` gets the number of references: with `groups` `match_ids` is left out for at most one
    reference instead of at most one condition of the legend
- `output::blocks::BlockOptions::only_match`: `listing::shown_lines` counts the characters of the matches in their
  lines instead of the lines (`max_output_per_file`, `max_output`)
- `output::lines::render_matches`: every listed fragment on its own lines, `N:` before every line of it, in the
  highlight style: `markers` and `auto` (unless the markers outgrow the match or it has no ids) wrap it with its ids,
  `columns` adds the columns line of the fragment
- `output::json`: `File::fragments` with `only_match` instead of `blocks`: `{ n, col, text, ids, escape, to, groups }`
  for `columns`, `{ n, text }` for `none` and `markers` (the text with the markers)
- `description.rs` and the schema: the two lines of the `find` description and the two options of the schema

### Decisions

- the filter works on `Found`, not on the response: every part of the output (text, JSON, limits, budget, cursor,
  repeated lines, result id) then follows by construction; rejected: a filter in the listing, which would need the
  counts, the budget and the cursor to be changed one by one
- the ids of a sub-group match are a `CondId` variant rather than a parallel type: the markers, the columns, JSON and
  `omit match_ids` handle them unchanged
- the filter is in `try_find`, not in `run`: `replace` needs all the fragments of its query, also with a configured
  `output.groups`

## Tests

- examples E130.1–E130.13 (`tasks/130__EXAMPLES__OUTPUT_GROUPS.md`)
- the earlier examples changed by the spec: `get_config` with the two keys (the `[output]` sections), the `find`
  description (E117, E131) and the schema (E117)
- unit tests:
  - `shown::tests`: whole groups, sub-groups, a file without a shown match, the path fragments, the references
    checked in order
  - `omit::tests`: the rule of `match_ids` with references
  - `lines::tests`: `render_matches` in every highlight style, a multiline match, without ids and line numbers
  - `listing::tests`: `max_output_per_file` counts the characters of the matches with `only_match`
  - `config::keys` tests: the type `GroupRefs`

## Steps

1. red: the unit tests and the examples
2. the keys and their types; `CondId::Sub`; `Shown` with its errors and the filter in `try_find`; the cursor check
3. the omit rule; `only_match` in text, then JSON
4. the description, the schema; the earlier examples of `get_config`, the description and the schema
5. summary, status, version `0.137.0`

## Risks

- a configured `output.groups` makes every query without the group fail; this is what the spec says, and the
  request list replaces it
- `CondId::Sub` must never reach the legend or the conditions of a file: only `Shown::apply` makes it
