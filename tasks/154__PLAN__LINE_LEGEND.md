# Plan for task 154: The legend of a `line(...)` without one entry per sub-condition

## Design

- spec (done, its own commit): OUTPUT.md#the-legend-of-the-written-ids — in spec 2 a written legend lists only the
  conditions whose ids the response writes, plus every labelled condition; a legend with no entry is not written;
  `show = ["legend"]` writes every condition; spec 1 unchanged. TOOLS.md#versioning names the change in the row of
  spec 2.
- `output/omit.rs`: a pure function `written_legend(legend, files, written) -> Vec<(CondId, String)>` with a small
  `Written` struct of what the response writes: `conditions` (the part `conditions` is written), `match_ids` (the
  part `match_ids` is written, the response shows matches and the highlight is not `none`), and the ids of the
  `condition` facet (the values up to `facet_limit`). It collects the ids:
  - the `conditions` of every file of the result when `conditions`
  - the ids of every content fragment and path fragment when `match_ids`; a `CondId::Sub` (`@def.fname`) is
    explained by its condition: the legend id whose text is the reference before its `.`
  - the ids of the facet
  and keeps the legend entries in their order whose id is collected or which are `CondId::Label`.
- `engine.rs` (`respond`): after the facets are counted and before `shown_files` takes the files, in spec 2 when the
  legend is written by the rules (not in `omit`) and not brought back by `show`, the filtered legend replaces
  `query.legend` in the `Response`; an empty one adds `Part::Legend` to the omit. The highlight comes from
  `highlight(settings, json)`; `Highlight::None` means no ids of matches.
- the text and JSON writers already write `Response::legend`, so they need no change; the budget estimates the
  response with the shorter legend.

## Decisions

- the decision is made over the whole result (all matched files), like OUTPUT.md#the-same-everywhere: pages and
  `max_files` don't change the legend; a line which `auto` writes without markers still counts its ids — deciding per
  written line would tie the legend to the cut of the budget and of the pages (rejected)
- labelled conditions stay, as the task leaves labels out of scope; they keep a legend which has no other entry
  (E154.5)
- `show = ["legend"]` writes the full legend, as in spec 1: an explicit request of the legend is a request of all
  of it (rejected: a filtered legend with `show`)
- no new omit part: the change only shortens the existing `legend` part

## Tests

- examples E154.1–E154.7 (tests/examples.rs reads them)
- unit tests in `output/omit.rs`: `written_legend` keeps the ids of the conditions, of the matches, of the facet,
  the labels, the condition of a sub-group, and gives nothing when nothing is written
- earlier spec 2 examples whose legend lists a condition written nowhere change; each is fixed in the examples file
  with the reason in the commit

## Steps

1. red: the unit tests of `written_legend` with a stub, the examples
2. green: `written_legend` and its use in `engine.rs`
3. fix the earlier spec 2 examples that change, check that only the legend lines differ
4. summary, status, version `0.153.0`

## Risks

- the budget and the batch build `Response` in several places: the filtered legend must reach every one (the
  engine builds one `Response` per query, so one place)
- earlier examples (tasks 130, 135, 141–155) may have legends with file-only conditions: a wide but mechanical fix
