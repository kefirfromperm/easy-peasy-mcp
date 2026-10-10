# Summary of task 87: `output.highlight`

Server version: `0.87.0`.

## What was built

- `output.highlight` (CONFIG.md#output, OUTPUT.md#highlight), default `"auto"`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.highlight`; invalid values like every choice key.
- `markers`: `⟦ids:text⟧` inside the lines, in JSON inside `text` without `fragments`.
- `columns`: in text the plain line and after a matched line `  @ s..e ids  @ …` (a multiline fragment ends with
  `line:col`, escape levels after the ids, columns of the original line also after dedent); in JSON `fragments`.
- `none`: plain lines; in JSON no `fragments` and `"match": true` on matched lines.
- `auto`: `markers` in text, `columns` in JSON. The repeated lines follow the style too.

## Architecture

- `src/output/mod.rs`: `Highlight { None, Columns, Markers }`, `Response.highlight` (`auto` resolved by the engine).
- `src/output/lines.rs`: `render` and `shown_line` in a style, `push_columns`, `columns` and `starts_in` shared with
  `src/output/json.rs` (`Line.matched` as `match`).
- `src/config/keys.rs`: `Key::Highlight`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–86 show the key and OUTPUT.md got the layout of the columns line and of
  `match` (separate commits); E87.4 was fixed (the second request of its session has the result `r2`).

## Tests

- examples E87.1–E87.13 (fixture `highlight`, fixture `repeated` of task 86)
- unit: `output::lines::tests::columns_after_the_line`, `request::tests::output`

## Limitations and known gaps

- Custom markers and the `marker_in_content` warning come with task 88.

## Try it

`{ "query": "text has 'unwrap('", "output": { "highlight": "columns" } }`
