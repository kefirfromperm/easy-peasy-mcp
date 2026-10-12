# Plan for task 124: no line for a facet without values

## Design

One condition in the text format, `src/output/mod.rs`: the loop which writes `facet.line(facet_limit)` for
`detail = facets` skips a facet whose `values` are empty. The JSON format already writes such a facet as `{}`
(`Values` serializes an empty map), as the spec now says; E124.3 and E124.4 pin it.

- `Facet::is_empty()` names the check, so the text writer reads as the spec
- the budget estimates the rendered text, so a facets response without the empty line is estimated without it; no
  change in `src/output/budget.rs`

No new types beyond the method, no modules, no dependencies.

## Rejected

- leaving empty facets out of `facets()`: JSON would lose the key, and the spec keeps the keys of `output.facets`
- `condition: (none)`: a line which costs tokens and says nothing

## Tests

- examples E124.1–E124.4 (`tests/examples.rs`)
- unit test in `src/output/mod.rs`: a facets response with an empty facet between two others writes the two lines
  only
- the examples of every earlier task, in particular 105–107 (facets) and 108 (budget), still pass

## Steps

1. the unit test, failing (with the examples of the previous commit)
2. the fix
3. summary, status `implemented`, patch version bump `0.122.0` → `0.122.1` (a fix of the output, no new feature)

## Risks

- none known: no earlier example shows an empty facet line
