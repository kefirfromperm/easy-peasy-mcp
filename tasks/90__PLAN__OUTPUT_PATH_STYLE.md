# Plan for task 90: `output.path_style`

## Design

- `src/config/keys.rs`: `Key::PathStyle` after `GroupRepeated` (`sort` comes between them in the spec, later),
  `Type::Choice(&["relative", "absolute"])`, default `"relative"`.
- `src/engine.rs`: with `absolute` the prefix of every working directory is its canonical path without the verbatim
  prefix of Windows, so the paths of the files, of the path fragments and of the warnings are absolute; the output
  doesn't change.
- `src/output/mod.rs`: `path_parts` drops the root after a Windows prefix (`C:` + `\` + `x`, not `C:\\x`).
- `src/description.rs`: the schema.

## Tests

- examples E90.1–E90.9 (fixture `path-frags` of task 89)
- unit: absolute display paths, the request field

## Steps

1. failing tests
2. key, absolute prefixes

## Risks

- Windows paths are checked by task 120.
