# Summary of task 80: `output.context`

Server version: `0.80.0`.

## What was built

- `output.context` (CONFIG.md#output, OUTPUT.md#options, OUTPUT.md#blocks), default `0`: lines before and after every
  matched line, like `grep -C`; from the config file, shown by `get_config` with its source, changed and reset by
  `update_config`, replaced for one request by `output.context`; invalid values like every whole number.
- Text: a context line is `N-text`; blocks whose context touches or overlaps are one, `--` between blocks. JSON: the
  context lines of a block with `n` and `text`. Context lines are not counted.

## Architecture

- `src/output/blocks.rs` (new): `BlockOptions` (`before`, `after`), `Block` and `BlockLine` (the index and whether it
  is matched), `blocks`; the text (`lines::render`, `render_line`) and the JSON renderer use them.
- `src/engine.rs`: the block options from the settings; `src/config/keys.rs`: `Key::Context`.

## Deviations from the plan

- The `get_config` examples of tasks 35–79 show the key (separate commit).

## Tests

- examples E80.1–E80.9 (fixture `context`)
- unit: `output::blocks::tests::context_before_and_after`, `request::tests::output`

## Limitations and known gaps

- `before` and `after` come with tasks 81–82, skipping blank context lines with task 83, dedent with task 84.

## Try it

`{ "query": "text has 'unwrap()'", "output": { "context": 2 } }`
