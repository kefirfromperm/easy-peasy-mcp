# Summary of task 86: `output.group_repeated`

Server version: `0.86.0`.

## What was built

- `output.group_repeated` (CONFIG.md#output, OUTPUT.md#repeated-lines), default `3`, `0` — off: from the config file,
  shown by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.group_repeated`; invalid values like every whole number key, with the hint `(0: off)`.
- In `lines` detail a matched line which is identical without its leading and trailing whitespace, with the same
  fragments, in at least that many files is printed once: the text section `repeated:` after the legend
  (`  ×N <line>    path:line, …, … M more`), the JSON field `repeated` (`count`, `text`, `fragments` with columns in
  `text`, the first three `locations`). Its lines and their context are left out of the files; a file without other
  matched lines is not printed. The counts don't change; `detail = files` lists every file.
- A line touched by a multiline fragment or with a fragment only in its whitespace is never grouped.

## Architecture

- `src/output/repeated.rs` (new): `repeated(files, min)` → `Repeated { lines, … }` and `Repeated::rest` (the fragments
  of a file not on repeated lines).
- `src/output/lines.rs`: `marked_line` (a line with its markers), shared with the blocks by `write_marked`.
- `src/output/mod.rs`, `src/output/json.rs`: `Response.group_repeated`, the section, the files left out.
- `src/config/keys.rs`: `Key::GroupRepeated`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–85 show the key (separate commit); OUTPUT.md got the layout of the section
  (separate commit).
- A fragment which covers part of the leading or trailing whitespace (a whole-line fragment) is cut to the trimmed text.

## Tests

- examples E86.1–E86.10 (fixture `repeated`)
- unit: `output::repeated::tests::*` (the minimum, other ids, multiline fragments, the order),
  `output::tests::repeated_lines_show_three_locations`, `request::tests::output`

## Limitations and known gaps

- A repeated line shown as the context of another matched line of a file is printed there as a context line.

## Try it

`{ "query": "text has 'use anyhow'", "output": { "group_repeated": 2 } }`
