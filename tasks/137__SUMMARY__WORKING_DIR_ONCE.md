# Summary of task 137: an absolute working directory written once

Server version: `0.133.0`.

## What was built

[OUTPUT.md#the-working-directory-once](../OUTPUT.md#the-working-directory-once): `path_style = in_dir` writes the
working directory once and the paths relative to it, so a search in an absolute working directory (a git worktree
outside the default directory) costs no more than one in `.`:

```
files: 2  lines: 3  matches: 3  result: r2
in: <fixture>/proj/
#1 text has 'cache('

src/
cache.rs  [#1]  lines: 2  matches: 2
...

warning: binary_skipped (1): data.bin
```

- one working directory: the `in:` line after the header, in the responses which write a path (listed files, facets,
  warnings about entries); the files, the directory headers, the repeated lines, the warnings, the `dir` facet and
  the hints are relative to it; `.` has no line
- several working directories: an `in:` line before the files of each, written whenever the working directory
  changes, like a directory header (the order of `sort` stays); the warnings, the facets, the hints and the
  locations of the repeated lines keep the working directory
- JSON: `dirs` after `result`, `dir` (the index in `dirs`) of every file with several working directories
- `within` and `since` work as with `relative`; `omit = ["dir"]` leaves out the lines and keeps the short paths
- the hint `narrow with: not path ~ '<dir>/**'` of a huge result now names a directory relative to the working
  directory with `in_dir`, as the `path` condition sees it

### Spec version

The `in:` line changes the text layout, a breaking change for a default (TOOLS.md#versioning). `in_dir` is a new value
of `path_style`, which is not breaking; the default stays `relative` and no earlier example changed its response.
Task 141 decides whether a later spec version makes it the default.

## Architecture

- `engine.rs`: `respond` keeps the result first, then `in_dirs` / `strip_in_dirs` take the working directory out of
  the paths (`strip_working_dirs`) and, with one working directory, of the warnings before the hints and the facets
  are made; with several only before the response. `differences` (`since`) compares the full paths and gives the
  removed files the index of their working directory
- `search/mod.rs`: `PathFragments::unshown`, the inverse of `shown`
- `warning.rs`: `Warnings::strip_prefix`, `has_paths`
- `output/mod.rs`: `Response::in_dirs`; `in_line` (one), `dir_line` (several), `location_path` for the repeated
  lines; `output/json.rs`: `dirs`, `dir`
- `output/omit.rs`: `Part::Dir` after `Result`

## Deviations from the plan

None.

## Tests

- E137.1–E137.8 in `tasks/137__EXAMPLES__WORKING_DIR_ONCE.md` (fixture `in-dir`)
- unit tests: `PathFragments::unshown`, `Part::Dir`
- earlier examples changed: the schema of `path_style` and `omit` (E117.2), the values of `path_style` in its errors
  (E90.6, E90.7), the list of the omit parts (E134.9, E136.12)

## Limitations

- several working directories: the warnings, the facets and the hints keep the long paths
- not the default of spec 1, see above

## How to try it

```json
{ "dirs": ["/abs/worktree"], "query": "text has 'TODO'", "output": { "path_style": "in_dir" } }
```
