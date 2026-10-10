# Plan for task 2: Empty query: all files

## Design

- `request.rs`: `query` missing, `""` or whitespace only is no longer `not_implemented`; `Request.query` stays a
  `String` (empty for a missing field).
- `query/mod.rs`: `Query.filter` becomes `Option<Filter>`; `None` is the empty query, which matches every file the
  traversal lets through. The legend is empty.
- `query/subset.rs`: the empty query is part of the subset; a query of comments only is still `not_implemented`
  (comments come with task 8).
- `search`: no filter → every regular file matches with no condition ids; the implicit `type = file` stays.
- `output`: a file line without condition ids is only the name, no `[ ]` (spec clarified in OUTPUT.md#text-format).
- `description.rs`: the cheat sheet says `Empty query = all files.`

No new dependencies.

## Tests

- examples E2.1–E2.3 (fixture `flat`: hidden, ignored and generated files are not returned)
- unit: request parsing of a missing / empty / whitespace query; `query::parse("")`; subset check of a comment-only
  query; output of a file without conditions
- the earlier examples and tests still pass

## Steps

1. failing tests (examples are in place; unit tests updated for the new behavior)
2. request, query, search, output changes
3. tool description

## Risks

None known.
