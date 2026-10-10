# Summary of task 2: Empty query: all files

Server version: `0.2.0`.

## What was built

- A request without `query`, with `""` or with whitespace and line breaks only is the empty query
  (QUERY.md#basics): it matches every regular file the traversal lets through, like `find -type f`; hidden,
  ignored and generated files stay skipped.
- The response of the empty query has no legend and its file lines have no `[ ]`; the spec now says so
  (OUTPUT.md#text-format).
- A query of comments only is still `not_implemented` until task 8.
- The `find` description says `Empty query = all files.`

## Architecture

`Query.filter` is `Option<Filter>`: `None` is the empty query. `search::evaluate` returns an empty list of condition
ids for it, and the output writes the file name alone when the list is empty.

## Deviations from the plan

None.

## Tests

- examples E2.1–E2.3, fixture `flat`
- unit: request (missing, null arguments, whitespace query), `query::parse` of whitespace, subset check of the empty
  query, output of a file line without conditions

## Limitations and known gaps

- an empty result has no funnel and hints (tasks 102–103); more than 100 files are not truncated (task 94)

## Try it

`{}` or `{ "query": "" }` as the arguments of `find`.
