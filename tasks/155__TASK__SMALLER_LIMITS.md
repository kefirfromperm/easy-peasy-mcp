# Task 155: Smaller default output limits

Status: in progress

Depends on: task 101, task 141

## Goal

The default output limits let one over-broad query cost thousands of tokens before any limit is hit. In run 2 of the
experiment (EXPERIMENT.md, task 150) two queries of the easy-peasy arm cost ~21K of its ~33K characters of search
output: `text has 'show'` (also `shown`, needs `/w`) and `name ~ '1[34]?__…'` (`?` is any character). Neither came
near a limit, so both were returned in full:

```
get_config section = "output"   (0.146.0)

max_files = 100
max_lines = 500
max_matches = 1000
max_matches_per_file = 50
max_line_length = 300
max_output_per_file = 4000
max_output = 40000
```

`max_output = 40000` characters is ~10K tokens for one response. Lower defaults, 5 times each, cut such a response
early with `truncated: yes (...)`, the funnel and the hints, and the agent narrows the query or pages on with
`output.cursor` when it really needs more:

| Limit                  | Now     | New    |
|------------------------|---------|--------|
| `max_files`            | `100`   | `20`   |
| `max_lines`            | `500`   | `100`  |
| `max_matches`          | `1000`  | `200`  |
| `max_matches_per_file` | `50`    | `10`   |
| `max_output_per_file`  | `4000`  | `800`  |
| `max_output`           | `40000` | `8000` |

The saving is up to ~32K characters (~8K tokens) on a response that hits `max_output` now, and nothing on a response
that stays below the new limits.

## Scope

A spec change first (its own commit):

- the new defaults in OUTPUT.md (the output options table, [Limits and truncation](../OUTPUT.md#limits-and-truncation))
  and CONFIG.md (the sample config, the defaults table, the note that the default `max_output` is about 10–13 thousand
  tokens)
- versioning: a changed default which changes the results is breaking (TOOLS.md#versioning). The spec step decides
  whether the new limits are part of the [defaults of spec 2](../OUTPUT.md#defaults-of-spec-2), as the `omit` default of
  task 141, with `"version": 1` keeping the old limits, or need a new spec version
- the limits of [output to a file](../OUTPUT.md#output-to-a-file) and of a [batch](../OUTPUT.md#batch) (`max_output`
  split between the queries) with the smaller default
- the hints and the funnel of a truncated response: say how to get more (`output.cursor`, a higher limit, a narrower
  query), checked against the new, more frequent truncation

## Out of scope

- `max_line_length` stays `300`: 60 would cut most lines of code
- `budget` (still not set by default), `max_output_file_size`, `facet_limit`
- a configured or requested limit: only the built-in defaults change

## Spec

- [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation), [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)
- [CONFIG.md](../CONFIG.md) (the `[output]` section and its defaults)

## Examples

`155__EXAMPLES__SMALLER_LIMITS.md` covers:

- a query over more than 20 files, more than 10 matches in one file, more than 100 lines: the truncated response with
  the new defaults, its `next_cursor` and hints
- the same request with the limits raised in the request and in the configuration
- `"version": 1` (if the spec step keeps the old limits for it), a batch, `output.file`
- `get_config` showing the new defaults

## Acceptance criteria

- the spec states the new defaults and the versioning decision before the code is written
- every example in `155__EXAMPLES__SMALLER_LIMITS.md` passes as a test; the examples of all earlier tasks still pass
  (an example which changes because it relied on the old defaults is fixed with the reason in its commit, or sets the
  old limit explicitly)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
