# Task 134: Every part of the output optional: `output.omit`

Status: not implemented

Depends on: task 105, task 108, task 113, task 123

## Goal

An agent can leave out any part of a response it doesn't need, so it pays tokens only for what it reads. Today some
parts can be turned off (`funnel = never`, `highlight = none`, `group_dirs = false`, `meta = false`,
`group_repeated = 0`), others can't: the legend, the condition ids of the files, the per file counts, the line
numbers, the `--` gaps, the `⇥N` lines, the warnings. For a one-condition query the legend `#1 text has 'x'` repeats
the request and every `[#1]` says nothing.

Found by trying the server: three searches of `detail = lines` cost 280 tokens against 120 for the equivalent `git
grep`, most of the difference the legend and the funnel.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md, TOOLS.md, the tool description), then the code.

- a new option `output.omit`: a list of the parts to leave out, default `[]`; a request option and a config key
  (`[output] omit`, `update_config`, `queries` each with its own), merged like the other lists of `output`
- the parts, each with the text and the JSON it removes:

  | Part              | Text                                              | JSON                                  |
  |-------------------|---------------------------------------------------|---------------------------------------|
  | `counts`          | `files: N  lines: N  matches: N` of the header    | `counts`, `counts_exact`              |
  | `result`          | `result: r1`                                      | `result`                              |
  | `legend`          | the legend lines                                  | `legend`                              |
  | `funnel`          | `funnel:` and its steps                           | `funnel`                              |
  | `hints`           | `hints:` and its lines                            | `hints`                               |
  | `conditions`      | `[#1 #2]` after a path                            | the conditions of a file              |
  | `file_counts`     | `lines: N  matches: N` after a path               | the counts of a file                  |
  | `match_ids`       | `#1:` inside a marker: `⟦unwrap(⟧`                | the ids of a fragment                 |
  | `line_numbers`    | `42:` / `41-` before a line                       | `line` of a line                      |
  | `gaps`            | `--` between blocks                               | —                                     |
  | `indent`          | the `⇥N` line (the block stays dedented)          | `indent`                              |
  | `warnings`        | the `warning:` lines                              | `warnings`                            |

  the final names and the JSON fields are fixed in the spec step; every part of OUTPUT.md which can be left out
  without breaking the format gets a name, the ones above at least
- `omit = "all"`: every part of the list, the bare grep-like output; a string other than `"all"` is
  `invalid_value`
- `line_numbers` can be left out with context lines: a context line and a matched line may then look the same, which
  the agent asked for; the spec says so
- an unknown part is `invalid_value` with the list of the parts as the hint
- the existing switches keep working; `omit = ["funnel"]` and `funnel = never` are the same
- the budget estimates the response without the omitted parts

## Out of scope

- `version` and `exists` of JSON: the frame of the response, always written
- the truncation: `truncated: yes (...)` and `next cursor: ...` (JSON `truncated`, `truncated_by`, `next_cursor`) are
  always written when something was cut, also with `omit = "all"`, so an incomplete result never looks complete
- the paths of the files and the matched lines themselves: what the response is for (use `detail`)
- reordering parts or new short forms of the header

## Spec

- [OUTPUT.md#options](../OUTPUT.md#options), [OUTPUT.md#response](../OUTPUT.md#response), [OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids), [OUTPUT.md#budget](../OUTPUT.md#budget)
- [CONFIG.md#output](../CONFIG.md#output)
- [TOOLS.md](../TOOLS.md): the `find` schema

## Examples

`134__EXAMPLES__OUTPUT_OMIT.md` covers:

- every part left out alone, in text and JSON
- `omit = "all"`, and the truncation kept with it
- `line_numbers` left out with context lines
- the bare output: everything which can be left out, against the default response of the same query
- `omit` from the config, from `update_config` and per query of `queries`
- `omit` with the budget: more fits
- an unknown part

## Acceptance criteria

- the spec lists every part and what leaving it out removes, before the code is written
- every example in `134__EXAMPLES__OUTPUT_OMIT.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description and schema name `output.omit`
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
