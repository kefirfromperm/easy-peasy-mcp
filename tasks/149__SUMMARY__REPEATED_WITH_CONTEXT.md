# Summary of task 149: A repeated line keeps the context which was asked for

Server version: `0.146.0`.

## What was built

- In spec 2 a matched line with context lines (`context`, `before`, `after`, also `after = "block"`) is grouped in
  the `repeated` section only when its whole block is the same in the files
  ([OUTPUT.md#repeated-lines-with-context](../OUTPUT.md#repeated-lines-with-context)): the line with the context lines
  its file would write for it alone, without the common leading whitespace of the block and the trailing whitespace of
  every line, the blank lines only with `blank_context`. An anchor line (a fence, a heading, `#[test]`) whose context
  differs is written under every file with its context, as the agent asked.
- A block which is the same is written once: `  ×N <locations>` and its lines after four spaces, the matched line
  after `:`, the context lines after `-`; with `columns` the columns of the line after six spaces. JSON has
  `before` and `after`, the texts of the context lines, each only when it isn't empty.
- A line with another matched line in its block, or with a fragment in the common indentation, is not grouped in that
  file; that file is not counted in `N`.
- A block of only the matched line (no context at that place, `after = "block"` on a line which opens no block) and
  `context = 0` group the lines as before; spec 1 (`"version": 1`) keeps dropping the context of a repeated line.
- The lines of a block are like the repeated line: not cut, not counted in the limits; the counts don't change.

## Architecture

- `output/blocks.rs`: `context(content, m, options)` gives the lines of the context of one matched line and is used
  by `blocks` too; `BlockOptions::has_context` replaces the check in `engine.rs`; `indent` is shared.
- `output/repeated.rs`: `repeated(files, listed, min, context)`; `block` makes the key of a line (the context lines
  before, the text, the fragments, the context lines after), `trimmed` takes the indentation of the block.
  `RepeatedLine` has `before` and `after`.
- `output/mod.rs`: `Response::repeated` passes the block options in spec 2; `push_repeated` writes the block.
- `output/json.rs`: `before` and `after` of `Repeated`.

## Deviations from the plan

- The spec commit was amended before the examples to say where the columns line of a block goes (six spaces), found
  when the first run showed the columns are 1-based.
- E149.9 was fixed: a line with fragments has no `"match": true` (only `highlight = none` writes it).
- No earlier example changed: the spec 2 examples with repeated lines have no context.

## Tests

- unit, `output/repeated.rs`: the same block grouped with its context, a different context line, another matched
  line in the block, a fragment in the common indentation, blank context lines, a block of only the line, spec 1
- unit, `output/mod.rs`: `a_repeated_line_with_its_context` — the text with and without `locations`, spec 1
- examples: E149.1–E149.9 (fixture `repeated-context`)

## Limitations

- A long `after = "block"` body which is the same in many files is written once and not cut, like a repeated line.

## How to try it

```json
{ "query": "line(text starts '```text tree')", "output": { "context": 1 } }
```

in the `tasks/` directory lists every fence with the line after it, which differs per file, instead of one
`×N` line without context.
