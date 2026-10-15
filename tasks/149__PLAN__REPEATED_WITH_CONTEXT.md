# Plan for task 149: A repeated line keeps the context which was asked for

## Design

The spec step ([OUTPUT.md#repeated-lines-with-context](../OUTPUT.md#repeated-lines-with-context)) groups a matched
line with context lines in spec 2 only when its whole block is the same. The grouping stays in `output/repeated.rs`;
the listing, the limits and the counts don't change, because a grouped line still only removes its fragments from its
file.

- `output/blocks.rs`: `context(content, m, options) -> (lo, hi)`, the lines of the context of one matched line
  (`before`, `after` lines or `after = "block"`), taken out of `blocks`, which uses it; `indent` becomes
  `pub(super)` to count the common leading whitespace of a block
- `output/repeated.rs`: `repeated(files, listed, min, context: Option<&BlockOptions>)`; with a context the key of a
  line is its block: the context lines (the blank ones only with `blank_context`) without the common leading
  whitespace and the trailing whitespace, the matched line likewise with its fragments. A block with another matched
  line of the file, or with a fragment in the common whitespace, gives no key: the line stays in its file.
  `RepeatedLine` gets `before` and `after`, the texts of the context lines; a block of only the matched line is the
  line of spec 1 (trimmed)
- `output/mod.rs`: `Response::repeated` passes `Some(&self.blocks)` in spec 2 when there is context (`before > 0`,
  `after` above 0 or `Block`), `None` in spec 1; `push_repeated` writes a line with `before` or `after` as
  `  ×N <locations>` and its lines after four spaces with `-` / `:`, the columns after six
- `output/json.rs`: `before` and `after` of `Repeated`, skipped when empty, in the order of the spec

Rejected: grouping the line and dropping its context when the context differs (what spec 1 does), and writing the
context of every file in the section (as many tokens as `group_repeated = 0`).

## Tests

- unit, `repeated.rs`: the same block in three files is grouped with its `before`/`after`; a different context line
  keeps the line in its file; a block indented differently is the same; another matched line in the block; a fragment
  in the common whitespace; `None` (spec 1) groups by the line as before
- unit, `mod.rs`: the text of a block in the section, with and without `locations`
- examples: E149.1–E149.9; the spec 2 examples of earlier tasks with context and repeated lines may change (fixed
  with the reason in their commit)

## Steps

1. red: the unit tests and the examples
2. `blocks::context`, `repeated` with the block key
3. the text and JSON format
4. the earlier examples which change, clippy, fmt, the summary

## Risks

- `after = "block"` blocks can be long; they are written once in the section, not cut, like the repeated line
