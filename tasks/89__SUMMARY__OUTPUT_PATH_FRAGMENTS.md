# Summary of task 89: `output.path_fragments`

Server version: `0.89.0`.

## What was built

- `output.path_fragments` (CONFIG.md#output, OUTPUT.md#path-fragments), default `false`: from the config file, shown
  by `get_config` with its source, changed and reset by `update_config`, replaced for one request by
  `output.path_fragments`; invalid values like every boolean key.
- With it every listed file gets the fragments of its contributing `name` and `path` conditions: every regex match
  (all of the value for `=` and `~`), the runs of matched characters of `~~`; a `name` fragment is in the name at the
  end of the path; identical ones merged. Not counted as matches.
- JSON: `path_fragments` per file (after `conditions`), `col` in the characters of `path`. Text: a line after the
  file line, `  path: src/⟦#1:config⟧.rs` with markers, `  path @ 5..11 #1` with columns, none with `none`.

## Architecture

- `src/query/fuzzy.rs`: `Fuzzy::positions` — the scoring DP keeps its rows and walks back the best alignment.
- `src/query/eval.rs`: `Filter::path_ranges`.
- `src/search/mod.rs`: `SearchOptions.path_fragments`, `Found.path_fragments: Option<PathFragments>`,
  `PathFragments::of` (from the contributing leaves) and `shown` (the working directory and native separators,
  applied by `engine::search_all`).
- `src/output/mod.rs`, `src/output/json.rs`, `src/output/lines.rs` (`columns_text`); `src/config/keys.rs`:
  `Key::PathFragments`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–88 show the key and OUTPUT.md got the section Path fragments (separate
  commits); E89.7 was fixed (a directory needs `type = dir`).

## Tests

- examples E89.1–E89.14 (fixture `path-frags`)
- unit: `query::fuzzy::tests::positions_of_the_best_alignment`, `query::eval::tests::ranges_in_the_path`,
  `search::tests::path_fragments_after_the_working_directory`, `request::tests::output`

## Limitations and known gaps

- The fragments are computed on the NFC path which the conditions see; a name stored decomposed (macOS) is shown
  composed in the `path:` line and its columns count the composed characters.

## Try it

`{ "query": "name ~~ 'cfgldr'", "output": { "path_fragments": true } }`
