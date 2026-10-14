# Summary of task 136: counts per file and per line, `output.count_by`

Server version: `0.132.0`.

## What was built

[OUTPUT.md#counts-per-file-and-per-line](../OUTPUT.md#counts-per-file-and-per-line): `output.count_by = total |
file | line` sets the grain of the counts of `detail = count`, so an agent asks for the matches per file (`grep -c`)
or the matched line numbers with their counts (`grep -n -o | uniq -c`) and pays for nothing else:

```
files: 3  lines: 6  matches: 7  result: r3

src/
lib.rs  lines: 2  matches: 3
  2: 2
  4: 1
tests/
load.rs  lines: 3  matches: 3
  3: 1
  4: 1
  5: 1
src/
main.rs  lines: 1  matches: 1
  2: 1
```

- `total` (the default) is the response of `detail = count` as before
- `file`: the totals, then the files as in `files` detail without the legend and the conditions — under directory
  headers, sorted by `sort`, with `(cut)`, the encoding, `meta` and the path fragments. It is the same as
  `detail = files` with `omit = ["legend", "conditions"]` (E136.3)
- `line`: under every file its listed lines, `  42: 2`, the count being the matches which start in the line; a line
  a multiline match only continues into has `0`, so the counts of the lines add up to the file's `matches` (E136.5)
- JSON: the files without `conditions`; `matched_lines: [{ "n", "matches" }]` with `line`, `matches` also when 0
- `omit`: `counts`, `file_counts` (the paths alone), `line_numbers` (`  2`), the new part `line_counts` (`  42`, the
  list of the matched line numbers); without both the lines are left out; `"all"` leaves the paths
- the limits: `max_files` and `max_output` cut the list, with `line` also `max_lines`, `max_matches` and
  `max_matches_per_file`; `max_output` counts the paths (there is no text), `max_output_per_file` and
  `max_line_length` don't apply; a line cut by `max_matches` shows its listed matches and the next page the rest
  (E136.7); the totals are the real ones, a cut list has a cursor
- the budget of a requested `count` with `count_by`: as requested, `file` (from `line`), the file list cut to the
  budget (at least 10 files or all), the totals; a lowered `count_by` is `count by: file (budget N)` in the header,
  `count_by_used` in JSON and in the summary of a response written to a file. A higher detail which the budget lowers
  to `count` gets the totals
- `since` and `within` work as on every result with `count` detail; with `since` the changed files are listed with
  their marks and lines (E136.11)
- a request option and a configuration key (`[output] count_by`, after `detail`), `update_config`, `get_config` and
  the items of `queries`; the other details ignore it; the `find` description has one line about it, the schema its
  field

## Architecture

- `config/keys.rs`: `Key::CountBy`, a choice, default `"total"`; `config/mod.rs` names it in `[output]`
- `output/omit.rs`: `Part::LineCounts` after `LineNumbers`
- `output/mod.rs`: `CountBy`, the field `Response::count_by`; `Response::lists()` (files and lines detail, or count
  with `count_by`) replaces the `detail >= Files` checks of the listed files, `max_files` and the truncation;
  `listing()` uses the listing of `lines` detail for `count_by = line` without block options; `push_line_counts`
  writes the lines; `BudgetUse::count_by` is the lowered `count_by` for the header
- `output/listing.rs`: `Listing::new` takes the block options as an `Option`: without them `max_output_per_file`
  doesn't apply and `max_output` counts the paths only
- `output/lines.rs`: `line_counts` — the lines of the fragments with the number of fragments starting in each
- `output/json.rs`: `matched_lines`, `count_by_used`, no `conditions` in `count` detail
- `output/budget.rs`: `fit_counts` for the levels of the counts; the search in halves of the file list moved into
  `cut_files`, shared with level 4

## Deviations from the plan

None in the design. The spec got two small follow-ups in their own commits: `matches` of a matched line is written
also when 0, and `count_by_used` is a field of the summary of a response written to a file.

## Tests

- E136.1–E136.12 in `tasks/136__EXAMPLES__COUNT_LEVELS.md` (fixtures `count-by` and `count-by-many`): the grains in
  text and JSON, `count_by = file` against `detail = files`, every `omit` part, a multiline match and a `line( ... )`
  group, `group_dirs = false` and `sort`, the limits with their cursors, the four budget levels, the config file,
  `update_config` and `queries`, `since`, wrong values
- unit tests: `Key::CountBy` after `detail`, `Part::LineCounts`, the text of the line counts with a multiline
  fragment and the omit parts (`output/mod.rs`), the budget levels of the counts and the totals of a lowered detail
  (`output/budget.rs`)
- the earlier examples changed: `count_by = "total"` in every `get_config` of `[output]`, the `find` description and
  schema (E117.2, E131.1), the list of the omit parts (E134.9)

## Limitations

- counts per directory, language or kind stay with `detail = facets`; counts per condition inside a file are not
  available

## How to try it

```sh
cargo build --release
```

```json
{ "query": "text has 'unwrap('", "output": { "detail": "count", "count_by": "line", "omit": ["line_counts"] } }
```

returns the matched line numbers of every file, the cheapest way to know where to edit.
