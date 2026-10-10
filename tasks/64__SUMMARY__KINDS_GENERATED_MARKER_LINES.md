# Summary of task 64: `kinds.generated_marker_lines`

Server version: `0.64.0`.

## What was built

- `kinds.generated_marker_lines` (CONFIG.md#kinds), default `5`: how many first lines are checked for the markers of
  `generated`; from the config file, shown by `get_config` with its source, changed and reset by `update_config`,
  replaced for one request by `kinds.generated_marker_lines`.
- A whole number of at least 1 (spec added in its own commit for every key with a minimum): `invalid value for
  'kinds.generated_marker_lines': a whole number of at least 1 is expected`, in a request
  `'kinds.generated_marker_lines' must be a whole number of at least 1`.

## Architecture

- `src/config/keys.rs`: `Key::GeneratedMarkerLines`, `Type::AtLeast(min)`.
- `src/engine.rs`: `KindOptions.generated_marker_lines` from the settings.

## Deviations from the plan

- The `get_config` examples of tasks 35–63 show the key (separate commit).

## Tests

- examples E64.1–E64.7 (fixture `marker-lines`)
- unit: `request::tests::kinds`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = generated", "kinds": { "generated_marker_lines": 20 } }`
