# Plan for task 106: `output.facet_depth`

## Design

- `src/config/keys.rs`: `Key::FacetDepth` after `Facets`, `Type::AtLeast(1)`, default `2`.
- `src/engine.rs`: the depth of the `dir` values of `detail = facets` and of the hints of a huge result from the key
  instead of the constant `hints::FACET_DEPTH`, which goes away; `src/description.rs`.

## Tests

- examples E106.1–E106.8 (fixture `huge` of task 104)
- unit: the request field (the values themselves are tested by `output::facets::tests::directories_at_a_depth`)

## Steps

1. failing tests
2. key, the depth from the settings

## Risks

- none
