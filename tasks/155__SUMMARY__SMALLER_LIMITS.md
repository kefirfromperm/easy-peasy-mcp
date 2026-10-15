# Summary of task 155: Smaller default output limits

Server version: `0.149.0`.

## What was built

- smaller built-in limits in spec 2 ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2),
  [CONFIG.md#output](../CONFIG.md#output)):

  | Limit                  | Spec 1  | Spec 2 |
  |------------------------|---------|--------|
  | `max_files`            | `100`   | `20`   |
  | `max_lines`            | `500`   | `100`  |
  | `max_matches`          | `1000`  | `200`  |
  | `max_matches_per_file` | `50`    | `10`   |
  | `max_output_per_file`  | `4000`  | `800`  |
  | `max_output`           | `40000` | `8000` |

  `max_line_length`, `budget`, `facet_limit` and `max_output_file_size` are unchanged; a configured or requested
  limit applies as before
- versioning: the new limits are a part of the defaults of spec 2, like the `omit` default of task 141;
  `"version": 1` keeps the limits of spec 1 when the configuration doesn't set them
  ([TOOLS.md#versioning](../TOOLS.md#versioning)); the write tools follow: `output.max_lines` of `replace` is 100 in
  spec 2
- a response written to a file ([OUTPUT.md#limits-for-a-file](../OUTPUT.md#limits-for-a-file)) takes the count
  limits of spec 1 (`max_files`, `max_lines`, `max_matches`, `max_matches_per_file`) when neither the request nor the
  configuration sets them: a file is the way to take a big result out of the context
- a batch keeps its rule: `max_output` is split between the queries ([OUTPUT.md#batch](../OUTPUT.md#batch))
- [OUTPUT.md#how-to-get-more](../OUTPUT.md#how-to-get-more) (new section) says how a cut response tells the agent to
  get more, with no new response part: the names in `truncated: yes (...)` are the options to raise, `next cursor`
  pages on, the funnel of a huge result (now from 201 files) says how to narrow
- a bug fixed on the way: the next page of a request without `output` (`"output": {"cursor": ...}`) failed with
  `cursor_mismatch`, because the hash saw `"output": {}`; with the new limits a plain request is the common one to be
  cut. The cursor of a request with only `output.file` changed with it (E133.15)

## Architecture

- `config/keys.rs`: the new values in `Key::default_value`; `Key::default_for(1)` returns the limits of spec 1. The
  engine already applies `default_for` to every key of source `default`, at the start of a session and per request,
  so no other code reads the spec version for the limits
- `engine.rs` `file_limits`: the count limits of a file from the request, else from the configuration, else
  `default_for(1)`
- `cursor.rs` `request_hash`: an `output` left empty after removing `cursor` and `file` is removed

## Deviations from the plan

None; the cursor fix was found while writing E155.1 and is in the plan.

## Tests

- examples E155.1–E155.9 (fixture `smaller-limits`: `one.txt` with 15 matches, 25 files of 5): a broad query cut by
  `max_files`, `max_lines` and `max_matches_per_file` with its next page, a file cut by `max_matches_per_file`,
  more than 20 files, the limits raised in the request and in the configuration, `"version": 1`, a batch,
  `output.file`, the funnel of a huge result, `get_config`
- earlier examples: the 137 examples files written for spec 1 are unchanged; E130.13, E141.11, E141.12 and E150.6
  (`get_config` in spec 2) show the new defaults; E133.15 has the cursor of the plain request
- unit: `the_limits_per_spec` (`config/keys.rs`), `a_request_without_output_has_the_hash_of_its_next_page`
  (`cursor.rs`)

## Limitations

- the huge-result funnel needs more than 200 matched files with the default `max_files`; the examples show it with
  `max_files = 2`
- a file cut by a per-file limit has no cursor: the limit is raised instead

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --root .
```

```json
{ "query": "text has 'TODO'" }
```

A broad query answers at most 20 files, 100 lines and 10 matches per file, with `truncated: yes (...)` and
`next cursor: ...`; `{"version": 1, ...}` answers with the old limits.
