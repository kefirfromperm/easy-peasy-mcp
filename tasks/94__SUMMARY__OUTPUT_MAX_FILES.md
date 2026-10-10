# Summary of task 94: `output.max_files`

Server version: `0.94.0`.

## What was built

- `output.max_files` (CONFIG.md#output, OUTPUT.md#limits-and-truncation), default `100`, at least 1: from the config
  file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.max_files`; invalid values like every key with a minimum.
- In `files` and `lines` detail only the first files of the sorted list are listed (repeated lines are grouped among
  them); the counts stay the totals of all matched files and the result cache keeps them all.
- `truncated_by` in the order of the limits table: the text header `truncated: yes (max_files)` after the result id
  (`max_files, timeout` with both), JSON `"truncated": true, "truncated_by": ["max_files"]`.

## Architecture

- `src/output/mod.rs`: `Limits { max_files }` in `Response`, `Response::listed`, `Response::truncated_by`, used by
  both formats.
- `src/engine.rs`: the limits from the settings; `highlight` and `block_options` split from `try_find`.
- `src/config/keys.rs`: `Key::MaxFiles`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–93 show the key and OUTPUT.md says the order of `truncated_by` and where the
  limits apply (separate commits). `tests/mcp_stdio.rs` uses `output.chars_per_token` as the key which is not
  implemented yet.

## Tests

- examples E94.1–E94.11 (fixture `sorting` of task 92)
- unit: `output::tests::only_the_first_files_are_listed`, `request::tests::output`

## Limitations and known gaps

- The cursor of the next page comes with task 101.

## Try it

`{ "query": "text has 'TODO'", "output": { "detail": "files", "max_files": 5, "sort": "modified desc" } }`
