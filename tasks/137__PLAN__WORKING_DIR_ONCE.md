# Plan for task 137: an absolute working directory written once

## Design

The spec change is committed already
([OUTPUT.md#the-working-directory-once](../OUTPUT.md#the-working-directory-once)): a new value `path_style = in_dir`,
the `in:` line, JSON `dirs` / `dir`, the omit part `dir`.

### Spec version

`in:` changes the text layout, which is breaking for a changed default (TOOLS.md#versioning). A new value of an option
is not: `relative` stays the default of spec 1 and every earlier example keeps its output. Task 141 decides the
defaults.

### Where the working directory is taken out

The search joins the working directory to the path of every file (`Found::rel`) and the paths of the warnings; the
result cache, `since`, `within` and the write tools rely on it. They stay as they are; `in_dir` takes the working
directory out at the end, in `Engine::respond`:

- after the result is kept and `since` made its differences (the removed files of `since` get the index of their
  working directory, `Found::root`)
- `rel` of every file stripped of the prefix of its working directory; its path fragments shifted back
  (`PathFragments::unshown`, the inverse of `shown`)
- one working directory: the warnings stripped too (`Warnings::strip_prefix`), the `dir` facet and the `dir:` hint of
  a huge result computed on the stripped paths (`facets::facets` and the hints take the prefix to strip)
- several working directories: the warnings, the facets, the hints and the locations of the repeated lines keep the
  working directory, the locations join it back

### The response

- `Response::dirs`: `Some(names)` with `in_dir`, the working directories as written (`display_path` + separator, `./`
  for `.`)
- one working directory other than `.`: `in: <dir>` after the header line in the responses with a path (the files
  are listed, facets, or warnings with paths)
- several: `push_file` writes `in: <dir>` when the root differs from the previous file's and resets the directory
  header
- JSON: `dirs` after `result`, `dir` of a file with several working directories
- `Part::Dir` after `Result`

## Rejected

- an empty prefix in the working directories of the request: the cache and `since` would store paths without the
  directory, `within` and the write tools would lose it, and a prefix stripped twice would hit a path which starts
  with the same directory name
- ordering the files by working directory: the order of `sort` is the contract, the `in:` lines follow it like the
  directory headers

## Tests

- E137.1–E137.8
- unit tests: `PathFragments::unshown` against `shown`, the text with several working directories, `Part::Dir`
- earlier examples which change: the list of the omit parts in the errors, the schema of `path_style` and `omit`

## Steps

1. red: the unit tests
2. `in_dir`, `Part::Dir`, the strip in the engine, the response in text and JSON
3. schema
4. fix the earlier examples
5. summary, status `implemented`, minor version bump `0.132.0` → `0.133.0`

## Risks

- the facets and the hints computed on paths without the working directory: only with one working directory, where
  they are unambiguous
