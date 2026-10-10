# Plan for task 121: spec examples as tests

## Design

- `121__EXAMPLES__SPEC_CONFORMANCE.md`: one example per request/response example of README.md and TOOLS.md
  (E121.1–E121.19), QUERY.md — its recipes, errors and the other queries with a result — (E121.20–E121.49),
  OUTPUT.md — its sections (E121.50–E121.79) and its Examples (E121.80–E121.99) — and CONFIG.md (E121.100–E121.119),
  each titled with the file and the section it reproduces, with fixtures `spec-<file>-*` built so the responses
  match byte for byte; startup examples for the command line, the config files, `get_config`, `update_config` and
  the JSON-RPC exchanges of TOOLS.md. Where the spec leaves out the result id, the dedent or the directory groups to
  stay short, the example first turns them off with `update_config`, so the spec's request and response stay as
  they are.
- `tests/spec_queries.rs`: every query written in the code blocks of QUERY.md parses; the blocks which aren't
  queries (grammar, value lists, sample data, placeholders) are skipped by name.
- a spec example which the server can't reproduce because the example itself is inconsistent (counts, ids, cursors,
  order, a cut which breaks its own rule) is fixed in the spec, each spec file in its own commit; where the server
  breaks a rule of the spec, the code is fixed and the example is its test: `~` in the working directories
  (`server.default_dirs`) and in the path of `--config`.
- a working directory inside an ignored directory stays searched (like a path given to `rg`): the spec says so now.
- the Known gaps of all task files: each names the later task which closes it; checked against the task statuses.
- the work was split by spec file between agents in their own worktrees; their parts were merged into the one
  examples file, ids in ranges per file.

## Tests

- examples E121.1–E121.119
- `tests/spec_queries.rs`: `every_query_of_query_md_is_valid`

## Steps

1. the spec fixes, per spec file
2. the examples and their fixtures (E121.114 and E121.119 fail: `~`)
3. the parse test of QUERY.md
4. `~` in the working directories and in `--config`

## Risks

- the big files of some fixtures (a log over 10M, a file over 100M) are written by the setup on every run.
