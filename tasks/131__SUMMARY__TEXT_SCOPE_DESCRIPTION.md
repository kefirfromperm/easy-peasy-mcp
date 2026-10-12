# Summary of task 131: the file and the line scope of `text` in the tool description

Server version: `0.123.0`.

## What was built

The `find` description ([TOOLS.md#description](../TOOLS.md#description)) has one more line after the `text` line of
the cheat sheet:

```
  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)
  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it
```

An agent which reads only the description now knows that `text` outside `line()` is the whole file content and that
"a line starting with `## `" is `line(text starts '## ')`. [QUERY.md#gotchas](../QUERY.md#gotchas) has the same pair
next to the gotcha on `text has 'a' and text has 'b'`, with the query which started the task
(`name ~ '*.md' and text starts '## '` finds only Markdown files whose first line is a `## ` heading).

The query language is unchanged: the behavior the line describes was already specified
([QUERY.md#condition-scope](../QUERY.md#condition-scope)) and implemented; the examples pin it.

The spec change came first, in its own commit; E117.2 was fixed to the new text in its own commit.

## Architecture

- `src/description.rs`: the new line in `FIND_DESCRIPTION`, sent in `tools/list`

## Deviations from the plan

None.

## Tests

- E131.1 (startup example, `tests/startup.rs`): `tools/list` sends the new description
- E131.2–E131.4 (`tests/examples.rs`, fixture `text-scope`): `text starts '## '` finds nothing in a Markdown file with
  `## ` headings and shows the funnel, `line(text starts '## ')` finds both headings, `text starts '#!'` finds the
  script
- E117.2 and `tools_are_the_spec_text` in `tests/mcp_stdio.rs` (the description equals the block of TOOLS.md)
- the examples of every earlier task pass

## Limitations

None known. A warning for `starts`, `ends` or `=` in file scope stays out of scope: the funnel of an empty result
shows the condition which removed the files.

## How to try it

`tools/list` shows the new line; on a Markdown file

```json
{ "query": "name ~ '*.md' and line(text starts '## ')" }
```

lists its `## ` headings.
