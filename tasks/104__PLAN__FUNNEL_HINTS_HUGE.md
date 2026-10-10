# Plan for task 104: hints of a huge result

## Design

- `src/output/facets.rs` (new): `dir_value` — the directory `facet_depth` levels below the working directory (a file
  higher up in its own directory, `./` for the working directory), `count` — the files per value, more files first
  then by value; task 105 builds the facets on it.
- `src/hints.rs`: `huge` — `dir:` and `lang:` with the four values with the most files (` · `), and `narrow with:`
  (OUTPUT.md#hints, clarified in its own commit): `not path ~ '<dir>/**'` for the first directory with at least a
  third of the files but not all, `kind = source` without a `kind` condition when at least a tenth of the files are
  not source, `/w` for a `text ... has` condition without `/w` and `/n`.
- `src/query/mod.rs`: what the narrowing needs of a query (a `kind` condition, a `text has` condition without `/w`
  and `/n`).
- `src/engine.rs`: with a shown funnel and more than 10 × `max_files` files the hints of a huge result; the search
  computes the metadata (language, kind) whenever the funnel may be shown, dropped again without `meta`.

## Tests

- examples E104.1–E104.5 (fixture `huge`)
- unit: the directory values, the order of the values, the narrowing of a query

## Steps

1. failing tests
2. the facet values, the hints

## Risks

- none
