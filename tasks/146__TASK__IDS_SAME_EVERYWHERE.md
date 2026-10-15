# Task 146: The ids which are the same everywhere are left out

Status: not implemented

Depends on: task 141, task 143

## Goal

The spec 2 rule ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2),
[OUTPUT.md#counted-conditions](../OUTPUT.md#counted-conditions)) leaves out the legend, the conditions of the files
and the ids of the matches only for a query with one counted condition. A query of several conditions joined with
`and` — the most common kind — still writes the same `[#1 #2]` after every file and the same `⟦#2,#3:` in every
match: they can't tell anything apart, every file and every match has them all.

Found while working on task 143: `name ~ '*__TASK__*' and line(text = 'Status: not implemented')` answered

```
#1 name ~ '*__TASK__*'
#2 line(text = 'Status: not implemented')
#3 text = 'Status: not implemented'

tasks/
144__TASK__CONTEXT_BLOCK.md  [#1 #2]  lines: 1  matches: 1
145__TASK__DIR_GROUPS_ONCE.md  [#1 #2]  lines: 1  matches: 1
```

and `name ~ '*.rs' and line(text has 'x')` writes `⟦#2,#3:x⟧` in every match (E143.5).

## Scope

A spec change first (its own commit; OUTPUT.md), then the code.

- the conditions of the files are written only when they differ between the files of the result (all its files, not
  only the page, so the pages of a cursor agree); the ids of the matches only when they differ between the matches;
  the legend only when one of the two is written
- the spec step decides: whether "differ" is over the whole result or the listed files (the whole result is stable
  across pages and `max_files`); how it combines with `output.groups` (the ids follow its references today); whether
  `count`, `facets` (the `condition` facet) and JSON follow the same rule
- the parts stay optional: `show = ["legend"]`, `["conditions"]`, `["match_ids"]` bring each back
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- the counted conditions of task 143; the text of the legend

## Examples

`146__EXAMPLES__IDS_SAME_EVERYWHERE.md` covers:

- `and` of a file condition and a text condition: no legend, no ids; text and JSON
- `or` where the files differ but the matches don't (the conditions of the files written, not the ids of the matches)
- `line(text has 'a' or text has 'b')` where the matches differ (the ids of the matches written)
- a cursor whose first page alone has the same ids everywhere but the result doesn't
- `show` of each part; the same queries with `"version": 1`, unchanged

## Acceptance criteria

- the spec says what "the same everywhere" is before the code is written
- every example in `146__EXAMPLES__IDS_SAME_EVERYWHERE.md` passes as a test; the examples of all earlier tasks still
  pass (a spec 2 example which changes is fixed with the reason in its commit, as the versioning rule allows)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
