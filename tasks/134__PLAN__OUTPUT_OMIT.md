# Plan for task 134: every part of the output optional, `output.omit`

## Design

The spec change is committed already ([OUTPUT.md#omit](../OUTPUT.md#omit)).

### The value

- `src/output/omit.rs` (new): `Part` (the twelve parts in the order of the table, with their names) and `Omit`, a
  small set of parts: `Omit::from_value` (`"all"` or a list of names), `Omit::has(Part)`
- `config/keys.rs`: `Key::Omit` (`output.omit`, after `path_style`), default `[]`, `Type::Omit`: `"all"` or a list
  of part names, as `KeyValue::Text("all")` or `KeyValue::List`; `expected` / `must_be` name the parts. The request,
  the config file, `update_config` and `get_config` take it through the key like the other lists

### Where the parts are left out

- engine (`respond`): `funnel` is `funnel = never` (no funnel, no hints), `hints` drops the hints; `match_ids`
  clears the ids and the escape levels of the fragments and the path fragments of the files before the response is
  made, so the markers, the columns, JSON and the `auto` rule see matches without ids
- `Response::omit` for the rest:
  - text header: the parts of the header line in a list joined with two spaces; `counts` and `result` left out;
    no line when the list is empty (`Response::header_line`)
  - the legend, the empty line before the files only after something, the warnings and their empty line
  - `push_file`: `conditions`, `file_counts`
  - `lines::render(…, omit)`: `line_numbers`, `gaps`, `indent`
  - JSON: `counts` / `counts_exact`, `result`, `legend`, `warnings`; the file `conditions`, `lines`, `matches`; the
    block `indent`; the line `n` becomes optional; `ids` of a fragment are skipped when empty
- the summary of `output.file` takes the header from `Response::header_line`, not the first line of the text

## Rejected

- leaving out the ids while writing the markers: `write_window`, `columns_text`, the JSON fragments, the repeated
  lines and the path fragments would all need the flag; a fragment without ids is written without them in every
  place already, once `write_window` skips the empty prefix
- a flag per part in `Response`: the set keeps the struct small and the names in one place

## Tests

- E134.1–E134.9 (`tests/examples.rs`)
- unit tests: `Type::Omit` (TOML and JSON, `"all"`, unknown parts), `Omit::from_value`, `lines::render` without line
  numbers, gaps and indentation, a marker without ids, `Response::to_text` with an empty header and without the
  warnings, JSON without `n` and `ids`
- earlier examples which change: `get_config` with the `[output]` section (`omit = []`), the `find` description
  and schema (E117.2, E131.1, E121.8) — fixed in their own commit
- the examples of every earlier task still pass

## Steps

1. red: the unit tests
2. `Part`, `Omit`, `Key::Omit`
3. the engine parts, then the response parts in text and JSON, the summary of `output.file`
4. description and schema
5. fix the earlier examples
6. summary, status `implemented`, minor version bump `0.125.0` → `0.126.0`

## Risks

- clearing the ids changes the grouping of repeated lines: lines which differ only by their ids are one repeated
  line, which the spec says
- the header of a batch section can be empty: its section line starts with the first line of its response
