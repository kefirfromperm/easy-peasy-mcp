# Plan for task 136: counts per file and per line, `output.count_by`

## Design

The spec change is committed already
([OUTPUT.md#counts-per-file-and-per-line](../OUTPUT.md#counts-per-file-and-per-line)): `output.count_by = total |
file | line`, the omit part `line_counts`, the budget levels with `count by: file (budget N)` / `count_by_used`.

### The value

- `config/keys.rs`: `Key::CountBy` (`output.count_by`, after `detail`), `Type::Choice(["total", "file", "line"])`,
  default `"total"`; the request, the config file, `update_config`, `get_config` and the batch items take it through
  the key like the other choices; `config/mod.rs` names it in the `[output]` section
- `output/omit.rs`: `Part::LineCounts` after `LineNumbers`, so the error lists the parts in the order of the table
- `output/mod.rs`: `CountBy { Total, File, Line }`, a field `count_by` of `Response`; the engine reads it from the
  settings

### The response

`count_by = file` is the file list of `files` detail without the legend and the conditions, `line` adds the counts
of the lines; both reuse the listing of `files` and `lines` detail:

- `Response::lists()`: whether the files are listed — `files` and `lines` detail, or `count` with `count_by` other
  than `total`; it replaces the `detail >= Files` checks of `listed`, `files_cut`, `truncated_by` and JSON
- `Response::listing`: in `count` detail with `line` the listing of `lines` detail without repeated lines, the
  `max_output_per_file` check and the characters of the lines in `max_output`: `Listing::new` takes the block options
  as an `Option`, `None` counts the paths only (as `without_lines`)
- text: after the header the funnel (no legend), an empty line, the files; `push_file` leaves out the conditions in
  `count` detail and writes the lines of `line` as `  42: 2`, the count being the listed fragments which start in the
  line (`lines::starts_in`); `line_numbers` and `line_counts` leave out the parts, without both no line
- JSON: the files without `conditions`; `matched_lines: [{ n, matches }]` with optional fields, left out when both
  parts are omitted
- budget (`output/budget.rs`): `BudgetUse` gets `count_by: Option<CountBy>` — the lowered `count_by`; the header
  writes `count by: <value> (budget N)`, JSON `count_by_used`. A requested `count` with `count_by` other than `total`
  runs its own levels: as requested, `file` (from `line`), the file list cut to the budget (the search in halves of
  level 4, moved into a function), `total`. Lowering a higher detail to `count` sets `count_by = total`

## Rejected

- a new `Detail` level for the counts: the spec keeps the levels, and every `detail` check would need it
- rendering `count_by = file` as `detail = files` with the conditions left out: the budget, the funnel position
  and the JSON legend differ, and `line` needs the listing of `lines` anyway

## Tests

- E136.1–E136.12 (`tests/examples.rs`)
- unit tests: `Key::CountBy` default and values, `Part::LineCounts` in the order, the text of a `line` response with
  a multiline fragment (`0`), without line numbers / counts, the budget levels of `count`
- earlier examples which change: `get_config` with the `[output]` section (`count_by = "total"`), the `find`
  description and schema, the list of the omit parts in the errors — fixed in their own commit
- the examples of every earlier task still pass

## Steps

1. red: the unit tests
2. `Key::CountBy`, `Part::LineCounts`, `CountBy`
3. listing and text, JSON, budget, the summary
4. description and schema
5. fix the earlier examples
6. summary, status `implemented`, minor version bump `0.131.0` → `0.132.0`

## Risks

- the cursor of a page which starts inside a line (`max_matches`): the line is listed again on the next page with
  the matches which are left, as the spec says ("a line shows the matches which are listed")
- the budget of `count` with `count_by` can't reach the facets: the spec keeps `count` and lowers only `count_by`
