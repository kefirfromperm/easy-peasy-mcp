# Task 44: `traversal.min_depth`

Status: in progress

Depends on: task 36

## Goal

Skip entries close to the working directory: `min_depth = 2`.

## Scope

- the key `min_depth` in the `[traversal]` section of the config file, default `0`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `traversal.min_depth` replaces it for one request
- the request object `traversal` is added to the `find` input schema by this task, with this key only; the other keys of the spec are added by their tasks
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the direct children of the working directory are depth 1

## Spec

- [CONFIG.md#traversal](../CONFIG.md#traversal)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`44__EXAMPLES__TRAVERSAL_MIN_DEPTH.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `44__EXAMPLES__TRAVERSAL_MIN_DEPTH.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
