# Summary of task 143: the legend rule counts the conditions an agent wrote

Server version: `0.140.0`.

## What was built

- The spec 2 rule of the legend, the conditions of the files and the ids of the matches
  ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)) counts the
  [counted conditions](../OUTPUT.md#counted-conditions) of the query instead of the entries of the legend:
  a simple condition one; `line( ... )`, `near( ... )` and a labelled bracket what is inside them, at least one;
  a condition under `not` nothing; `and` / `or` / `xor` / plain brackets the sum of their parts.
- `line(text = 'x')`, `line(not text starts '#')`, `(text has 'x') @x` and `text has 'x' and not text has 'y'` now
  answer without the legend and without `[#1 #2]` / `⟦#1,#2:` in every file and match; `near(a, b, n)`,
  `line(a and b)` and `name ~ '*.rs' and line(text has 'x')` keep them. `show = ["legend"]` brings them back, spec 1
  is unchanged.
- Spec (own commits): the rule and the new subsection `Counted conditions`. A change of a spec 2 rule, allowed since
  spec 2 has not been used outside this repository (TOOLS.md#versioning); no earlier example changed.

## Architecture

- `query::ast::Expr::counted()` counts the conditions recursively.
- `Query::many_conditions` (set by `query::parse`): more than one counted condition.
- `output::omit::Omit::effective` takes this flag instead of the number of legend entries; with `output.groups` the
  ids of the matches still follow the number of its references. `engine.rs` passes the flag.

## Deviations from the task and the plan

- The task said a label `@name` keeps the legend. The examples of task 130 (spec 2) use one labelled regex in
  `output.groups` and show no legend, and such a label carries nothing: its id is the same in every file and match.
  So a label adds nothing to the count (second spec commit); `Expr::has_label` of the plan was dropped.
- E143.5 was fixed before the code: `name ~ '*.rs'` reads no binary file, so no warning is left out.

## Tests

- examples E143.1–E143.8 (fixture `omit` of task 134): a line group, `near`, labels, `not`, a file condition with a
  line group, JSON, `show`, spec 1
- unit `query::tests::the_counted_conditions`; `omit` tests adapted to the flag
- all earlier examples unchanged

## Limitations and known gaps

- `cargo clippy` of the system toolchain 1.99 (the pinned 1.98 is not installed here) still reports its
  `assert_is_empty` lint on the asserts of the earlier tests, as before this task; clean with that lint allowed.

## Try it

`{"query": "line(text = 'Spec: `1`')", "dirs": ["tasks"]}` — no legend, no ids in the repeated line.
