# Task 160: No huge-result hints for a small `max_files` of the request

Status: not implemented

Depends on: task 102, task 155

## Goal

A result is huge when it has more than 10 × `max_files` files (OUTPUT.md#limits-and-truncation), and the threshold
uses the `max_files` of the request. An agent who sets `max_files` small on purpose — to see one file of a search it
knows is broad — gets the funnel and the hints of a huge result it didn't need. Found in run 5 of the experiment
(EXPERIMENT.md, task 153, the easy-peasy arm), reproduced on 0.151.0:

```
find  query = "text has 'the'"  output = {"detail": "files", "max_files": 1, "highlight": "none"}

files: 815  lines: 13007  matches: 23231  result: r26  truncated: yes (max_files)  next cursor: eyJ2...
funnel:
  text has 'the'                       815
hints:
  dir: tasks/ 622 · tests/fixtures/ 120 · src/ 26 · ./ 12
  lang: markdown 640 · rust 146 · unknown 11 · javascript 3
  narrow with: not path ~ 'tasks/**', kind = source, /w

tasks/
152__PLAN__MATCHES_EQUAL_LINES.md  lines: 25  matches: 50
```

With the default `max_files` (20) the threshold is 200 files, and 815 would still be huge; but a result of 15 files
with `max_files = 1` is huge too, where the default would show it whole without hints. The funnel and the hints are
~70 tokens per such response, plus the time the server spends on them (at most 20% of the timeout).

## Scope

A spec change first (its own commit; OUTPUT.md#limits-and-truncation, OUTPUT.md#funnel), then the code.

- the spec step decides the threshold: 10 × the configured `output.max_files` (the default or the server config,
  not the request), the larger of the configured and the requested one, or no huge-result hints when the request
  sets `max_files` below the configured one
- `output.funnel = "always"` still writes the funnel; the hints of an empty result are unchanged
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- the content of the huge-result hints (`dir:`, `lang:`, `narrow with:`)
- a larger `max_files` of the request (a threshold above the configured one)

## Examples

`160__EXAMPLES__HUGE_HINTS_SMALL_MAX_FILES.md` covers:

- a broad search with `max_files = 1`: no hints when the result isn't huge by the chosen threshold
- the same result huge by any threshold: the hints stay
- `output.funnel = "always"` with a small `max_files`
- `max_files` set in the server config

## Acceptance criteria

- the spec states the threshold before the code is written
- every example in `160__EXAMPLES__HUGE_HINTS_SMALL_MAX_FILES.md` passes as a test; the examples of all earlier tasks
  still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
