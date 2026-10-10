# Summary of task 14: `accessed` target

Server version: `0.14.0`.

## What was built

- `accessed` with the operators and date values of `modified` (QUERY.md#targets); unknown and `date_unsupported`
  where the file system has no access date.
- The access time is taken from the metadata before the search reads any content (CONFIG.md#platforms: it may be
  stale with `noatime` / `relatime`, but the search itself doesn't make files match).
- The `find` description shows `accessed`.
- The fixture setup sets access times: `atime <path> <date> <time>` (AGENTS.md).

## Architecture

`NumberTarget::Accessed`, `FileValues.accessed`, `Needs.accessed`; `date_unsupported` covers `created` and
`accessed`.

## Deviations from the plan

None.

## Tests

- examples E14.1–E14.6 (fixture `atimes`)
- unit: an `accessed` condition and its needs

## Limitations and known gaps

None new.

## Try it

`{ "query": "accessed < -30d" }`
