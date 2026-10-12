# Task 125: `delete` tool

Status: not implemented

Depends on: task 111, task 115, task 120

## Goal

An agent which found files with `find` can remove them in one call, without a shell: the files of a previous result,
or the files of a query. Files go to the trash of the system, so a mistake can be undone.

This is the first tool which changes files. It also lays the ground for the other write tools
([126](126__TASK__RENAME_TOOL.md), [127](127__TASK__COPY_TOOL.md)): how they are enabled, how they select files, the
dry run and the response.

## Scope

A spec change first (its own commit; TOOLS.md, CONFIG.md, OUTPUT.md, README.md), then the code.

### Enabling — shared by all write tools

- a new read-only key `server.allow_write`, default `false`, set only by the config file and the command line
  (`--allow-write`); `get_config` shows it in the `# read-only:` comment line
- with `allow_write = false` the write tools are not listed and a call made anyway fails with `write_disabled`:
  `write tools are disabled`, hint `the configuration has server.allow_write = false`
- `update_config` can't change it (`read_only`)
- the server `instructions` and the tool annotations: `find` stays `readOnlyHint: true`; `delete` is
  `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false`
- not a breaking change: new tools and a new key ([TOOLS.md#versioning](../TOOLS.md#versioning))

### Selecting files — shared by all write tools

Exactly one of:

- `result: "r12"` — the files of a previous `find` result (all matched files, not only the shown ones); unknown or
  evicted: `result_expired` as for `within`; with `server.result_cache = 0` only `query` works
- `query` with `dirs`, `traversal`, `kinds`, `languages`, `version` — the same fields and meaning as in `find`; the
  query is evaluated like a `find` request

And:

- `expect: N` — optional: the operation fails with `count_mismatch` when the selection has a different number of files,
  before anything is changed; a guard for a query which matches more than the agent thinks
- a file of a result which changed since the result (size or modification time) or no longer exists is skipped with a
  warning, never acted on
- every path is checked against `server.allowed_roots` when the tool acts; outside: skipped with `permission_denied`
- a file inside an archive can't be changed: skipped with a warning
- symlinks are acted on themselves, never followed; a directory in the selection (`type = dir`) is acted on as a whole
  tree

### Dry run — shared by all write tools

- `dry_run: true` lists what would be done and changes nothing; the response is the same as the real one, with
  `dry run` in the header
- `dry_run` defaults to `false`; the tool description tells the agent to use it for a selection it hasn't seen

### Response — shared by all write tools

- the text format: a header with the counts (`deleted: 12  skipped: 1`), then the paths grouped by directory like
  `find` (`output.group_dirs`), then the warnings; the JSON format with `output.format = "json"`
- a file which fails (permission, busy, vanished) doesn't stop the others; it is reported with its error;
  `isError` is true only when nothing was done
- timeout and cancellation stop between files; the response lists what was done

### `delete`

- moves the files to the trash of the system (freedesktop trash on Linux, Trash on macOS, Recycle Bin on Windows)
- `permanent: true` deletes them instead; the description says it can't be undone
- where there is no trash (e.g. a file system without one) the file is skipped with `trash_unavailable`, never
  deleted permanently without `permanent: true`
- a directory is moved to the trash as one item; with `permanent` it is removed recursively

## Out of scope

- rename and copy: tasks 126 and 127
- restoring from the trash
- changing the content of files

## Spec

- [TOOLS.md#tools](../TOOLS.md#tools) — the new tool, its description, input schema, annotations
- [CONFIG.md#server](../CONFIG.md#server) — `server.allow_write`, `--allow-write`
- [OUTPUT.md#result-ids](../OUTPUT.md#result-ids) — results used by the write tools
- [OUTPUT.md#errors](../OUTPUT.md#errors) and [OUTPUT.md#warnings](../OUTPUT.md#warnings) — the new codes

## Examples

`125__EXAMPLES__DELETE_TOOL.md` covers:

- `allow_write = false`: the tool is not listed (startup example with `tools/list`), a call fails with `write_disabled`
- delete the files of a result; delete by a query; a directory as a whole
- `dry_run`: the response and the files untouched
- `expect` which matches and which doesn't
- a file of the result changed or deleted since the result; a file in an archive; a file outside the allowed roots
- `permanent: true`
- `result` and `query` together, neither of them, an expired result
- the JSON format
- the trash is checked through a test hook of the library, so the examples don't touch the trash of the machine

## Acceptance criteria

- the spec describes the tool, the shared rules and the new codes before the code is written
- every example in `125__EXAMPLES__DELETE_TOOL.md` passes as a test; the examples of all earlier tasks still pass
- the plan names the trash dependency (e.g. the `trash` crate) and why
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
