# Plan for task 44: `traversal.min_depth`

## Design

- `src/config.rs`: `traversal.min_depth` (a whole number) in the file values, `Settings` with its source and as a
  `Mutable` key; `get_config` shows the `[traversal]` section; `section_keys` gives the keys of a section to the
  request check.
- `src/request.rs`: the request object `traversal` with `min_depth`: `'traversal' must be an object`,
  `'traversal.min_depth' must be a whole number`, `unknown field 'traversal.<key>'` (did you mean, the keys of the
  section); another key of the section is `not_implemented` until its task.
- `src/engine.rs`: the min depth of the request, else of the settings; `src/search/mod.rs`: `SearchOptions.min_depth`
  — entries closer than it to the working directory are not returned, the walk goes on below them.
- `src/description.rs`: `traversal.min_depth` in the `find` schema.

## Tests

- examples E44.1–E44.7 (fixture `depth`)
- unit: the request field; the file value; the rendering of the traversal section

## Steps

1. failing tests
2. key, request field, search

## Risks

- none
