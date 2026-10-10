# Summary of task 96: `output.max_matches`

Server version: `0.96.0`.

## What was built

- `output.max_matches` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `1000`, at least 1: from the
  config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request
  by `output.max_matches`; invalid values like every key with a minimum.
- In `lines` detail only the first fragments under the files are listed, in the order of the files and of the
  fragments: a line with some of its fragments listed shows only those marked, a line without one is not listed, a
  file without one is listed with its counts. `truncated: yes (max_matches)` (`max_lines, max_matches` with both),
  JSON `truncated_by`; the counts stay the totals.

## Architecture

- `src/output/listing.rs`: the matches left are cut after the lines; both budgets count what is left.
- `src/output/mod.rs`: `Limits.max_matches`; `src/config/keys.rs`: `Key::MaxMatches`; `src/engine.rs`,
  `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–95 show the key (separate commit).

## Tests

- examples E96.1–E96.10 (fixture `sorting` of task 92)
- unit: `output::listing::tests::the_first_matches`, `request::tests::output`

## Limitations and known gaps

- The cursor of the next page comes with task 101.

## Try it

`{ "query": "text has 'unwrap('", "output": { "max_matches": 50 } }`
