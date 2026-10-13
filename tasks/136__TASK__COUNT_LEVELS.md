# Task 136: Counts per file and per line: `output.count`

Status: not implemented

Depends on: task 79, task 134

## Goal

An agent can ask how many matches there are at the grain it needs, and pay only for that:

| Grain      | Answers                                         | Like                       |
|------------|-------------------------------------------------|----------------------------|
| total      | how many files, lines, matches (today's `count`) | `grep -c \| sum`, `wc -l`  |
| per file   | how many lines and matches in every file         | `grep -c`                  |
| per line   | which lines match and how many matches each has  | `grep -n -o \| uniq -c`    |
| lines      | the matched lines themselves (today's `lines`)   | `grep -n`                  |

Found by trying the server: in the parallel runs of task 125 an agent wanted the matches per file and found that
`detail = count` returns only the totals. `detail = files` has the counts of every file, but with the conditions and
not under the name of a count; the line numbers of the matches without their text can't be asked for at all, though
they are what an agent needs to edit or to judge where the matches are.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md, TOOLS.md, the tool description), then the code.

- a new option `output.count = total | files | lines`, default `total`; a request option and a config key (`[output]
  count`, `update_config`, `queries` each with its own); it applies to `detail = count` (and to the `count` level the
  [budget](../OUTPUT.md#budget) picks), other details ignore it
  - `total`: the response of `detail = count` today, unchanged
  - `files`: the totals, then every matched file with its counts, grouped by directory like `files`
    (`output.group_dirs`), without the conditions: `lib.rs  lines: 3  matches: 4`
  - `lines`: as `files`, then under every file its matched lines as `42: 2` — the line number and the number of
    matches in it, without the text; a line of a `line( ... )` group with no fragment is `42: 0` or `42`, fixed in the
    spec step
- the names of the option and of its values are fixed in the spec step (e.g. `output.count_by = total | file |
  line`); the levels of `detail` stay as they are, so no request changes its meaning
- **every part stays optional** ([task 134](134__TASK__OUTPUT_OMIT.md), [OUTPUT.md#omit](../OUTPUT.md#omit)):
  `counts` leaves out the totals, `file_counts` the counts of a file (with `count = files` the paths alone are left:
  the same as `detail = files` with `omit = ["conditions"]`, the spec says so), `line_numbers` the line numbers of the
  per line counts; a new part for the count of a line (e.g. `line_counts`) leaves out `: 2`, so `count = lines` with it
  is the list of the matched line numbers. Every new piece of the response gets a part of `omit`, and `omit = "all"`
  removes it
- the JSON format: the counts of a file as in `files` detail, the lines as `"lines": [{ "n": 42, "matches": 2 }]`
- the limits: `max_files`, `max_lines`, `max_matches_per_file`, `max_output` and the budget cut the list as in `files` and
  `lines` detail; the totals are always the real ones; the cursor pages through the list
- `sort` orders the files as in `files` detail; `sort = matches desc` gives the files with the most matches first
- `within` and `since` work on these results like on every result with `count` detail or higher; `since` reports a
  file whose counts changed
- `queries`: every item with its own `count`

## Out of scope

- counts per directory, language, kind: `detail = facets`
- counts per condition inside a file
- new levels of `detail`

## Spec

- [OUTPUT.md#detail-levels](../OUTPUT.md#detail-levels), [OUTPUT.md#counts](../OUTPUT.md#counts),
  [OUTPUT.md#omit](../OUTPUT.md#omit), [OUTPUT.md#budget](../OUTPUT.md#budget),
  [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation)
- [CONFIG.md#output](../CONFIG.md#output)
- [TOOLS.md](../TOOLS.md): the `find` schema and description

## Examples

`136__EXAMPLES__COUNT_LEVELS.md` covers:

- the same query with `count = total`, `files`, `lines` and with `detail = lines`, in text and JSON
- `count = files` against `detail = files` with `omit = ["conditions"]`
- every part of `omit` which touches these responses, alone, and `omit = "all"`
- a `line( ... )` group and a multiline match in `count = lines`
- `group_dirs = false`, `sort = matches desc`
- the list cut by `max_files`, `max_lines` and the budget, with the real totals and the cursor
- `count` from the config, from `update_config` and per query of `queries`; `count` with `detail = files` ignored
- an unknown value

## Acceptance criteria

- the spec describes the option, the response of every value and its parts of `omit` before the code is written
- every example in `136__EXAMPLES__COUNT_LEVELS.md` passes as a test; the examples of all earlier tasks still pass
- every new piece of the response can be left out with `omit`
- the `find` tool description and schema name the option
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
