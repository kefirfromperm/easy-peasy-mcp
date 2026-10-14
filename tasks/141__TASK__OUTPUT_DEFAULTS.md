# Task 141: Useful output by default: the default of `output.omit`

Status: not implemented

Depends on: task 134

## Goal

An agent which sends a plain `find` gets the parts of the response it uses and pays nothing for the others, without
having to know `output.omit`. A part which is rarely useful is left out by default; an agent asks for it when it needs
it, and a user who wants other defaults sets them in the configuration.

Found by using the server: in the searches of task 129 the responses carried parts the agent never read — the
funnel and 28 `binary_skipped` / `decode_errors` warnings of test fixtures on a search with no match, the legend and
the conditions of every file on a search with one condition. Each was avoidable with `output.omit`, but only by an
agent which remembers to list them on every request; by default every part is shown (`omit = []`).

## Scope

A review first, then a spec change (its own commit; OUTPUT.md, CONFIG.md, TOOLS.md, the tool description), then the
code.

### Review

Every part of [OUTPUT.md#omit](../OUTPUT.md#omit) (`counts`, `result`, `legend`, `funnel`, `hints`, `conditions`,
`file_counts`, `match_ids`, `groups`, `line_numbers`, `gaps`, `indent`, `warnings`, and the parts added by the
tasks done before this one) gets a row in a table of `141__PLAN__OUTPUT_DEFAULTS.md`:

- what the agent does with it, and in which kind of search (one condition or several, a hit or no hit, `files` or
  `lines`, text or JSON)
- what it costs: its tokens in the examples and in the searches of `EASY_PEASY_STATS.local.md`
- how the agent gets it back when it is omitted, and how bad it is to miss it (e.g. a `timeout` warning hidden makes
  a partial result look complete — that must never be omitted silently)
- the decision: shown by default, omitted by default, or shown only when it carries information (e.g. the legend only
  with more than one condition, the warnings only of codes about the result itself, the funnel only for an empty
  result — today `funnel = auto`)

The review also looks at the output options which decide what a part looks like (`highlight`, `dedent`, `funnel`,
`group_repeated`, `max_line_length`) and names any default worth changing in the same spec change.

### Defaults

- the built-in default of `[output] omit` becomes the list of the review; the config file, the command line and
  `update_config` change it as today; `get_config` shows it with its source
- a part "shown when it carries information" is a rule of the part in OUTPUT.md, not an entry of the list
- a part omitted by default must be easy to get back in one request: a request `omit` replaces the configured list
  (CONFIG.md: lists are replaced, not merged), so `omit = ["legend"]` would bring back every default-omitted part.
  The spec step fixes this, e.g. with a new `output.show = [...]` which adds parts back on top of the configured
  `omit`, or with `omit` entries like `"+funnel"` / `"-funnel"`; the choice and the alternatives rejected go in the
  plan
- a response says when a part with information was left out, where missing it would mislead: at least the warnings
  (`warnings: 28 omitted` or a hint how to see them); the review decides which parts need such a mark, and the mark has
  its own part of `omit` ([every part is optional](134__TASK__OUTPUT_OMIT.md))
- the `find` description shows the default output and how to ask for more

### Spec version

A changed default which changes the text layout is a breaking change ([TOOLS.md#versioning](../TOOLS.md#versioning)).
The spec step decides between:

- spec version 2 with the new defaults, `"version": 1` keeping the old ones (`supported specs: 1, 2`), or
- keeping spec 1 by arguing why a default of `omit` is not a layout change, with the versioning table changed in the
  same commit

The examples of the earlier tasks stay valid: they either pin the old defaults (`version` 1 or a `config` block with
`omit = []`), or the spec says why they change.

## Out of scope

- the responses of the write tools (`delete`, `rename`, `copy`, `replace`): they have no `omit`; a separate task if the
  review finds parts there worth hiding
- new parts of the response, other than the marks of omitted information
- the budget's choice of the detail level

## Spec

- [OUTPUT.md#omit](../OUTPUT.md#omit), [OUTPUT.md#warnings](../OUTPUT.md#warnings),
  [OUTPUT.md#funnel](../OUTPUT.md#funnel)
- [CONFIG.md#output](../CONFIG.md#output)
- [TOOLS.md#versioning](../TOOLS.md#versioning), the `find` schema and description

## Examples

`141__EXAMPLES__OUTPUT_DEFAULTS.md` covers:

- the same searches with the old and the new defaults: one condition and several, a hit and no hit, `files` and
  `lines`, text and JSON
- a default-omitted part asked for in one request, without losing the other defaults
- the defaults changed in the config file, on the command line and by `update_config`; `get_config` showing them
- a mark of omitted information (warnings), and the mark itself omitted
- a `timeout` or another warning which must stay visible
- `queries` items with their own `omit`
- the spec version decision: the old defaults with `version` 1, or the reason they aren't needed

## Acceptance criteria

- the review table is in the plan, with a decision for every part, before the spec is changed
- the spec describes the new defaults, how to get a part back and the spec version before the code is written
- every example in `141__EXAMPLES__OUTPUT_DEFAULTS.md` passes as a test; the examples of all earlier tasks still pass
- a plain `find` on the searches of the review costs fewer tokens than before, measured in the plan or the summary
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
