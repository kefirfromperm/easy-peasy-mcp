# Plan for task 153: A hint for `=` with a wildcard pattern

## Design

- `query/mod.rs` `variations`: a new variation for a `name =` / `path =` condition whose string value has a wildcard
  character (`*`, `?`, `[`, `{`): the condition rewritten with `~` for `=`, the modifiers and the value as written
  (`rewritten`, as the `has` instead of `=` variation). It comes after `has` instead of `=` (OUTPUT.md#hints, the
  order of the table). The hints already search a variation with the steps before it and report it only when it
  matches, with the names (`name`) or paths (`path`) of the files as examples, aligned with the funnel; a variation
  which doesn't parse (a modifier `~` doesn't take) is dropped by `hints::variation_hint`.
- `query/mod.rs` `lookup`: no similar names for `name =` with a wildcard character in the value, no similar
  directories for `path =` / `path starts` with one in the directory: a pattern isn't a misspelt name, and
  `no such directory` for `**` would mislead.
- a helper `has_wildcard(&str) -> bool` next to them.
- no new response part: the hint is a line of `hints`, left out with `omit = ["hints"]` like the others; JSON gets it
  in `hints` as every variation.

## Decisions

- the hint is written only when the `~` form matches: it costs one more search of the files of the steps before,
  inside the time budget of the hints, and an unhelpful hint would cost the agent tokens too
- the wording is the one of the other variations (the suggested condition and its files) rather than a sentence:
  shorter, and the agent sees the operator change by comparing with the funnel line above it
- spec version 1 gets the hint too: a new hint is not breaking (TOOLS.md#versioning), like a new warning
- `text =` gets no such variation: its value is text, not a pattern

## Tests

- examples E153.1–E153.8 (`tests/examples.rs`)
- unit tests in `query/mod.rs`: the variations of `name = '*.rs'`, `path = 'src/**' /i` (the modifier stays),
  `name = 'a.rs'` (no `~` variation), `name ~ '*.rs'` (none), `text = '*'` (none); `lookup` of `name = '*.rs'` and
  `path = '**/x.rs'` is none, of `path = 'src/x.rs'` still the directory

## Steps

1. red: the unit tests and the examples
2. green: `has_wildcard`, the variation, the lookup exception
3. summary, status, version 0.150.0

## Risks

- an existing example with `name =` / `path =` and a wildcard value would change: none found (searched the examples
  and the tests for such a condition)
