# Summary of task 107: `output.facet_limit`

Server version: `0.107.0`.

## What was built

- `output.facet_limit` (CONFIG.md#output, OUTPUT.md#facets), default `8`, at least 1: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.facet_limit`; invalid values like every key with a minimum.
- A facet lists at most that many values; the others end with ` · … N more (M)` (`N` values with `M` files), in JSON
  `"… N more": M`.

## Architecture

- `src/config/keys.rs`: `Key::FacetLimit`; `src/output/mod.rs`: `Response.facet_limit` for the text lines and the
  JSON object (`src/output/json.rs`), the constant `facets::LIMIT` is gone; `src/engine.rs`, `src/description.rs`.
- `src/lib.rs`: `recursion_limit = "256"` for the `json!` literal of the input schema.

## Deviations from the plan

- The `get_config` examples of tasks 35–106 show the key (separate commit).

## Tests

- examples E107.1–E107.8 (fixture `huge` of task 104)
- unit: `request::tests::output_limits`, `output::facets::tests::values_over_the_limit`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "detail": "facets", "facet_limit": 3 } }`
