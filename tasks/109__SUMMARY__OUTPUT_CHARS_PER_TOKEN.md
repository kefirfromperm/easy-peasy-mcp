# Summary of task 109: `output.chars_per_token`

Server version: `0.109.0`.

## What was built

- `output.chars_per_token` (CONFIG.md#output, OUTPUT.md#budget), default `4`, a positive number (also `3.5`): from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `output.chars_per_token`; an invalid value is `a positive number is expected` / `must be a positive
  number`.
- The estimate of the budget divides the characters of the response by it.

## Architecture

- `src/config/keys.rs`: `Type::Positive`, `Key::CharsPerToken`; `src/engine.rs` passes it to `output::fit` (the
  constant is gone); `src/description.rs`.
- Every configuration key of the spec is implemented now: the tests which used `output.chars_per_token` as the key
  not implemented yet check its invalid values in the same forms (a section, a dotted key, an inline table,
  `update_config`, the start of the server).

## Deviations from the plan

- The `get_config` examples of tasks 35–108 show the key and CONFIG.md got its errors (separate commits).

## Tests

- examples E109.1–E109.8 (fixture `huge` of task 104)
- unit: `config::keys::tests::positive_numbers`, `request::tests::output_limits`; `tests/mcp_stdio.rs`:
  `an_invalid_value_at_start`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "budget": 1000, "chars_per_token": 3.5 } }`
