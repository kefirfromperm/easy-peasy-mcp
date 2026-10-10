# Task 20: Binary files and unknown content

Status: implemented, see [20__SUMMARY__UNKNOWN_CONTENT.md](20__SUMMARY__UNKNOWN_CONTENT.md)

Depends on: task 19, task 13

## Goal

Binary files are not searched by content and never returned because of their content, but still found by other conditions: `name ~ '*.png' or text has 'logo'`.

## Scope

- a file is binary if its first 8K contain a zero byte (BOM and UTF-16 checks come with task 25)
- `binary = skip` (the default): content conditions are unknown for binary files, three-valued logic of task 13; a `binary_skipped` warning
- the warning format: grouped by code with a count and a few example paths, in text and JSON

## Spec

- [QUERY.md#files-whose-content-is-not-searched](../QUERY.md#files-whose-content-is-not-searched)
- [CONFIG.md#traversal](../CONFIG.md#traversal)
- [OUTPUT.md#warnings](../OUTPUT.md#warnings)

## Out of scope

- `traversal.binary` as an option — task 53
- `max_file_size` — task 54

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`20__EXAMPLES__UNKNOWN_CONTENT.md` covers:

- `text has 'x'` and `not text has 'x'` don't return a binary file; `name ~ '*.bin' or text has 'x'` returns it by the name
- the `binary_skipped` warning

## Acceptance criteria

- every example in `20__EXAMPLES__UNKNOWN_CONTENT.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
