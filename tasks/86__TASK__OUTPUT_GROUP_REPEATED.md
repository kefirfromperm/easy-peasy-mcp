# Task 86: `output.group_repeated`

Status: implemented, see [86__SUMMARY__OUTPUT_GROUP_REPEATED.md](86__SUMMARY__OUTPUT_GROUP_REPEATED.md)

Depends on: task 80

## Goal

A matched line repeated in many files is printed once.

## Scope

- the key `group_repeated` in the `[output]` section of the config file, default `3`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.group_repeated` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- identical ignoring leading and trailing whitespace and with the same condition ids, in at least N files
- the `repeated:` section before the files: `×37 line    locations, … N more`
- repeated lines are not printed in the files; a file with no other lines is left out in `lines` detail, still listed in `files` detail; counts unchanged
- `0` — off
- `repeated` in JSON

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`86__EXAMPLES__OUTPUT_GROUP_REPEATED.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `86__EXAMPLES__OUTPUT_GROUP_REPEATED.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
