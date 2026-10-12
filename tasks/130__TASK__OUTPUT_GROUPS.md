# Task 130: Output limited to groups

Status: not implemented

Depends on: task 128

## Goal

An agent can see only the parts of the matches it cares about. In

```
(text has 'serde') @ser and text has 'apple'
```

`apple` is needed to choose the files, but the agent wants to see only where `serde` is.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md), then the code.

- `output.groups` — a list of [group references](128__TASK__QUERY_GROUPS.md#group-references) (`@ser`, `#2`,
  `@def.fname`): in `lines` detail only the lines with matches of these groups are shown, and only these matches are
  marked; the other conditions still decide which files match
- `output.only_match` — `true` prints the matched text of the groups instead of the whole line, one per line with its
  line number, like `grep -o`: `12:serde` (`12:⟦@ser:serde⟧` with markers)
- the counts of the header (`lines`, `matches`) count what is shown; the totals of the files stay
- `max_matches`, `max_matches_per_file`, the budget and the cursor work on the shown matches
- an unknown group: `unknown_group` as in task 128
- both options are output options: in the request, the config file and `update_config`, with `get_config` showing them
- default: not set — the output is as today

## Out of scope

- replacing: [task 129](129__TASK__REPLACE_TOOL.md)

## Spec

- [OUTPUT.md#options](../OUTPUT.md#options)
- [CONFIG.md#output](../CONFIG.md#output)

## Examples

`130__EXAMPLES__OUTPUT_GROUPS.md` covers:

- the example of the goal: only the `serde` lines
- `only_match` with a regex sub-group (`@def.fname`: only the function names)
- a group inside `line( ... )` and `near( ... )`
- limits, budget and cursor on the shown matches
- an unknown group
- the JSON format

## Acceptance criteria

- the spec describes the options before the code is written
- every example in `130__EXAMPLES__OUTPUT_GROUPS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description and `get_config` describe the options
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
