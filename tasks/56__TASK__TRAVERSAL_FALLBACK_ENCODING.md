# Task 56: `traversal.fallback_encoding`

Status: implemented, see [56__SUMMARY__TRAVERSAL_FALLBACK_ENCODING.md](56__SUMMARY__TRAVERSAL_FALLBACK_ENCODING.md)

Depends on: task 55

## Goal

Decode legacy files which are not valid UTF-8: `fallback_encoding = "windows-1251"`.

## Scope

- the key `fallback_encoding` in the `[traversal]` section of the config file, default not set
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `traversal.fallback_encoding` replaces it for one request
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- only with `encoding = auto`, only for files which are not valid UTF-8; no `decode_errors` warning for them
- the output shows the encoding used

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

`56__EXAMPLES__TRAVERSAL_FALLBACK_ENCODING.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `56__EXAMPLES__TRAVERSAL_FALLBACK_ENCODING.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
