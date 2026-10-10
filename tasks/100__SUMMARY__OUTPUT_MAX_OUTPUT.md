# Summary of task 100: `output.max_output`

Server version: `0.100.0`.

## What was built

- `output.max_output` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `40000`, at least 100: from the
  config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request
  by `output.max_output`; invalid values like every key with a minimum.
- In `files` and `lines` detail the characters of the full path of every listed file and of its lines (as
  `max_output_per_file` counts them) are added up in the order of the files: the file whose path would go over is
  not listed, the line which would go over is not listed (its file is), and nothing is listed after them;
  `truncated: yes (max_output)`, JSON `truncated_by`; the counts stay the totals.

## Architecture

- `src/output/listing.rs`: `Listing::limit_output` (a last pass), `Listing.shown`, `shown_lines` shared with
  `first_over`; `Listing::without_lines` takes the limits.
- `src/output/mod.rs`, `src/output/json.rs`: only the first `shown` files are written; `Limits.max_output`.
- `src/config/keys.rs`: `Key::MaxOutput`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–99 show the key and OUTPUT.md says what the limit counts (separate
  commits). `request::tests::output` was split (`output_limits`); its not implemented field is `output.budget`.

## Tests

- examples E100.1–E100.9 (fixture `long-names`, fixture `output-limits` of task 99)
- unit: `output::listing::tests::the_first_characters_of_the_response`, `request::tests::output_limits`

## Limitations and known gaps

- The cursor of the next page comes with task 101.

## Try it

`{ "query": "text has 'TODO'", "output": { "max_output": 2000 } }`
