# Summary of task 104: hints of a huge result

Server version: `0.104.0`.

## What was built

- With a shown funnel and more than 10 × `max_files` files the response has `hints:` after the funnel (JSON
  `hints`):
  - `dir:` and `lang:` — the four values of the facet with the most files (`dir` two levels below the working
    directory until `facet_depth` comes with task 106), more files first, then in the order of the values, joined
    with ` · `
  - `narrow with:` — `not path ~ '<dir>/**'` for the directory right below the working directory with at least a
    third of the files but not all, `kind = source` when the query has no `kind` condition and a tenth of the files or
    more are not source, `/w` for a `text ... has` condition without `/w` and `/n`; left out without suggestions
- The search computes the metadata (language, kind) whenever the funnel may be shown; it is dropped again without
  `meta`.

## Architecture

- `src/output/facets.rs` (new): `dir_value`, `count` — shared with the facets of task 105.
- `src/hints.rs`: `huge`, `narrow`, `FACET_DEPTH`; `src/query/mod.rs`: `Query.narrowing` (`Narrowing`).
- `src/engine.rs`: `response_hints` (empty or huge), `hints_deadline`; the metadata is dropped after the hints.

## Deviations from the plan

- OUTPUT.md got the details of these hints (separate commit); E102.5 shows the hints of its huge result (separate
  commit).

## Tests

- examples E104.1–E104.5 (fixture `huge`)
- unit: `output::facets::tests::*`, `query::tests::what_narrows_a_query`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'error'", "output": { "max_files": 10 } }` in a project with many matches.
