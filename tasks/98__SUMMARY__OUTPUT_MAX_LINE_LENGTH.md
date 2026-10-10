# Summary of task 98: `output.max_line_length`

Server version: `0.98.0`.

## What was built

- `output.max_line_length` (CONFIG.md#output, OUTPUT.md#long-lines), default `300`, at least 20: from the config file,
  shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.max_line_length`; invalid values like every key with a minimum.
- A line of a block longer than that (in characters, after dedent) is cut: a matched line to windows around the
  merged parts of its fragments, `(max − their characters) / (2 × parts)` characters on each side, touching windows
  merged; a context line to its first `max` characters. `…` marks the cuts; JSON lines get `"cut": true` before
  `text`; `col` refers to the original line. The repeated lines are not cut.

## Architecture

- `src/output/lines.rs`: `windows`, `write_window` (a window of a line with its markers; `write_marked` uses it for
  the whole line), `write_cut`, `cut_line`; `render` cuts every line.
- `src/output/json.rs`: `Line.cut`; `src/output/mod.rs`: `Limits.max_line_length`.
- `src/config/keys.rs`: `Key::MaxLineLength`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–97 show the key and OUTPUT.md says how a line is cut (separate commits).

## Tests

- examples E98.1–E98.10 (fixture `long-lines`)
- unit: `output::lines::tests::long_lines_cut_to_windows`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'unwrap('", "output": { "max_line_length": 80 } }`
