# Summary of task 82: `output.after`

Server version: `0.82.0`.

## What was built

- `output.after` (CONFIG.md#output, OUTPUT.md#options), not set by default (`# after =  (not set)` in
  `get_config`): the context lines after every matched line, like `grep -A`; when it is not set it is `context`.
  From the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `output.after`; invalid values like every whole number. The context ends at the end of the file.

## Architecture

- `src/config/keys.rs`: `Key::After` (`Type::OptWhole`).
- `src/engine.rs`: `BlockOptions.after` from `after`, else `context`.

## Deviations from the plan

- The `get_config` examples of tasks 35–81 show the key (separate commit).

## Tests

- examples E82.1–E82.8 (fixture `context` of task 80)
- unit: `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'unwrap()'", "output": { "after": 3 } }`
