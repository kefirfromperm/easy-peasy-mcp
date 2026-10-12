# Task 124: No line for a facet without values

Status: in progress

Depends on: task 105

## Goal

The `facets` output has no empty lines which look like a broken response and cost tokens.

Found by trying the server: an empty query with `detail = facets` prints a line `condition:  ` with nothing after it,
because an empty query has no conditions. The same happens to `git` outside of a repository and to any facet whose
files all lack the value. The spec ([OUTPUT.md#facets](../OUTPUT.md#facets)) says that a value with no files is not
listed, but not what happens to a facet with no values at all.

## Scope

A spec change first (its own commit), then the code:

- text format: a facet with no values has no line
- JSON format: the facet stays as an empty object `{}`, so a client can rely on the keys of `output.facets`
- `facet_limit` and the order of the other facets unchanged

## Out of scope

- new facets or values

## Spec

- [OUTPUT.md#facets](../OUTPUT.md#facets)

## Examples

`124__EXAMPLES__EMPTY_FACET.md` covers:

- an empty query with the default facets: no `condition` line
- `facets = ["git"]` outside of a repository
- the same two in the JSON format

## Acceptance criteria

- the spec states the rule before the code is written; spec examples which show an empty facet line are fixed
- every example in `124__EXAMPLES__EMPTY_FACET.md` passes as a test; the examples of all earlier tasks still pass
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
