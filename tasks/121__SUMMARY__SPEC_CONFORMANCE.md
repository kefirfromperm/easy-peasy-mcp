# Summary of task 121: spec examples as tests

Server version: `0.121.0`.

## What was built

- 106 examples (E121.1–E121.119) reproduce the request and response examples of the spec byte for byte, each titled
  with its file and section: README.md and TOOLS.md (the request of the README, results, errors, progress,
  cancellation, the session example, the `update_config` description, the examples of the `find` description),
  QUERY.md (every recipe, every error code, scopes, comments, escaping, proximity, naming styles, fuzzy names,
  labels, gotchas), OUTPUT.md (every section with a response and every example of its Examples) and CONFIG.md (where
  the values come from, the command line, the full config file, ignore files, kinds, languages, `get_config`,
  `update_config`, the configuration examples)
- `tests/spec_queries.rs`: every query in the code blocks of QUERY.md (68) parses
- the spec examples which couldn't be reproduced because they were inconsistent were fixed in the spec, one commit
  per file: counts a small fixture reaches, result ids from `r1`, real cursors (the request hash has 8 hex digits),
  a long-line cut which follows its rule, hints and warnings in the format and order of their rules, `get_config`
  lists as literal strings, the whole sources form, paths under `~` instead of machine paths, the escape level
  which has too many forms (11), the `unknown_value` row with its `expected one of` hint
- two server bugs found by the examples and fixed: `~` in the working directories (`server.default_dirs`, also the
  `dirs` of a request) and in the path of `--config` (an MCP client starts the server without a shell)
- the spec now says what the server does where it was silent or different: a working directory inside an ignored
  directory is searched (like a path given to `rg`); a line which isn't JSON is ignored (as rmcp and the other MCP
  SDKs do), JSON which isn't a valid message gets `-32600`; `progress` is a float
- the fixture `exclude` of task 51 was missing its `target` directories in git (`.gitignore`): tracked now, so the
  examples pass on a fresh checkout, as in CI

## Known gaps of the task files

All checked; every gap names a later task, and all of them are implemented:

| Gap | Closed by |
|-----|-----------|
| relevance order | task 93 |
| `max_files` truncation, cursor | tasks 94, 101 |
| output limits, long line cuts | tasks 95–100, 98 |
| funnel and hints of empty and huge results | tasks 102–104 |
| dedent, repeated lines | tasks 84, 86 |
| `permission_denied` warnings of unreadable entries | task 33 |
| the keys of later tasks in `get_config` | the key tasks; `UNIMPLEMENTED` is empty |
| the instructions and the tool descriptions | task 117 |

## Architecture

- `src/engine.rs`: `working_dir` expands `~`; `src/main.rs`: `--config` expands `~` with the same home directory.
- `tests/spec_queries.rs`: the code blocks of QUERY.md, the blocks which aren't queries skipped by name.
- the work was split by spec file between agents in their own worktrees; the parts were merged into one examples
  file, ids in ranges per file.

## Deviations from the plan

None.

## Tests

- examples E121.1–E121.119 (fixtures `spec-*`), E121.116 on Windows only
- `tests/spec_queries.rs`: `every_query_of_query_md_is_valid`
- `tests/search.rs`: `a_working_directory_in_the_home_directory`

## Limitations and known gaps

- E121.116 (the Windows configuration example) was not run here.
- the `-32600` answer of rmcp to JSON which isn't a valid message has no `id`, even when the message had one.
- the fixtures with a log over 10M and a file over 100M write them on every run of the examples.

## Try it

`cargo test --test examples` runs every spec example; `grep -n "^## E121" tasks/121__EXAMPLES__SPEC_CONFORMANCE.md`
lists them by spec section.
