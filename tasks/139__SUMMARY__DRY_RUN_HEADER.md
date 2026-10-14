# Summary of task 139: a dry run header which can't be read as done

Server version: `0.136.0`. Spec version: `2`.

## What was built

[TOOLS.md#dry-run](../TOOLS.md#dry-run): in spec 2 the header of a dry run of a write tool starts with what it is, so
an agent which skims it can't report files as deleted which are still there:

```
dry run  would delete: 2  skipped: 1  permanent
```

- `dry run`, then the count named with `would` and the verb of the tool (`would delete`, `would rename`, `would
  copy`, `would replace`), then the other counts (`matches`, `unchanged`, `skipped`, `failed`) and `permanent` as in
  the real call; no `dry run` at the end
- the header of the real call is unchanged, and so are the entries, the warnings and the errors of a dry run (a
  failing `expect` is the same `count_mismatch`)
- JSON keeps its keys in both versions (`"deleted": 2`, `"dry_run": true`)

### Spec version

The header is the text layout, so the change is breaking (TOOLS.md#versioning). It waited for spec version 2, which
task 141 introduced, and is a part of it: `"version": 1` keeps `deleted: 2  permanent  dry run` (E139.6). Task 141 was
done before this one for that reason; the table of the spec versions in TOOLS.md names both changes of spec 2.

## Architecture

- `write.rs`: `Outcome::to_text(verb, group_dirs, spec)`, `infinitive` makes the verb from the count
- `engine.rs`: `write_output` passes the spec version of the request to the text too

## Deviations from the plan

None.

## Tests

- E139.1–E139.10: `delete` with entries, with `skipped` and `permanent`, an empty selection, JSON, the real call,
  spec 1, a failing `expect`; `rename`, `copy`, `replace`
- unit test of the header in both versions and for every verb
- the examples of tasks 125–129 and 140 are spec 1 examples and keep their headers

## How to try it

```json
{ "query": "name ~ '*.tmp'", "dry_run": true }
```
