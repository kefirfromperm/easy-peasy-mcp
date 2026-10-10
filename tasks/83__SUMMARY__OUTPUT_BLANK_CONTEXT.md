# Summary of task 83: `output.blank_context`

Server version: `0.83.0`.

## What was built

- `output.blank_context` (CONFIG.md#output, OUTPUT.md#blank-context-lines), default `false`: from the config file,
  shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.blank_context`; invalid values like every boolean key.
- `false`: empty and whitespace-only context lines are skipped (in text and JSON); they still join blocks, so the
  line numbers show the gap. `true`: they are kept. Matched lines are always shown.

## Architecture

- `src/output/blocks.rs`: `BlockOptions.blank_context`, the filter after the blocks are made.
- `src/config/keys.rs`: `Key::BlankContext`; `src/engine.rs`: the block options.

## Deviations from the plan

- The `get_config` examples of tasks 35–82 show the key (separate commit).

## Tests

- examples E83.1–E83.8 (fixture `blank`)
- unit: `output::blocks::tests::context_before_and_after`, `request::tests::output`

## Limitations and known gaps

- Dedent comes with task 84.

## Try it

`{ "query": "text has 'unwrap()'", "output": { "context": 3, "blank_context": true } }`
