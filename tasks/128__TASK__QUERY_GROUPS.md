# Task 128: Query groups

Status: not implemented

Depends on: task 7, task 22

## Goal

An agent can name the parts of a query whose matched text it wants to use later: to show only them
([task 130](130__TASK__OUTPUT_GROUPS.md)) or to replace them ([task 129](129__TASK__REPLACE_TOOL.md)). Like the groups
of a regex, but over the whole query:

```
(text has 'serde' or text has 'serde_json') @ser and text has 'apple'
```

finds the files with `serde` (or `serde_json`) and `apple`; the group `@ser` is the matched `serde` / `serde_json`
text, not `apple`.

## Scope

A spec change first (its own commit; QUERY.md, OUTPUT.md), then the code. The groups build on
[labels](../QUERY.md#labels-and-condition-ids), they don't add a second naming syntax.

### A group is a label

- a labeled condition is a group already: `text has 'serde' @ser` — its matches are the group
- new: a label after a bracketed expression, `( ... ) @name`: the group is every match of the `text` conditions inside
  the brackets (also inside nested groups and `line( ... )` / `near( ... )`)
- a bracket label is an id like any other: it is listed in the legend, counted by the `condition` facet, shown in the
  match markers next to the ids of the inner conditions (`⟦#1,@ser:serde⟧`)
- a bracket group without a `text` condition inside is valid as a condition id; it has no matched text
- the labels stay unique across conditions and groups

### Regex groups

- the capture groups of a `=~` condition are sub-groups of its id: `@def.1`, `@def.2`, ... and `@def.name` for a
  named group `(?<name>...)`; without a label `#3.1`, `#3.name`
- `text =~ `fn\s+(?<fname>\w+)\(` @def` — `@def` is the whole match, `@def.fname` the function name
- a group which didn't take part in a match has no text for that match
- the output of existing queries doesn't change: capture groups are not marked in the text; the JSON format may add
  them to a match (a new JSON field is not breaking, [TOOLS.md#versioning](../TOOLS.md#versioning))

### Group references

The syntax used by the later tasks to refer to groups, defined here once:

- `@name`, `#3` — a whole group; `@name.1`, `@name.fname` — a regex sub-group
- a reference to an id which doesn't exist: `unknown_group` with the position and the ids of the query as the hint

### Errors

- a label after `)` of a bracket which is not a group of the query (e.g. inside a value) is a syntax error as today
- a duplicate label: `duplicate_label` as today, also between a group and a condition
- `@x.y` where `@x` is not a regex condition or has no group `y`: `unknown_group`

## Out of scope

- limiting the output to groups: [task 130](130__TASK__OUTPUT_GROUPS.md)
- replacing: [task 129](129__TASK__REPLACE_TOOL.md)
- groups on metadata conditions (`name`, `path`): they have no matched text in the content; may come later

## Spec

- [QUERY.md#labels-and-condition-ids](../QUERY.md#labels-and-condition-ids)
- [QUERY.md#grammar](../QUERY.md#grammar)
- [QUERY.md#errors](../QUERY.md#errors)
- [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids)

## Examples

`128__EXAMPLES__QUERY_GROUPS.md` covers:

- the example of the goal: legend, markers, `condition` facet
- nested groups; a group inside `line( ... )`; a group around `or` branches
- a group without a `text` condition
- regex groups, numbered and named, in the JSON format
- duplicate labels, an unknown group reference
- an existing query with a regex with groups: the output is unchanged

## Acceptance criteria

- the spec describes groups before the code is written; the grammar is updated
- every example in `128__EXAMPLES__QUERY_GROUPS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description mentions bracket labels
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
