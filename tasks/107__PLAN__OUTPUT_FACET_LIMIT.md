# Plan for task 107: `output.facet_limit`

## Design

- `src/config/keys.rs`: `Key::FacetLimit` after `FacetDepth`, `Type::AtLeast(1)`, default `8`.
- `src/output/mod.rs`: `Response.facet_limit` used by the text lines and the JSON object of the facets instead of the
  constant `facets::LIMIT`, which goes away; `src/engine.rs`, `src/description.rs`.

## Tests

- examples E107.1–E107.8 (fixture `huge` of task 104)
- unit: the request field (the lines over the limit are tested by `output::facets::tests::values_over_the_limit`)

## Steps

1. failing tests
2. key, the limit from the settings

## Risks

- none
