# Task 154: The legend of a `line(...)` without one entry per sub-condition

Status: implemented, see [154__SUMMARY__LINE_LEGEND.md](154__SUMMARY__LINE_LEGEND.md)

Depends on: task 143, task 146

## Goal

The legend lists every condition with its id, and the conditions inside a `line(...)` get ids of their own. A query
with one `line(a or b)` pays four legend entries, although the matched lines already show the text that matched and
the agent wrote the query a moment ago.

Found in run 2 of the experiment (EXPERIMENT.md, task 150), and reproduced on 0.146.0 while reviewing it:

```
find  query = "lang = rust and line(text has '\"left_out\"' or text has 'left_out\"')"  dirs = ["src"]

files: 2  lines: 5  matches: 9  result: r18
#1 lang = rust
#2 line(text has '"left_out"' or text has 'left_out"')
#3 text has '"left_out"'
#4 text has 'left_out"'
...
```

The four lines cost about 35 tokens of a ~200-token response. The easy-peasy arm of the run noted the same on every
response with a `line(...)` group: the sub-conditions #3–#6 "add a few lines to every response".

## Scope

A spec change first (its own commit; OUTPUT.md#condition-ids and the legend rule of task 143). The spec step decides:

- whether the sub-conditions of a `line(...)` / `near(...)` / bracket group are listed when no match id refers to
  them in the response (`highlight = "none"`, match ids omitted or the same everywhere, task 146)
- whether a condition that only selects files (`lang = rust`) is listed when nothing in the response refers to it
- the effect on `[#3 #4]` match ids and `⟦ids:text⟧` when they are shown: the legend keeps every id that is written
- spec 1 unchanged; a change of the spec 2 output follows [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- labels (`@name`): a labelled condition stays in the legend
- the `conditions` part and the funnel

## Spec

- [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids), [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)

## Examples

`154__EXAMPLES__LINE_LEGEND.md` covers:

- `line(a or b)` with `highlight = "none"`: the legend after the rule
- the same with markers or match ids shown: the ids that are written stay in the legend
- `near(...)`, a bracket group, a labelled sub-condition, `show = ["legend"]`, `"version": 1`

## Acceptance criteria

- the spec says which conditions the legend lists, before the code is written
- every example in `154__EXAMPLES__LINE_LEGEND.md` passes as a test; the examples of all earlier tasks still pass (a
  spec 2 example which changes is fixed with the reason in its commit)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
