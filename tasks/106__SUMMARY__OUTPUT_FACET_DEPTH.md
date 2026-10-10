# Summary of task 106: `output.facet_depth`

Server version: `0.106.0`.

## What was built

- `output.facet_depth` (CONFIG.md#output, OUTPUT.md#facets), default `2`, at least 1: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.facet_depth`; invalid values like every key with a minimum.
- The `dir` values of `detail = facets` and of the hints of a huge result are the directories that many levels below
  the working directory; a file higher up counts in its own directory.

## Architecture

- `src/config/keys.rs`: `Key::FacetDepth`; `src/engine.rs` passes it to `output::facets::facets` and
  `hints::huge` (the constant `hints::FACET_DEPTH` is gone); `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–105 show the key (separate commit).

## Tests

- examples E106.1–E106.8 (fixture `huge` of task 104)
- unit: `request::tests::output_limits`, `output::facets::tests::directories_at_a_depth`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "detail": "facets", "facets": ["dir"], "facet_depth": 3 } }`
