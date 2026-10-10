# Plan for task 105: `output.facets`

## Design

- `src/config/keys.rs`: `Type::Facets` — a list of `dir`, `ext`, `lang`, `kind`, `git`, `condition`; `Key::Facets`
  after `Funnel`, default `['dir', 'lang', 'kind', 'condition']`; `detail = facets` is no longer unimplemented.
- `src/output/mod.rs`: `Detail::Facets` between `Count` and `Files`; `Response.facets` (the names, the depth of
  `dir`); the facet lines after the legend and the funnel, no files.
- `src/output/facets.rs`: the values of every facet over all matched files (OUTPUT.md#facets, clarified in its own
  commit), the text lines (a column of 12, ` · `, `… N more (M)` over the limit) and the JSON object.
- `src/search/mod.rs`: `SearchOptions.git_status` — the git status of a matched file for the `git` facet, without a
  warning; `Found.git`.
- `src/engine.rs`: the metadata (language, kind) with `detail = facets`; `src/description.rs`.

## Tests

- examples E105.1–E105.9 (fixtures `huge` of task 104, `gitrepo` of task 17)
- unit: the values of the facets, the lines, the type, the request field

## Steps

1. failing tests
2. the type and key, the facets, the output

## Risks

- `facet_depth` and `facet_limit` come with tasks 106 and 107: 2 and 8 until then
