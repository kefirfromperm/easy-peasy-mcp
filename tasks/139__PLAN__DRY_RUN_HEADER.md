# Plan for task 139: a dry run header which can't be read as done

## Design

The spec change is committed already ([TOOLS.md#dry-run](../TOOLS.md#dry-run)): the header of a dry run starts with
`dry run  would <verb>: N` in spec 2; spec 1 keeps `deleted: N  …  dry run`.

### Spec version

The header is the text layout of the response: a breaking change (TOOLS.md#versioning). It waited for spec 2 (task
141) and goes into it; a request with `"version": 1` keeps the old header. JSON keeps its keys (`"deleted": 2`,
`"dry_run": true`): renaming a key would break a parser in both versions for no gain, the key names the count and
`dry_run` says what it is.

- `write.rs`: `Outcome::to_text(verb, group_dirs, spec)`; in spec 2 a dry run writes `dry run  would <verb>: N`, the
  verb made from the count (`deleted` → `delete`, `renamed` → `rename`, `copied` → `copy`, `replaced` → `replace`),
  then the other counts and `permanent`, no `dry run` at the end
- `engine.rs`: `write_output` passes the spec version of the request (already there for the JSON version)

## Tests

- E139.1–E139.10 (spec 2): `delete`, `rename`, `copy`, `replace`; JSON; the real call; spec 1; a failing `expect`
- unit test of the header in both versions
- the examples of the write tools of tasks 125–129 and 140 are spec 1 examples and keep `… dry run`

## Steps

1. red: the unit test
2. the header
3. summary, status `implemented`, minor version bump `0.135.0` → `0.136.0`
