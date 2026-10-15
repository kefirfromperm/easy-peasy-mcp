# Summary of task 151: The indentation of the line in a replace template

Server version: `0.148.0`.

## What was built

- The placeholder `{indent}` of a `replace` template ([TOOLS.md#templates](../TOOLS.md#templates)): the spaces and
  tabs at the start of the line where the match starts. It works with every group, so one template inserts a line
  under matched lines of any indentation: `{"@item": "{@item}\n{indent}\"more_lines\","}`.
- It is the whole indentation of the line: also for a match in the middle of the line, and for a match which takes
  the indentation itself (`^\s*` in a regex), which can write it back. A multiline match gets the indentation of its
  first line; a line which starts with another character has an empty one.
- A line break of a template is not followed by the indentation on its own: a new line without `{indent}` starts at
  the first column.
- `/e` doesn't escape it (it is text of the file); a `/n` match leaves it out, since the naming style of the match has
  no spaces or tabs (otherwise tabs would end up inside the first word).
- `invalid_template`: the hint of `unknown placeholder` and `empty placeholder` lists `{indent}` after the group's
  placeholders; the hint of `'{@x}' is not '@old' or its sub-group` stays the list of the group (E129.12 unchanged).
- The description of `replace` names `{indent}` with an example.
- Every spec version: a new placeholder is not a breaking change.

## Architecture

`src/replace.rs`:

- `Part::Indent`, a new part of a `Template`; `Template::parse` maps the name `indent` to it. The hint closure takes
  whether to list `{indent}`.
- `Template::expand` takes the indentation as an argument.
- `indentation(text, start)`: from the start of the line of `start` (after the last `\n` before it, in the normalized
  text the fragments are in) the run of `' '` and `'\t'`.
- `replacements` computes it once per match, or passes `""` for a match restyled by `/n`.

`src/description.rs`: one line in `REPLACE_DESCRIPTION`, the same as in TOOLS.md#replace.

## Deviations from the plan

- E151.10 (`/n` leaves the indentation out) was added with the tests: building a `/n` filter in a unit test needs the
  whole query parser, a startup example is simpler and checks the real behavior.

## Tests

- examples E151.1–E151.10 in [151__EXAMPLES__TEMPLATE_INDENT.md](151__EXAMPLES__TEMPLATE_INDENT.md) (startup
  examples, `tests/startup.rs`), fixture `tests/fixtures/template-indent`: a line inserted in files of two spaces, four
  spaces and tabs (dry run, real call and `find` after it), a match in the middle of a line, a match which takes the
  indentation, a line break without `{indent}`, a multiline match, the hints of `invalid_template`, `/n`
- unit tests in `replace.rs`: `the_indentation_of_the_line_of_a_match` (spaces, tabs, mixed, inside the
  indentation, at a line break, the first and the last line), `templates_and_their_errors` (`{indent}` expanded, `{{indent}}`
  literal, the hints with and without it), `literal_text_is_escaped_like_the_match` (the indentation is not escaped)
- all earlier examples pass unchanged

## Limitations

- No re-indentation of a block: the lines of a multiline match keep their own indentation, only new lines written by
  the template get `{indent}` (out of scope of the task).
- `rename` and `copy` templates don't have `{indent}`; there it is an unknown placeholder of their own table.

## How to try it

```json
{"query": "text has '\"left_out\",' @item", "replace": {"@item": "{@item}\n{indent}\"more_lines\","}, "dry_run": true}
```

with the `replace` tool (`--allow-write`) shows `"more_lines",` under every `"left_out",`, indented like it.
