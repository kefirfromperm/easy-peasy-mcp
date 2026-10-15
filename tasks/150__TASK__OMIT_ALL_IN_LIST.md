# Task 150: The error of "all" in a list of parts says what to write

Status: not implemented

Depends on: task 134

## Goal

`output.omit = ["all"]` (and `output.show = ["all"]`) is an error whose message lists the parts but doesn't say that
`"all"` is valid only as a bare string:

```
error[invalid_option]: 'output.omit' must be "all" or a list of parts (counts, result, dir, legend, funnel, hints, conditions, file_counts, match_ids, groups, line_numbers, line_counts, gaps, indent, locations, warnings, left_out)
```

Found in the experiment of task 144: the agent wrote `omit: ["all"]` and had to guess why it failed.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md where the errors of `omit` are listed), then the code.

- the spec step decides one of: `["all"]` is accepted as `"all"` (the list form of the same value); or the error
  names the item and gives a hint `write "all" without the brackets`
- the same for `show`, the config file and `update_config`

## Out of scope

- the other errors of `omit` and `show`

## Examples

`150__EXAMPLES__OMIT_ALL_IN_LIST.md` covers:

- `omit = ["all"]`, `show = ["all"]`, `["all", "legend"]`
- the same in the config file and `update_config`
- `"version": 1`

## Acceptance criteria

- the spec says what `["all"]` does before the code is written
- every example in `150__EXAMPLES__OMIT_ALL_IN_LIST.md` passes as a test; the examples of all earlier tasks still pass
- the server version gets a patch bump, or a minor one if `["all"]` becomes valid
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
