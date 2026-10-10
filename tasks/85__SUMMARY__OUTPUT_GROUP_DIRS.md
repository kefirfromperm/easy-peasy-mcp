# Summary of task 85: `output.group_dirs`

Server version: `0.85.0`.

## What was built

- `output.group_dirs` (CONFIG.md#output, OUTPUT.md#directory-groups), default `true`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.group_dirs`; invalid values like every boolean key.
- `true`: the text format writes files under directory headers, as before; `false`: no headers, the full path on
  every file line. JSON paths are always full.

## Architecture

- `src/output/mod.rs`: `Response.group_dirs` in the text format; `src/config/keys.rs`: `Key::GroupDirs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–84 show the key (separate commit).

## Tests

- examples E85.1–E85.7 (fixture `detail` of task 79)
- unit: `output::tests::full_paths_without_groups`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "group_dirs": false } }`
