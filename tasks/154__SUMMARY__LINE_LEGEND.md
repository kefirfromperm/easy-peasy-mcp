# Summary of task 154: The legend of a `line(...)` without one entry per sub-condition

Server version: `0.153.0`.

## What was built

- spec 2: a written legend lists only the conditions whose ids the response writes
  (OUTPUT.md#the-legend-of-the-written-ids):
  - the ids of the conditions of the files, when the part `conditions` is written
  - the ids of the matches, when `match_ids` is written, the response shows matches (`lines` detail, path fragments)
    and the highlight writes ids (`markers`, `columns`, `auto`; not `none`); with `groups` the references of the
    shown matches; a sub-group is explained by its condition
  - the ids of the `condition` facet in `facets` detail (the values up to `facet_limit`)
  - every labelled condition (labels are out of scope and stay)
- a legend left with no entry is not written: `line(a or b)` with `highlight = "none"` has no legend at all, and a
  condition which only selects files (`lang = rust`) leaves the legend when nothing refers to it. The query of the
  task, `lang = rust and line(text has a or text has b)`, saves the `#1 lang = rust` line with markers and all four
  lines with `highlight = "none"`
- the decision is made over the whole result, like OUTPUT.md#the-same-everywhere: pages, `max_files` and the budget
  don't change it, and a line which `auto` writes without markers still counts its ids
- `show = ["legend"]` writes every condition; `"version": 1` is unchanged
- TOOLS.md#versioning names the change in the row of spec 2 (a change of a spec 2 rule, allowed since spec 2 is not
  used outside this repository)

## Architecture

- `src/output/omit.rs`: `Written` (what the response writes: the conditions, the ids of the matches, the ids of the
  facet) and the pure function `written_legend(legend, files, written)`, which collects the written ids over the
  files and keeps the legend entries in their order
- `src/engine.rs`: after the facets are counted and the omit is decided, in spec 2 when the rules write the legend and
  `show` doesn't bring it back, the filtered legend replaces `query.legend` in the `Response`; an empty one adds
  `Part::Legend` to the omit. The text, JSON and budget code is unchanged: it writes `Response::legend`

## Deviations from the plan

- no earlier example changed: the legends of the spec 2 examples of tasks 130–155 already list only written ids
  (their conditions are written, or the legend is left out by the rule of task 146)

## Tests

- examples E154.1–E154.7 in `tasks/154__EXAMPLES__LINE_LEGEND.md` on the fixture `omit` of task 134: `highlight =
  "none"` in text and JSON, markers, columns, JSON, `groups`, `near(...)`, a bracket group whose file conditions
  write every id, the `condition` facet and another facet, labelled conditions with and without a highlight,
  `show = ["legend"]`, `"version": 1`
- unit tests `the_legend_of_the_written_ids` and `labels_stay_and_sub_groups_bring_their_condition` in
  `src/output/omit.rs`
- all examples of the earlier tasks still pass

## Limitations

- the ids common to every match (the `#2` of `⟦#2,#3:…⟧` for `line(a or b)`) are still written and so stay in the
  legend; leaving them out of the markers would be a change of `match_ids`, not of the legend
- the OUTPUT.md examples written in the spec 1 style (OUTPUT.md#examples) keep their full legends

## How to try it

```sh
cargo build --release
```

Register `target/release/easy-peasy-mcp` as a stdio MCP server and call `find` with

```json
{ "query": "name ~ '*.rs' and line(text has 'unwrap(' or text has 'TODO')", "output": { "highlight": "none" } }
```

The response has no legend; without `highlight` it lists `#2`, `#3` and `#4`, not `#1 name ~ '*.rs'`.
