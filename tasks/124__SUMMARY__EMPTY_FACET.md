# Summary of task 124: no line for a facet without values

Server version: `0.122.1`.

## What was built

With `detail = facets`, a facet whose matched files have no value at all — `condition` of the empty query, `git`
outside of a repository, any facet whose files all lack the value — no longer prints a line like `condition:  ` with
nothing after it ([OUTPUT.md#facets](../OUTPUT.md#facets)). The other facets keep their lines, their order and
`facet_limit`. In JSON the facet stays as an empty object `{}`, so the keys of `facets` are always those of
`output.facets`; the code already did this, the spec now says it and two examples pin it.

The spec change came first, in its own commit. No earlier spec example showed an empty facet line.

## Architecture

- `src/output/facets.rs`: `Facet::is_empty()`
- `src/output/mod.rs`: the text writer of `detail = facets` skips the empty facets
- the budget estimates the rendered text, so a facets response is estimated without the empty line; no change there

## Deviations from the plan

None.

## Tests

- examples E124.1–E124.4 in `tasks/124__EXAMPLES__EMPTY_FACET.md` (fixture `huge`): the empty query and `git` outside
  of a repository, in text and JSON
- unit test `no_line_for_a_facet_without_values` in `src/output/mod.rs`
- the examples of every earlier task pass unchanged

## Limitations

None known.

## How to try it

```json
{ "query": "", "output": { "detail": "facets" } }
```

answers with the `dir`, `lang` and `kind` lines and no `condition` line.
