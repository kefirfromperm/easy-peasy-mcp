# Plan for task 1: MVP — find files by name

## Design

One crate: `src/lib.rs` holds everything, `src/main.rs` parses the command line, sets up `tracing` to stderr and runs
the MCP server over stdio.

```
src/
  main.rs             --version, usage, logging, start the server
  lib.rs              module tree, SPEC_VERSION, re-exports
  engine.rs           Engine: session state (result ids), find(arguments) -> ToolOutput
  mcp.rs              rmcp ServerHandler: initialize info, tools/list, tools/call -> Engine
  description.rs      the find tool description and input schema
  error.rs            Report: code, message, position, hint; rendering in the text format
  request.rs          JSON arguments -> Request (field checks: unknown -> invalid_option, spec but not
                      implemented -> not_implemented)
  query/
    mod.rs            parse(text) -> Query: lexer -> parser -> ids/legend -> subset check -> compiled plan
    lexer.rs          tokens with spans (char offsets): words, strings, operators, values, modifiers, labels,
                      brackets, commas, comments
    parser.rs         the full grammar of QUERY.md#grammar into an AST; syntax errors with hints
    ast.rs            Expr (or/xor/and/not/group/cond), Condition, Target, Op, ...
    subset.rs         rejects the valid syntax this server doesn't implement yet -> not_implemented
    wildcard.rs       wildcard -> regex, invalid_wildcard errors
    suggest.rs        "did you mean" by synonyms and edit distance
  search/
    mod.rs            search(&Plan, root) -> Vec<Match>
    walk.rs           ignore::WalkBuilder: hidden, ignore files, git excludes; hard link dedup
    generated.rs      kinds.generated wildcards and generated markers
  output/
    mod.rs            text format: header, legend, directory groups
```

### Data flow

`tools/call find` → `Engine::find(arguments)` → `request::parse` (fields) → `query::parse` (text → `Query` with
condition ids, legend and a compiled matcher) → `search::run` (walk, evaluate `name`, skip generated, dedup) →
sort by path → `output::text` → `ToolOutput { text, is_error }`. Every error is a `Report` rendered by
`error.rs`; `is_error = true`.

### Decisions

- **Full grammar parser now, subset check after it.** The parser knows every token and construct of the spec, so a
  query outside the subset gets the error the finished server gives (a syntax error) or `not_implemented` at the first
  unsupported token. The alternative — a parser for the subset only — would report `unexpected_token` for valid
  syntax like `and`, which later tasks would have to change.
- **Hand-written lexer and recursive descent parser** instead of `pest` / `chumsky`: exact error positions and hints
  are the main requirement, a hand-written parser controls them directly and needs no dependency.
- **Wildcards compiled to `regex`** instead of `globset`: the errors (`invalid_wildcard` with a hint) and the exact
  semantics (`**/` matches no directory too, `[!...]` and `?` never match `/`, bracket escapes) are ours; the regex
  crate gives Unicode simple case folding for `/i` and `path_case`.
- **rmcp with a hand-written `ServerHandler`** (no tool macros): the arguments arrive as raw JSON, so invalid input is
  a tool error (`invalid_option`), never a protocol error.
- **Walk with `ignore`**: the standard `.gitignore` handling is off and `.gitignore`, `.ignore`, `.aiignore` are
  custom ignore file names in this order, so they work outside a git repository and a later name wins at the same
  level (CONFIG.md#ignore-files); `git_exclude` and `git_global` on, `hidden(true)`, `follow_links(false)`.
- **Hard links**: on Unix files with `nlink > 1` are deduplicated by `(dev, ino)` after sorting by path, so the path
  found first in path order is kept; Windows identity comes with task 120 (there the walk only finds each path once).
- **Path order**: `Path` component order of the relative path, which is the order inside a directory walk.
- **Generated files**: the `kinds.generated` default wildcards (matched like `path ~`) and the first 5 lines checked
  for the default markers; only for files which matched the query.
- **Case**: `path_case = native` from `cfg!(target_os)`: sensitive on Linux, insensitive elsewhere (`(?i)` in the
  regex). Names and patterns are compared in NFC (`unicode-normalization`).
- **Tests use the library entry point** `Engine::find` with `git_global` off, so the machine's global git excludes
  don't change the results.

### Dependencies

| Crate | Why |
|-------|-----|
| `rmcp` (`server`, `transport-io`) | MCP server over stdio, protocol negotiation |
| `tokio` | async runtime for rmcp; the search runs in `spawn_blocking` |
| `serde`, `serde_json` | tool arguments and schemas |
| `ignore` | directory walk with ignore files |
| `regex` | compiled wildcards |
| `unicode-normalization` | NFC names |
| `thiserror` | error enums |
| `tracing`, `tracing-subscriber` | logs to stderr |
| `anyhow` | `main.rs` only |
| dev: `tempfile`, `filetime`, `pretty_assertions` | fixtures in a temp dir with fixed mtimes, readable diffs |

## Tests

- unit, `query/lexer.rs`: every token kind, string escapes, raw strings, unterminated strings, keywords in any case,
  spans in characters
- unit, `query/parser.rs`: conditions, every error code of the task with message, position and hint; valid syntax
  outside the subset parses
- unit, `query/subset.rs`: `not_implemented` for other targets, operators, modifiers, labels, logical operators,
  brackets, `line`, `near`, comments, the empty query, with the position of the first unsupported token
- unit, `query/wildcard.rs`: `*`, `?`, `[abc]`, `[a-z]`, `[!abc]`, `{a,b}`, nested alternatives, bracket escapes,
  `**`, invalid patterns, case insensitivity
- unit, `request.rs`: unknown field → `invalid_option`, spec fields → `not_implemented`, missing/empty query, wrong type
- unit, `output`: header, legend, directory groups including `./`
- example tests, `tests/examples.rs`: reads every `tasks/*__EXAMPLES__*.md`, copies the fixture into a temp dir, sets
  the mtimes of the tree block, runs each request through `Engine::find` and compares the whole text; E1.1–E1.25
- integration, `tests/search.rs`: hard links returned once; an NFD file name matched by an NFC query
- end-to-end, `tests/mcp_stdio.rs`: spawn the binary, `initialize` (serverInfo, capabilities, instructions),
  `tools/list` (only `find`, annotations, schema), `tools/call find` on a temp dir; `--version`; an unknown argument

## Steps

1. Cargo project: edition 2024, `rust-toolchain.toml`, lints (`clippy::pedantic` warn, `unsafe_code = "forbid"`)
2. failing tests: example runner, unit tests, e2e test
3. lexer, parser, wildcard, subset check, condition ids and legend
4. request parsing and errors
5. walk, generated check, dedup, sort
6. text output, engine, MCP adapter, main
7. clippy, fmt, release build, register the binary in `.mcp.json` and try it

## Risks

- `ignore` precedence of custom ignore files at the same level must be "later name wins" — covered by E1.18
- the user's global git excludes could hide fixture files; tests turn `git_global` off
- rmcp 3.x API details (non-exhaustive structs) — only the low-level handler is used
