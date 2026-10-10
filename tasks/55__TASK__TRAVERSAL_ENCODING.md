# Task 55: `traversal.encoding`

Status: not implemented

Depends on: task 25, task 44

## Goal

Force an encoding for files without a BOM: `encoding = "windows-1252"`.

## Scope

- the key `encoding` in the `[traversal]` section of the config file, default `"auto"`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `traversal.encoding` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- WHATWG encoding labels
- files with a BOM are still decoded by the BOM

## Spec

- [CONFIG.md#traversal](../CONFIG.md#traversal)
- [CONFIG.md#encoding](../CONFIG.md#encoding)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`55__EXAMPLES__TRAVERSAL_ENCODING.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `55__EXAMPLES__TRAVERSAL_ENCODING.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
