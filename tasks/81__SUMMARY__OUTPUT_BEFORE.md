# Summary of task 81: `output.before`

Server version: `0.81.0`.

## What was built

- `output.before` (CONFIG.md#output, OUTPUT.md#options), not set by default (`# before =  (not set)` in
  `get_config`): the context lines before every matched line, like `grep -B`; when it is not set it is `context`.
  From the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `output.before`; invalid values like every whole number.

## Architecture

- `src/config/keys.rs`: `Key::Before` (`Type::OptWhole`).
- `src/engine.rs`: `BlockOptions.before` from `before`, else `context`.

## Deviations from the plan

- The `get_config` examples of tasks 35–80 show the key (separate commit).

## Tests

- examples E81.1–E81.8 (fixture `context` of task 80)
- unit: `request::tests::output`

## Limitations and known gaps

- `after` comes with task 82.

## Try it

`{ "query": "text has 'unwrap()'", "output": { "before": 3 } }`
