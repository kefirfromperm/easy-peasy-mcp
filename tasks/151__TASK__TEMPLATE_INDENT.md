# Task 151: The indentation of the line in a replace template

Status: implemented, see [151__SUMMARY__TEMPLATE_INDENT.md](151__SUMMARY__TEMPLATE_INDENT.md)

Depends on: task 129

## Goal

A [template of `replace`](../TOOLS.md#templates) can't refer to the indentation of the matched line. An agent which
inserts a line (a new item in an indented list, a field in pretty-printed JSON) types the indentation by hand, so one
replace can't serve lines with different indentation; a regex which takes `^\s*` into the match loses the
indentation unless it is captured and written back.

Found in the experiment of task 144: adding `"more_lines",` after `"left_out",` in pretty-printed JSON examples of
several files, and a dry run where `^\s*` swallowed the indentation.

## Scope

A spec change first (its own commit; TOOLS.md#templates), then the code.

- a placeholder for the leading whitespace of the line where the match starts, e.g. `{indent}`, usable with every
  group
- the spec step decides: its name; whether a line break in a template is followed by that indentation on its own
  (likely not: explicit is safer); the indentation of a multiline match; the errors (`invalid_template` hints list
  the new placeholder)

## Out of scope

- changing the indentation of a block (re-indent); templates of `rename` and `copy`

## Examples

`151__EXAMPLES__TEMPLATE_INDENT.md` covers:

- inserting a line after a matched line in files of different indentation (spaces, tabs)
- a match at the start of a line, in the middle of a line
- a multiline match; the dry run of each
- the hint of `invalid_template` with the new placeholder

## Acceptance criteria

- the spec says what the placeholder writes before the code is written
- every example in `151__EXAMPLES__TEMPLATE_INDENT.md` passes as a test; the examples of all earlier tasks still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
