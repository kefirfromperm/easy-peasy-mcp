# Plan for task 140: the write tools say how to select directories and symlinks

## Design

The spec change is committed already ([TOOLS.md#delete](../TOOLS.md#delete), [TOOLS.md#selection](../TOOLS.md#selection)):
the descriptions of `delete`, `rename` and `copy` replace their line about directories and symlinks with one which says
that the query selects files only and names `type = dir` and `type = link`. `replace` acts on files only and keeps its
description.

- `src/description.rs`: the three descriptions, word for word as in TOOLS.md
- no change of the selection itself: the implicit `type = file` stays

## Rejected

- the hint of an empty selection (`no files selected; 1 symlink matches: add type = link`): a second evaluation of the
  query, and a new piece of output which needs an `omit` part the write tools don't have; the description removes the
  cause, the dry run shows the selection

## Tests

- E140.1–E140.4; E140.1 fails before the change
- earlier examples which change: the descriptions of the write tools in `tools/list`, if an example has them in full

## Steps

1. red: the examples
2. the descriptions
3. fix the earlier examples
4. summary, status `implemented`, patch version bump `0.134.0` → `0.134.1`
