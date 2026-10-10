# Summary of task 102: `output.funnel`

Server version: `0.102.0`.

## What was built

- `output.funnel` (CONFIG.md#output, OUTPUT.md#funnel), default `"auto"`: from the config file, shown by `get_config`
  with its source, changed and reset by `update_config`, replaced for one request by `output.funnel`; invalid values
  like every choice key.
- `auto`: the funnel of an empty result or of more than 10 × `max_files` files; `always`: every response with
  `count` detail or higher; `never`: none.
- The steps: the items of a top-level `and` (metadata steps first, then the steps with content conditions, each in
  the order of the query; a bracket group, `not ...`, `line( ... )`, `near( ... )` is one step) with the files left
  after each, the branches of a top-level `or` with the files counted in their first true branch, or the whole
  query as one step. Unknown doesn't pass; matched entries which are left out (generated, binary excluded,
  unreadable) are not counted.
- Text: `funnel:` after the legend (after the header without one), the counts right-aligned 40 characters after the
  indent; JSON: `"funnel": [{ "step", "files" }]` after `result`.

## Architecture

- `src/query/mod.rs`: `Query.funnel` (`Funnel`, `FunnelKind`, `FunnelStep`), `funnel_steps`, `item_tokens` (the
  tokens of an item with its brackets and `not`), `has_content`.
- `src/search/mod.rs`: `SearchOptions.funnel`, `Walk::funnel_point` / `count_funnel` (atomic counters),
  `SearchResult.funnel`; the last step of `and` is not evaluated again for an entry which doesn't match.
- `src/output/funnel.rs` (new): the text lines; `Response.funnel`, JSON `FunnelStep`.
- `src/engine.rs`: `shown_funnel`, `search_all` returns a `SearchResult` with the summed counts; `working_dirs`,
  `detail` and `limits` split from `try_find`; `src/config/keys.rs`: `Key::Funnel`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–101 show the key and OUTPUT.md got the layout of the funnel and its JSON
  form (separate commits). No earlier example had an empty result, so none changed.

## Tests

- examples E102.1–E102.12 (fixture `funnel`)
- unit: `query::tests::the_steps_of_the_funnel`, `output::funnel::tests::counts_end_in_one_column`,
  `request::tests::output_limits`

## Limitations and known gaps

- The hints come with tasks 103 and 104. A file reachable through several hard links in one working directory is
  counted once per path.

## Try it

`{ "query": "lang = rust and modified > -7d and text has 'parseQuery'", "output": { "funnel": "always" } }`
