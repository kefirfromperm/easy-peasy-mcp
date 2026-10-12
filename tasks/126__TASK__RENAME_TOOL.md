# Task 126: `rename` tool

Status: not implemented

Depends on: task 125

## Goal

An agent can rename or move the files it found in one call: give every file of a result a new name or a new
directory, by a pattern instead of one call per file.

## Scope

A spec change first (its own commit), then the code. Enabling, selecting files, `expect`, `dry_run`, the response and
the errors are the shared rules of [task 125](125__TASK__DELETE_TOOL.md).

### The target

- `to` — a template of the new path, relative to the working directory of the file, `/` as the separator:
  `{dir}` (the directory, empty for the working directory), `{name}` (the file name), `{stem}` (the name without the
  last extension), `{ext}` (the last extension without the dot, empty without one), `{path}` (the whole relative
  path); `{{` and `}}` for literal braces
- `pattern` — optional regex (Rust syntax, like `=~`) on the relative path; its groups are `{1}`, `{2}`, ... and
  `{name}`-like named groups in `to`; a file whose path doesn't match is skipped and reported
- examples: `{dir}/{stem}.tsx` (change the extension), `archive/{path}` (move under a directory),
  `pattern = 'src/(.*)_test\.rs'`, `to = 'tests/{1}.rs'`
- an unknown placeholder, an unclosed brace, a group which doesn't exist: `invalid_template` with the position, before
  anything is changed
- a target outside the allowed roots: the file is skipped with `permission_denied`
- missing directories of the target are created

### Conflicts

- `on_conflict`: `fail` (default), `skip`, `overwrite`
- the whole plan is checked before anything changes: two files with the same target, a target which is another file
  of the selection, a target which exists — with `fail` nothing is renamed and the error lists the conflicts
- `overwrite` never replaces a directory with a file or a file with a directory
- a rename to the same path is a no-op, reported as unchanged

### Moving

- within one file system: an atomic rename
- across file systems: copy, then delete the source; a failed copy leaves the source untouched
- annotations: `readOnlyHint: false`, `destructiveHint: true` (an overwrite replaces a file), `idempotentHint: false`,
  `openWorldHint: false`

## Out of scope

- changing references to the renamed files in other files
- case-only renames on case-insensitive file systems beyond what the platform rename does (a known gap, listed in the
  summary if not handled)
- copy: task 127

## Spec

- [TOOLS.md#tools](../TOOLS.md#tools) — the tool, its description, input schema, annotations
- [OUTPUT.md#errors](../OUTPUT.md#errors) — `invalid_template`, `rename_conflict`

## Examples

`126__EXAMPLES__RENAME_TOOL.md` covers:

- change the extension of the files of a result
- move the files under another directory, creating it
- `pattern` with groups; a file which doesn't match the pattern
- conflicts with `fail`, `skip`, `overwrite`; two files to one target
- an invalid template
- a target outside the allowed roots
- `dry_run`; the JSON format

## Acceptance criteria

- the spec describes the tool before the code is written
- every example in `126__EXAMPLES__RENAME_TOOL.md` passes as a test; the examples of all earlier tasks still pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
