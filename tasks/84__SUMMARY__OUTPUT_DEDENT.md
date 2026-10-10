# Summary of task 84: `output.dedent`

Server version: `0.84.0`.

## What was built

- `output.dedent` (CONFIG.md#output, OUTPUT.md#dedent), default `true`: from the config file, shown by `get_config`
  with its source, changed and reset by `update_config`, replaced for one request by `output.dedent`; invalid values
  like every boolean key.
- The common leading spaces and tabs of every block are removed: of its lines which are not empty or whitespace-only,
  never into a match — a fragment which starts in the indentation keeps the whitespace before it (spec clarified in
  its own commit). Text: `⇥N` before the block, no `⇥` line when nothing is removed; JSON: `indent` of the block.
  `col` of a fragment refers to the original line.

## Architecture

- `src/output/blocks.rs`: `BlockOptions.dedent`, `Block.indent`, `indent`.
- `src/output/lines.rs`: `⇥N`, the lines from their indent; `src/output/json.rs`: `indent` and the dedented `text`.
- `src/config/keys.rs`: `Key::Dedent`; `src/engine.rs`: the block options.

## Deviations from the plan

- The `get_config` examples of tasks 35–83 show the key (separate commit).
- E30.1 and E30.2 were fixed in their own commit: their block of indented lines has `⇥4`.

## Tests

- examples E84.1–E84.10 (fixture `dedent`), E30.1, E30.2
- unit: `output::blocks::tests::context_before_and_after` (the common prefix, blank lines, a tab, a fragment in the
  indentation), `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'unwrap()'", "output": { "context": 2, "dedent": false } }`
