# Summary of task 90: `output.path_style`

Server version: `0.90.0`.

## What was built

- `output.path_style` (CONFIG.md#output, OUTPUT.md#paths), default `"relative"`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.path_style`; invalid values like every choice key.
- `absolute`: the canonical working directory (symlinks resolved, without the verbatim prefix of Windows) and the
  path inside it, in the files, the path fragments, the warnings and the directory headers of the text format.

## Architecture

- `src/engine.rs`: with `absolute` the prefix of every working directory is its canonical path
  (`without_verbatim`); the rest of the output is unchanged.
- `src/output/mod.rs`: `path_parts` joins a Windows prefix and the root with one separator.
- `src/config/keys.rs`: `Key::PathStyle`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–89 show the key and OUTPUT.md says what an absolute path is made of
  (separate commits).

## Tests

- examples E90.1–E90.9 (fixture `path-frags` of task 89)
- unit: `output::tests::absolute_paths`, `request::tests::output`

## Limitations and known gaps

- Windows paths are only checked with task 120.

## Try it

`{ "query": "name = 'Cargo.toml'", "output": { "path_style": "absolute" } }`
