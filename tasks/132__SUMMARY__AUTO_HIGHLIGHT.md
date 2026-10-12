# Summary of task 132: `highlight = auto` without markers which outgrow the line

Server version: `0.124.0`.

## What was built

[OUTPUT.md#highlight](../OUTPUT.md#highlight): in the text format `auto` writes a matched line with markers, unless
its markers would add more characters than the line has; then the line is written as with `none`. The table which
started the task now costs its own size for the header and the separator:

```
1:| Task | Status |
2:|------|--------|
3:⟦#2,#3:|⟧ Parse the query ⟦#2,#3:|⟧ done ⟦#2,#3:|⟧
```

- the rule is per line; the lines joined by a multiline fragment are counted together, so a marker never opens on a
  written line and closes on a plain one
- the line is counted as it is shown: after the dedent and the long line cut with its `…`; the markers with their
  ids and escape levels; as many characters as the line keeps the markers
- a repeated line follows the rule of its text in the `repeated` section
- `marker_in_content` lists only files with a matched line written with markers
- `highlight = "markers"` marks every fragment; `none`, `columns` and JSON are unchanged; the path fragments keep
  their markers with `auto`

The spec change came first, in its own commit. The tool description still says `⟦ids:text⟧ a match`: a plain line
is still a matched line (`42:`), so no word was added.

## Architecture

- `src/output/mod.rs`: `Highlight::Auto(open, close)` next to `Markers`, `Highlight::markers()` gives the markers of
  both; the path fragments write `Auto` as `Markers`; `all_warnings` leaves out the plain lines of `auto`
- `src/engine.rs`: `highlight` returns `Auto` for `auto` in text
- `src/output/lines.rs`: `render` asks `plain_lines` per block which lines are written plain: every line which a part
  touches is rendered as shown without and with markers, the lines a part joins form one group with their sums;
  `outgrown(plain, marked)` is the rule; `shown_line` (the repeated lines) applies it to its one line;
  `plain_lines_of` serves the warning

## Deviations from the plan

- the path fragments went through `shown_line` and lost their markers with `auto` (E89.6); they are written with
  `Markers` now, as the spec says
- two startup examples (E96.9, E116.1) changed too, fixed in their own commit

## Tests

- E132.1–E132.7 (`tests/examples.rs`, fixture `auto-highlight`): the table, explicit `markers`, the boundary, the
  dedent, repeated lines, the warning, a multiline fragment
- unit tests in `src/output/lines.rs`: the boundary and `shown_line`, a multiline group, a cut line counted with its
  `…`
- earlier examples fixed to the rule (short matched lines such as `1:TODO`, `1:id`, `8:}` are plain now): E23.1–E23.4,
  E26.3, E26.6, E27.3, E27.4, E28.9, E28.11, E92.5, E94.3, E96.2, E96.4, E96.5, E96.9, E97.2, E97.4, E101.5, E116.1,
  E121.40, E121.45, E131.3
- the examples of every earlier task pass

## Limitations

- every line a fragment touches is rendered twice more with `auto`; the output is bounded by `max_output`
- `marker_in_content` judges the lines in the full blocks of a file, also the lines a limit leaves out, as before
- no configurable threshold and no merging of fragments (out of scope)

## How to try it

```json
{ "query": "name ~ '*.md' and line(text has '|')" }
```

on a Markdown table writes the short lines plain; `"output": { "highlight": "markers" }` marks every `|`.
