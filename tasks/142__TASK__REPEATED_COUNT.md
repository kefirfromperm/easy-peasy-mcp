# Task 142: The number of files of a repeated line

Status: not implemented

Depends on: task 86, task 138

## Goal

The `×N` of a [repeated line](../OUTPUT.md#repeated-lines) tells the agent in how many files of the result the line
is, so it can trust it when it edits them all. Today the repeated lines are found among the listed files only: with
the default `max_files = 100` a line in 137 files is written as `×100`, and nothing says that 37 files are missing
from the count and from the locations:

```
files: 137  lines: 137  matches: 137  result: r5  truncated: yes (max_files, max_output)  next cursor: …
repeated:
  ×100 ⟦#1,#2:Spec: `1`⟧    tasks/121__EXAMPLES__SPEC_CONFORMANCE.md:3, …, … 90 more
```

Found by trying the server: `line(text = 'Spec: `1`')` with `repeated_locations = 0` over the 137 examples files
which have the line answered `×100`; `grep -lx` lists 137. The header has the real totals, but `×N` reads like one of
them.

## Scope

A spec change first (its own commit; OUTPUT.md), then the code. The spec step decides between:

- `×N` counts every file of the result which has the line, also the files after `max_files` and before the page; the
  locations stay the ones of the listed files, and `… M more` counts all the others; the cost: the repeated lines are
  grouped over all the matched files, not only the listed ones
- or `×N` stays the number of the listed files and the line says so when files are cut, e.g. `×100 of 137+`, or the
  spec says it in OUTPUT.md#repeated-lines

Either way:

- JSON `count` follows the text
- the pages of a cursor: a repeated line on the next page is counted the same way, so the agent doesn't add up the
  pages twice; the spec step says how
- `max_output`, `budget` and the repeated locations of task 138 keep working; any new piece of the line has a part of
  [`omit`](../OUTPUT.md#omit) (**every part of the output is optional**)
- a breaking change of the text format? Decided in the spec step by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- grouping lines which differ
- the repeated lines in other details than `lines`

## Spec

- [OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines), [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation),
  [OUTPUT.md#pagination](../OUTPUT.md#pagination)

## Examples

`142__EXAMPLES__REPEATED_COUNT.md` covers:

- a line repeated in more files than `max_files`, with the default and with `repeated_locations = 0`, text and JSON
- the next page of its cursor
- a line repeated only in the files after `max_files`
- `max_output` and the budget with such a line

## Acceptance criteria

- the spec says what `×N` and `count` count before the code is written
- every example in `142__EXAMPLES__REPEATED_COUNT.md` passes as a test; the examples of all earlier tasks still pass
  (or are changed by a decided breaking change, each change named in the summary)
- the server version gets a minor bump (a patch bump if only the spec text changes)
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
