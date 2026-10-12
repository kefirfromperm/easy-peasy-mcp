# Plan for task 131: the file and the line scope of `text` in the tool description

## Design

The spec change is committed already: one line after the `text` line of the cheat sheet in
[TOOLS.md#description](../TOOLS.md#description) and a gotcha in [QUERY.md#gotchas](../QUERY.md#gotchas). The code
change is the same line in `FIND_DESCRIPTION` of `src/description.rs`, which `tools/list` sends.

```
  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)
  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it
```

The new line keeps the layout of the cheat sheet (an example in the left column, what it means on the right), so
the description grows by one line. No query, search or output code changes: E131.2–E131.4 already pass and pin the
behavior the line describes.

## Rejected

- rewording the `text` line only (`file content contains`): it says "file" but not what `starts` does, and the
  contrast with `line()` is what the agent missed
- a warning for `text starts` in file scope: out of scope, the funnel of an empty result already shows the condition
- a new entry under EXAMPLES of the description: E121 turns each of them into an example; the pair belongs next to
  `text` in QUERY, where the agent reads the operators

## Tests

- E131.1 (`tests/startup.rs`, startup example): `tools/list` sends the new description
- E117.2, fixed to the new text in its own commit
- `tests/mcp_stdio.rs`: the description equals the block of TOOLS.md#description
- E131.2–E131.4 (`tests/examples.rs`): the pair on the fixture `text-scope`
- the examples of every earlier task still pass

## Steps

1. red: E131.1 and E117.2 fail against the old description (committed with the examples and the E117.2 fix)
2. the line in `src/description.rs`
3. summary, status `implemented`, minor version bump `0.122.1` → `0.123.0` (a feature change of the description)

## Risks

- none known: the description has no length limit in MCP; the task allows two more lines, this adds one
