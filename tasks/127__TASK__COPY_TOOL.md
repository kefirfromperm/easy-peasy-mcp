# Task 127: `copy` tool

Status: in progress

Depends on: task 126

## Goal

An agent can copy the files it found in one call: back them up before a change, or create files from existing ones,
by a pattern instead of one call per file.

## Scope

A spec change first (its own commit), then the code. Enabling, selecting files, `expect`, `dry_run`, the response and
the errors are the shared rules of [task 125](125__TASK__DELETE_TOOL.md); the target (`to`, `pattern`) and the
conflicts (`on_conflict`) are the ones of [task 126](126__TASK__RENAME_TOOL.md).

- the copy keeps the content byte for byte and the modification time; permissions as far as the platform keeps them
- a directory in the selection is copied with its whole tree; the traversal options of the request (hidden files,
  ignore files, `exclude`) don't filter inside it — the agent copies what it selected
- a symlink is copied as a symlink, never followed
- a file is written to a temporary name next to the target and renamed into place, so a failed copy leaves no partial
  file
- missing directories of the target are created
- `traversal.max_file_size` doesn't limit the copy: it is about reading content for a search
- annotations: `readOnlyHint: false`, `destructiveHint: true` (`overwrite` replaces files; the default `fail` never
  does), `idempotentHint: true`, `openWorldHint: false`

## Out of scope

- copying to a place outside the allowed roots
- copying between machines or into archives

## Spec

- [TOOLS.md#tools](../TOOLS.md#tools) — the tool, its description, input schema, annotations

## Examples

`127__EXAMPLES__COPY_TOOL.md` covers:

- copy the files of a result next to themselves (`{dir}/{stem}.bak.{ext}`)
- copy a directory as a whole tree into another directory
- conflicts with `fail`, `skip`, `overwrite`
- a symlink in the selection
- a target outside the allowed roots
- `dry_run`; the JSON format

## Acceptance criteria

- the spec describes the tool before the code is written
- every example in `127__EXAMPLES__COPY_TOOL.md` passes as a test; the examples of all earlier tasks still pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
