# Task 30: `/e` match escaped text

Status: not implemented

Depends on: task 23

## Goal

Find text stored escaped: ``text has `"type": "admin"` /e(backslash, any)``.

## Scope

- `/e(style, levels)` on `=`, `has`, `starts`, `ends` on `text`
- styles `backslash` (JSON also without `\'`), `quote_doubling`, `xml` (`&apos;` and `&#39;`), `url` (`%20` and `+`)
- levels: a number, a range `a..b`, `any` = 0 to `escape.max_level` (default 3)
- combines with `/i`, `/w`, `/c`, not with `/n`
- the escape level of each fragment: `escape` in JSON, `/eN` after the ids in markers
- `invalid_escape_level` above the max level

## Spec

- [QUERY.md#escaping](../QUERY.md#escaping)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)
- [OUTPUT.md#highlight](../OUTPUT.md#highlight)
- [CONFIG.md#escape](../CONFIG.md#escape)

## Out of scope

- `escape.max_level` configuration — task 61

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`30__EXAMPLES__MODIFIER_E.md` covers:

- each spec example for every style and level
- a level above 3

## Acceptance criteria

- every example in `30__EXAMPLES__MODIFIER_E.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
