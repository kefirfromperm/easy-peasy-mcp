# Task 139: A dry run header which can't be read as done

Status: not implemented

Depends on: task 125

## Goal

The response of a dry run says what would happen, not what happened. Today the header of a dry run is the header of
the real call with `dry run` at the end ([TOOLS.md#dry-run](../TOOLS.md#dry-run)):

```
deleted: 2  dry run

ep-delete-test/
a.tmp
```

The first word an agent reads is `deleted`; the `dry run` marker is last, after the counts, and is easy to miss in a
long header (`deleted: 3  skipped: 1  failed: 1  permanent  dry run`). An agent which skims the header can report
files as deleted which are still there, or skip the real call.

Found by trying the server: a dry run of `delete` with `query = "name ~ '*.tmp'"` answered `deleted: 2  dry run`.

## Scope

A spec change first (its own commit; TOOLS.md, the examples of the write tools), then the code.

- the header of a dry run starts with what it is, e.g. `dry run: would delete 2` or `would delete: 2`; the exact form
  is fixed in the spec step, for every count of the header (`skipped`, `failed`) and with `permanent`
- the same form serves the next write tools (`replace`, task 129): the spec words it for a write tool, not for
  `delete` only
- JSON keeps `"dry_run": true`; whether the count key changes too (`"deleted"` in a dry run) is decided in the spec
  step
- the header of the real call is unchanged
- a breaking change of the text format? Decided in the spec step by [TOOLS.md#versioning](../TOOLS.md#versioning);
  if it is, it waits for the next spec version or is behind an option

## Out of scope

- the entries and the warnings of a dry run: unchanged
- what a dry run checks (trash availability is still known only when acting)

## Spec

- [TOOLS.md#dry-run](../TOOLS.md#dry-run), [TOOLS.md#response-of-a-write-tool](../TOOLS.md#response-of-a-write-tool)
- [TOOLS.md#versioning](../TOOLS.md#versioning)

## Examples

`139__EXAMPLES__DRY_RUN_HEADER.md` covers:

- a dry run with entries, with an empty selection, with `skipped`, with `permanent`, text and JSON
- the real call with the same selection, unchanged
- a dry run with `expect` which fails: the error is unchanged

## Acceptance criteria

- the spec fixes the header of a dry run before the code is written
- every example in `139__EXAMPLES__DRY_RUN_HEADER.md` passes as a test; the examples of all earlier tasks still pass
  (or are changed by a decided breaking change, each change named in the summary)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
