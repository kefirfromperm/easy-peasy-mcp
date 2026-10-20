# Task 153: A hint for `=` with a wildcard pattern

Status: in progress

Depends on: task 103

## Goal

`name = '...'` and `path = '...'` compare the whole name, so a value with wildcard characters (`*`, `?`, `[...]`,
`{a,b}`) matches only a file named with those characters. An agent who writes `=` for `~` gets an empty result with a
funnel and no hint, and has to spot the mistake itself.

Found in run 2 of the experiment (EXPERIMENT.md, task 150, the easy-peasy arm), reproduced on 0.146.0:

```
find  query = "name = '149__TASK__*'"  dirs = ["tasks"]  output.detail = "files"

files: 0  lines: 0  matches: 0  result: r19
funnel:
  name = '149__TASK__*'                  0
```

`name ~ '149__TASK__*'` finds the file. A hint such as `hint: name = '149__TASK__*' compares the whole name; for a
wildcard pattern write name ~ '149__TASK__*'` saves the agent a call: about 100 tokens of a second request and
response, and the guessing.

## Scope

A spec change first (its own commit; OUTPUT.md#hints, QUERY.md where `=` and `~` of `name` and `path` are described).
The spec step decides:

- when the hint is written: an empty result (or an empty step of the funnel) caused by a `name =` / `path =`
  condition whose value contains wildcard characters; whether only when the `~` form would match (costs a second
  match over the same files) or always
- `/i` and the other modifiers carried over to the suggested condition
- `text =` is out: its value is text, not a pattern
- the hint's wording and its place among the hints of task 103; JSON (`hints` entry)

## Out of scope

- `~` with a value that has no wildcard (works like `=`)
- other kinds of near misses (case, extension), already covered by the hints of task 103 where they are

## Spec

- [OUTPUT.md#hints](../OUTPUT.md#hints), [QUERY.md](../QUERY.md)

## Examples

`153__EXAMPLES__EQUALS_WITH_WILDCARDS.md` covers:

- `name = '*.rs'` and `path = 'src/**'` with an empty result: the hint
- `name = '*.rs'` in a fixture where a file is really named `*.rs`: what is written
- a hint inside a larger query (`... and name = '*.md'`), `omit = ["hints"]`, JSON, `"version": 1`

## Acceptance criteria

- the spec says when the hint is written and its text, before the code is written
- every example in `153__EXAMPLES__EQUALS_WITH_WILDCARDS.md` passes as a test; the examples of all earlier tasks still
  pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
