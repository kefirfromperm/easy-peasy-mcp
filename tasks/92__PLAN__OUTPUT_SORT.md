# Plan for task 92: `output.sort`

## Design

- `src/config/keys.rs`: `Key::Sort` after `GroupRepeated`, `Type::Choice` of `relevance` and `path`, `modified`,
  `size`, `matches` with ` asc` / ` desc`, default `"relevance"`.
- `src/output/sort.rs` (new): `Sort` parsed from the value; `sort(files, sort)` — a stable sort of the files in path
  order by the value, `desc` reversing only the value, missing values last (OUTPUT.md, clarified in its own commit);
  `relevance` keeps the path order until task 93.
- `src/engine.rs`: the search computes the metadata also when the order needs the size or the date; the files are
  sorted after the search, before the result is kept; the metadata is dropped again without `meta`.
- `src/description.rs`: the schema.

## Tests

- examples E92.1–E92.11 (fixture `sorting`)
- unit: the orders, equal and missing values, the request field

## Steps

1. failing tests
2. key, the sort

## Risks

- sorting by size or date reads the metadata of every matched file, also its language and kind
