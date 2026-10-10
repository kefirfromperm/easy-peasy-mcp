# Summary of task 1: MVP — find files by name

Server version: `0.1.0` (the first task starts at `0.1.0`, AGENTS.md#server-version).

## What was built

- **Server** (TOOLS.md#transport): the binary `easy-peasy-mcp` speaks MCP over stdio with `rmcp`; stdout carries only
  protocol messages, logs (`warn`) go to stderr, the server stops when stdin closes. `initialize` returns
  `serverInfo` from `CARGO_PKG_VERSION`, `capabilities.tools.listChanged = false` and the instructions
  `File search: find + grep in one query. Spec version 1.`. `--version` prints `easy-peasy-mcp 0.1.0, spec 1`; any
  other argument prints the usage to stderr and exits with 2.
- **`find` tool** (TOOLS.md#find): `readOnlyHint: true`, `openWorldHint: false`, schema with `query` only, the cheat
  sheet reduced to the implemented subset. Unknown fields fail with `invalid_option` (with "did you mean"), the other
  spec fields and the empty query with `not_implemented`, a non-string `query` with `invalid_option`.
- **Query** (QUERY.md): the whole grammar is lexed and parsed — targets, operators, values, modifiers, labels,
  `not`/`and`/`xor`/`or`, brackets, `line( ... )`, `near( ... )`, comments — so syntax errors are the ones of the
  finished server: `unexpected_token`, `unexpected_end`, `unknown_target`, `unknown_operator`,
  `operator_not_allowed`, `unterminated_string`, each with the message, the position and a hint for what was
  expected. Valid syntax outside of `name = <string>` / `name ~ <wildcard>` fails with `not_implemented` at the
  first unsupported token. Strings `'...'`, `"..."` with `\' \" \\ \n \t` (other backslashes are kept), raw
  `` `...` ``; keywords in any case; the condition id `#1`; the legend as written in the query.
- **Wildcards** (QUERY.md#wildcards): `*`, `**`, `?`, `[abc]`, `[a-z]`, `[!abc]`, `{a,b}` (nested), the bracket
  escapes, whole-value match; `invalid_wildcard` for an unclosed `[` or `{` and a reversed range.
- **Search** (CONFIG.md#traversal): the start directory, hidden entries skipped, `.gitignore`, `.ignore`, `.aiignore`
  in every directory also outside of git (a later name wins), `.git/info/exclude` and the global git excludes, no
  symlinks followed, only regular files, generated files skipped by the built-in `kinds.generated` wildcards and
  markers in the first 5 lines, hard links returned once under the first path, names compared in NFC with the
  platform's case sensitivity.
- **Output** (OUTPUT.md#text-format): the header with result ids `r1`, `r2`, ... per session, the legend, files under
  directory headers (`./` for the working directory), `[#1]` per file, files in path order.

## Spec changes

- QUERY.md#errors: a message column for every query error and the rule for quoting tokens in messages and hints
- QUERY.md#strings: a backslash before another character is kept
- OUTPUT.md: the legend text rule, the header of files directly in a working directory, the error layout,
  `not_implemented` in the request errors

## Architecture

```
main.rs            command line, tracing to stderr, rmcp server over stdio
mcp.rs             ServerHandler: get_info, list_tools, call_tool → Engine (in spawn_blocking)
description.rs     instructions, the find description and schema
engine.rs          Engine { options, result counter }: call(tool, args) → ToolOutput { text, is_error }
request.rs         arguments → Request, field errors
query/             lexer → parser (AST) → ids and legend → validation + subset check → Filter
search/            ignore::WalkBuilder (parallel) → name filter → generated check → sort, dedup
output/            the text format
error.rs           ErrorCode, QueryError (byte span), Report → error text
```

A request flows `Engine::find` → `request::parse` → `query::parse` → `search::search` → `output::Response::to_text`.
The query layer reports the first error from the start of the query: syntax errors stop the parser, then wildcard
validation (for every `~` condition, also outside of the subset) and the subset check are ordered by position.

## Deviations from the plan

- Unit tests were written next to each module while implementing it rather than in the red commit; the red commit
  had the example, search and end-to-end tests.
- The subset check scans tokens instead of the AST: for one condition it is shorter and gives the position of the
  first unsupported token directly. Later tasks replace it with an AST check as the subset grows.

## Tests

- unit (53): lexer, parser (every error code of the task), subset (`not_implemented` cases), wildcards, suggestions,
  legend and ids, request fields, generated rules, output grouping, error rendering
- examples: E1.1–E1.25 in `tests/examples.rs`
- `tests/search.rs`: hard links, an NFD name matched by an NFC query, result ids not taken by errors
- `tests/mcp_stdio.rs`: initialize, tools/list, find, an error, result ids, an unknown tool, stdin close,
  `--version`, an unknown argument

## Limitations and known gaps

- empty result without funnel and hints (tasks 102–103), no `max_files` truncation (94), order by path instead of
  relevance (93), unreadable entries skipped silently (33)
- Windows hard links are not deduplicated yet (120)

## Try it

```sh
cargo build --release
cd ~/some/project && /path/to/target/release/easy-peasy-mcp
```

`.mcp.json` of Claude Code:

```json
{ "mcpServers": { "easy-peasy": { "command": "/path/to/target/release/easy-peasy-mcp" } } }
```

Request: `{ "query": "name ~ '*.rs'" }`.
