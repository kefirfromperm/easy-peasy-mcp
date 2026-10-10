# Task 46: `traversal.follow_symlinks`

Status: implemented, see [46__SUMMARY__TRAVERSAL_FOLLOW_SYMLINKS.md](46__SUMMARY__TRAVERSAL_FOLLOW_SYMLINKS.md)

Depends on: task 44, task 9

## Goal

Descend into symlinked directories and read symlinked files.

## Scope

- the key `follow_symlinks` in the `[traversal]` section of the config file, default `false`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `traversal.follow_symlinks` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- a symlink to a file is then both `file` and `link`, its target is read
- loops are detected and skipped
- a symlink pointing outside the allowed roots is skipped with a `permission_denied` warning
- duplicates through symlinks are returned once

## Spec

- [CONFIG.md#traversal](../CONFIG.md#traversal)
- [QUERY.md#files-and-directories](../QUERY.md#files-and-directories)
- [CONFIG.md#security](../CONFIG.md#security)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`46__EXAMPLES__TRAVERSAL_FOLLOW_SYMLINKS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `46__EXAMPLES__TRAVERSAL_FOLLOW_SYMLINKS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
