# Summary of task 134: every part of the output optional, `output.omit`

Server version: `0.126.0`.

## What was built

[OUTPUT.md#omit](../OUTPUT.md#omit): `output.omit` leaves parts of the response out, so the agent pays only for
what it reads. `"all"` gives the bare grep-like output:

```
src/
lib.rs
let raw = read().⟦unwrap(⟧);
parse(&text).⟦unwrap(⟧)
main.rs
let cfg = load().⟦unwrap(⟧);
```

instead of the 290 characters of the default response with its header, legend, conditions, counts, ids, line
numbers, gaps, `⇥4` lines and warning.

- the parts: `counts`, `result`, `legend`, `funnel`, `hints`, `conditions`, `file_counts`, `match_ids`,
  `line_numbers`, `gaps`, `indent`, `warnings`. Each one leaves out its part in text and JSON (`gaps` has nothing to
  leave out in JSON)
- `omit` is a request option and a configuration key (`[output] omit`, `update_config`, `get_config`, the items of
  `queries`); a list replaces the configured one, `"all"` is every part
- never left out: `version` and `exists` of JSON, `exists: yes`; the truncation and the cursor, also with `"all"`;
  `detail: ... (budget N)` and `since`; the paths and the lines
- the header line keeps the parts which are left and is dropped when none is left. The empty line before the files
  is written only after something, so a `count` without its counts and its result id is empty
- `funnel` is `funnel = never` (no hints either), `hints` drops only the hints
- `match_ids` takes the ids and the escape levels out of every match: the markers, the columns, the repeated lines,
  the path fragments and JSON fragments. `auto` counts the markers without them
- `line_numbers` with context lines is allowed: a matched line and a context line then look the same
- the budget estimates the response without the omitted parts, so more fits (E134.8)
- the `find` description has one line about `output.omit`, and the schema has its field

## Architecture

- `src/output/omit.rs` (new): `Part` (the parts with their names) and `Omit`, a set of parts read from the value of
  the key
- `src/config/keys.rs`: `Key::Omit` and `Type::Omit`, either `"all"` (`KeyValue::Text`) or a list of part names
- `src/engine.rs`: `omit(settings)`. `funnel` turns the funnel off in `run`, `hints` drops the hints, and
  `shown_files` / `without_ids` clear the fragment ids for `match_ids` before the response is made
- `src/output/mod.rs`: `Response::omit`. `header` / `header_line` build the header from its parts. `to_text` writes
  the legend and the empty line only when they apply, `push_file` drops the conditions and counts, and
  `all_warnings` is empty without warnings
- `src/output/lines.rs`: `render(…, omit)` drops the line numbers, the gaps and the `⇥N` lines. `write_window` and
  `columns_text` write a match without ids as `⟦text⟧` / `@ s..e`
- `src/output/json.rs`: `counts`, `result`, `legend`, the file counts and conditions, the block `indent`, the line
  `n` (now optional) and fragment `ids` (skipped when empty)
- the summary of `output.file` (task 133) takes its header from `Response::header_line`, and no header line when
  nothing is left of it

## Deviations from the plan and the task

- the task asked for `invalid_value` with the parts in the hint for an unknown part. In a request, a wrong value is
  `invalid_option` in this spec (`'output.omit' must be "all" or a list of parts (...)`, the list in the message like
  `facets`), and `update_config` reports `invalid_value` with the list in the hint
- `match_ids` leaves out the escape levels too: in text they are a part of the id prefix of a marker
- clippy's length limit moved the files of `since` and the ids into `shown_files`

## Tests

- E134.1–E134.9 (`tests/examples.rs`, fixture `omit`): the bare output against the default one (text and JSON),
  every part alone in text and in JSON, `match_ids` with columns, the funnel and the hints, line numbers with
  context, the truncation, `exists` and an empty `count`, the session / request / batch lists, the budget, wrong
  values
- unit tests: `output::omit` (names, values), `Type::Omit` (TOML, JSON, errors), `lines::render` without line
  numbers, gaps and indentation and a marker without ids
- earlier examples fixed in their own commit: `omit = []` in the `[output]` section of every `get_config` response,
  the find description and schema (E117.2, E131.1)
- the examples of every earlier task pass

## Limitations

- with `match_ids` left out, lines which differ only by their ids are one repeated line
- the section line of a batch query without a header starts with the first line of its response (`== a ==  src/`)
- no reordering of the parts and no new short forms of the header (out of scope)

## How to try it

```json
{ "query": "text has 'TODO'", "output": { "omit": "all" } }
```

or for the whole session: `update_config` with `{ "set": "output.omit = ['legend', 'conditions', 'match_ids']" }`.
