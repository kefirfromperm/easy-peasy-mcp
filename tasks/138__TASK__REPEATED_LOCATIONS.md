# Task 138: All the locations of a repeated line

Status: implemented, see [138__SUMMARY__REPEATED_LOCATIONS.md](138__SUMMARY__REPEATED_LOCATIONS.md)

Depends on: task 86, task 134

## Goal

An agent which needs every place of a line repeated in many files (to edit them all) gets them in one response.
Today a [repeated line](../OUTPUT.md#repeated-lines) is written with the first three locations and `… N more`; the
other locations can't be asked for: `group_repeated = 0` writes the line again under every file, which costs the
tokens the grouping saves.

```
repeated:
  ×10 "allow_update = true",    tasks/101__EXAMPLES__X.md:12, tasks/102__EXAMPLES__Y.md:30, … 8 more
```

Found by trying the server: in the parallel runs of task 125 an agent had to add `allow_write` next to the same line
in about 68 example files and could not list them with `find`.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md, TOOLS.md), then the code.

- a new option `output.repeated_locations`: the number of locations written for a repeated line, default `3` (today's
  output unchanged); `"all"` (or `0`, fixed in the spec step) writes all of them; a request option and a config key
  (`[output] repeated_locations`, `update_config`, `queries` each with its own)
- the text format: the locations after the line as today, `, … M more` only when some are left out; with many
  locations the form (one line, or one location per line under the line) is fixed in the spec step
- JSON: `locations` has as many entries as the option says; `count` stays the number of all the files
- the limits: the locations count in `max_output` and the budget; when they are cut the response is truncated as
  usual and the cursor continues
- **every part of the output is optional** ([task 134](134__TASK__OUTPUT_OMIT.md)): a new part of
  [`omit`](../OUTPUT.md#omit) (e.g. `locations`) leaves the locations out (the line and `×N` stay), and `omit =
  "all"` removes them

## Out of scope

- grouping lines which differ (by whitespace inside, by case)
- the repeated lines in other details than `lines`

## Spec

- [OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines), [OUTPUT.md#omit](../OUTPUT.md#omit),
  [OUTPUT.md#budget](../OUTPUT.md#budget)
- [CONFIG.md#output](../CONFIG.md#output)
- [TOOLS.md](../TOOLS.md): the `find` schema

## Examples

`138__EXAMPLES__REPEATED_LOCATIONS.md` covers:

- the default (3 locations), a number, all of them, in text and JSON
- the locations left out with `omit`, and with `omit = "all"`
- all the locations cut by `max_output` and the budget, and the cursor
- the option from the config, from `update_config` and per query of `queries`
- an invalid value

## Acceptance criteria

- the spec describes the option and its part of `omit` before the code is written
- every example in `138__EXAMPLES__REPEATED_LOCATIONS.md` passes as a test; the examples of all earlier tasks still
  pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
