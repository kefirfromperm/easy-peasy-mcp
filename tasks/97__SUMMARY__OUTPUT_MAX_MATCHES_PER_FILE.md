# Summary of task 97: `output.max_matches_per_file`

Server version: `0.97.0`.

## What was built

- `output.max_matches_per_file` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `50`, at least 1: from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `output.max_matches_per_file`; invalid values like every key with a minimum.
- In `lines` detail only the first fragments of every file are listed; the file gets `(cut)` after its counts,
  `"truncated": true` in JSON (after `score`), and the next file is listed as usual; `truncated_by` names
  `max_matches_per_file`. The per file cut comes first, `max_lines` and `max_matches` count what is left.

## Architecture

- `src/output/listing.rs`: the per file cut and `Listing.cut`; `src/output/mod.rs` (`push_file` with `cut`),
  `src/output/json.rs` (`File.truncated`); `Limits.max_matches_per_file`.
- `src/config/keys.rs`: `Key::MaxMatchesPerFile`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–96 show the key and OUTPUT.md says the per file limits cut first (separate
  commits).

## Tests

- examples E97.1–E97.10 (fixture `sorting` of task 92)
- unit: `output::listing::tests::the_first_matches_of_every_file`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'unwrap('", "output": { "max_matches_per_file": 3 } }`
