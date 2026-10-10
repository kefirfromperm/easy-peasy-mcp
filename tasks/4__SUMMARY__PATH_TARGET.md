# Summary of task 4: `path` target

Server version: `0.4.0`.

## What was built

- `path` with `=`, `has`, `starts`, `ends`, `~` (QUERY.md#targets): the path relative to the working directory,
  without it, `/` as the separator, compared in NFC with the platform's case sensitivity.
- Wildcards on paths (QUERY.md#wildcards): `*` and `?` stay inside one directory, `**` crosses directories and
  `**/` also matches no directory, so `**/tests/**` matches the top level `tests/` too.
- The `find` description shows `path ~` and `path starts`.

## Architecture

`Filter::Text { id, target: Name | Path, matcher }` replaces `Filter::Name`. The search builds a `FileInfo` with the
NFC name and the NFC `/`-separated relative path of every entry; the filter picks the value of its target.

## Deviations from the plan

None.

## Tests

- examples E4.1–E4.8, fixture `paths`; examples matching several depths have the shallower files first in path
  order too, so they stay valid with relevance sorting
- unit: the subset accepts `path`; a compiled `path ~` condition; `slash_path`

## Limitations and known gaps

- `\` as a separator in queries on Windows (task 120)

## Try it

`{ "query": "path ~ 'src/**/*.rs'" }`
