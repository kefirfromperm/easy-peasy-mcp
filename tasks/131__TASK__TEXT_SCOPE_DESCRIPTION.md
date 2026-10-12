# Task 131: The file and the line scope of `text` in the tool description

Status: not implemented

Depends on: task 117

## Goal

An agent which reads only the `find` description knows that `text` outside `line()` is the whole file content, so it
writes `line(text starts '## ')` for "a line which starts with `## `" at the first try.

Found by trying the server: `name ~ '*.md' and text starts '## '` found nothing in files full of `## ` headings. The
query was right by the spec ([QUERY.md#text-operators](../QUERY.md#text-operators): in file scope `=`, `has`,
`starts` and `ends` work on the whole file content), but the description says only `text has 'TODO'  content
contains; also: = starts ends`, which reads as grep, line by line. `has` and `=~` give the same files in both scopes,
so the difference shows only with `starts`, `ends`, `=` and `not` — and costs a wasted call and a wrong conclusion
("nothing there").

## Scope

A spec change first (its own commit), then the code:

- [TOOLS.md#description](../TOOLS.md#description): the `text` line of the cheat sheet says that `text` is the whole
  file content and the line content inside `line()`, with one contrasting pair, e.g.
  `text starts '#!'` (the file starts with it) vs `line(text starts '## ')` (a line starts with it); the description
  stays a compact cheat sheet, a line or two more at most
- [QUERY.md#gotchas](../QUERY.md#gotchas): a gotcha with the same pair, next to the one on `text has 'a' and text has
  'b'`
- the `find` description sent in `tools/list` exactly as in TOOLS.md

## Out of scope

- a warning or a hint in the response to `text starts` in file scope (an empty result already has a funnel)
- any change of the query language

## Spec

- [TOOLS.md#description](../TOOLS.md#description)
- [QUERY.md#condition-scope](../QUERY.md#condition-scope), [QUERY.md#text-operators](../QUERY.md#text-operators),
  [QUERY.md#gotchas](../QUERY.md#gotchas)

## Examples

`131__EXAMPLES__TEXT_SCOPE_DESCRIPTION.md` covers:

- `tools/list`: the `find` description with the new lines
- the pair of the description on a fixture: `text starts '## '` without a match in a Markdown file with `## `
  headings, `line(text starts '## ')` listing them

The examples of task 117 and E121 which show the description are fixed to the new text, with the reason in the commit
message (AGENTS.md#2-examples).

## Acceptance criteria

- every example in `131__EXAMPLES__TEXT_SCOPE_DESCRIPTION.md` passes as a test; the examples of all earlier tasks
  still pass
- the description is no longer than today plus two lines
- a feature change of the description: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
