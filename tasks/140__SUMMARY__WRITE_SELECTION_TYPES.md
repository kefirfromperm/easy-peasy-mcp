# Summary of task 140: the write tools say how to select directories and symlinks

Server version: `0.134.1`.

## What was built

[TOOLS.md#delete](../TOOLS.md#delete), [TOOLS.md#selection](../TOOLS.md#selection): the descriptions of `delete`,
`rename` and `copy` say that the query selects files only, like `find`, and name the conditions which select the
rest, in the line which said what happens to them:

```
The query selects files only, like find: type = dir selects directories, deleted with everything in them;
  type = link symlinks, deleted themselves, never followed.
```

An agent which wants to delete `link.tmp` with `name ~ '*.tmp'` now reads that it needs `type = link` (or
`type in (file, link)`), instead of turning on `traversal.follow_symlinks`, which follows every symlink of the walk.
The selection section of TOOLS.md says the same with that example. `replace` acts on files only and keeps its
description. Each description is one line longer.

## Architecture

- `src/description.rs`: the three descriptions; nothing else changed: the implicit `type = file` stays

## Deviations from the plan

None. The examples were fixed once before the code: the entries of a write tool in the working directory are under
the header `./`.

## Decisions

- no hint in the response of an empty selection: it would need a second evaluation of the query and a new piece of
  output with an `omit` part, which the write tools don't have; the description removes the cause. So a patch bump

## Tests

- E140.1–E140.4 (fixture `select-types`): the descriptions in `tools/list`, a name selects the file only,
  `type = link` deletes the symlink and keeps its target, `type in (file, link)`, `type = dir`

## How to try it

```json
{ "query": "name ~ '*.tmp' and type in (file, link)", "dry_run": true }
```
