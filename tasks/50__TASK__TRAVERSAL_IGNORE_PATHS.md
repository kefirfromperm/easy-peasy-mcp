# Task 50: `traversal.ignore_paths`

Status: not implemented

Depends on: task 48

## Goal

Personal ignore files outside of the tree, e.g. `~/.config/easy-peasy-mcp/ignore`.

## Scope

- the key `ignore_paths` in the `[traversal]` section of the config file, default `[]`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `traversal.ignore_paths` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- applied as if the file were in the working directory, with the lowest priority

## Spec

- [CONFIG.md#traversal](../CONFIG.md#traversal)
- [CONFIG.md#ignore-files](../CONFIG.md#ignore-files)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`50__EXAMPLES__TRAVERSAL_IGNORE_PATHS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `50__EXAMPLES__TRAVERSAL_IGNORE_PATHS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
