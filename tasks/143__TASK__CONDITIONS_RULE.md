# Task 143: The legend rule counts the conditions an agent wrote

Status: not implemented

Depends on: task 141

## Goal

The rule of spec 2 which leaves out the legend, the conditions of the files and the ids of the matches for a query
with one condition ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)) also applies to a query with one
`line( ... )` or `near( ... )` group around one condition. Today the rule counts the entries of the legend, and a group
has an entry of its own besides the condition inside it, so `line(text = 'x')` gets the legend and `⟦#1,#2:x⟧` in
every match, though the agent wrote one condition and the ids repeat the request:

```
#1 line(text = 'Spec: `1`')
#2 text = 'Spec: `1`'

repeated:
  ×100 ⟦#1,#2:Spec: `1`⟧    …
```

Found by trying the server: `line(text = 'Spec: `1`')` answered with a legend of two entries and two ids in every
match.

## Scope

A spec change first (its own commit; OUTPUT.md), then the code.

- the spec step fixes what the rule counts, e.g. the conditions which can tell matches or files apart: a group with
  one condition inside (`line( ... )`, `near( ... )` with one side, a bracket `( ... ) @name` around one condition) is
  one; `near(a, b)` is two; whether a condition under `not`, which produces no ids, counts is decided in the spec
  step; a label `@name` given by the agent keeps the legend (the agent asked for the name)
- the legend, the conditions and the ids follow the rule together, as in task 141; `show` brings them back
- spec 1 is unchanged; the rule is a part of spec 2: a change of it is decided by
  [TOOLS.md#versioning](../TOOLS.md#versioning) (spec 2 has not been used by agents outside this repository yet)

## Out of scope

- the other rules and defaults of spec 2
- the text of the legend

## Spec

- [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2), [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids)

## Examples

`143__EXAMPLES__CONDITIONS_RULE.md` covers:

- `line(text has 'x')`, `near(text has 'a', text has 'b', 2)`, `(text has 'x') @x`, `text has 'x' and not text has 'y'`,
  `name ~ '*.rs' and line(text has 'x')`, text and JSON
- `show = ["legend"]` with such a query
- the same queries with `"version": 1`, unchanged

## Acceptance criteria

- the spec says what the rule counts before the code is written
- every example in `143__EXAMPLES__CONDITIONS_RULE.md` passes as a test; the examples of all earlier tasks still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
