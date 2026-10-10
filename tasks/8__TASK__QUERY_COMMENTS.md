# Task 8: Comments in the query

Status: implemented, see [8__SUMMARY__QUERY_COMMENTS.md](8__SUMMARY__QUERY_COMMENTS.md)

Depends on: task 6

## Goal

Document long queries: `#` outside of strings starts a comment until the end of the line.

## Scope

- `#` comments outside strings, also after the last condition; `#` inside strings is text
- a query of only comments is the empty query (task 2)
- the legend writes conditions without comments

## Spec

- [QUERY.md#grammar](../QUERY.md#grammar)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`8__EXAMPLES__QUERY_COMMENTS.md` covers:

- a multi-line query with comments, `#` inside a string, a comment-only query

## Acceptance criteria

- every example in `8__EXAMPLES__QUERY_COMMENTS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
