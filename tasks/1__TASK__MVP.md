# Task 1: MVP — find files by name

Status: not implemented

## Goal

An agent registers `easy-peasy-mcp` in its MCP client and finds files by name in the directory the server was started
in, with one `find` call and a query like `name ~ '*.rs'` or `name = 'Cargo.toml'`. No configuration, no output
options: only the query.

## Scope

### Server

- one binary `easy-peasy-mcp`, local stdio transport, JSON-RPC 2.0, one message per line (TOOLS.md#transport)
- stdout carries only protocol messages, logs go to stderr; the server stops when stdin is closed
- `initialize`: protocol revision negotiated by `rmcp`, `serverInfo` `{ "name": "easy-peasy-mcp", "version": "0.1.0" }`,
  capabilities `{ "tools": { "listChanged": false } }`, `instructions`:
  `File search: find + grep in one query. Spec version 1.` (the `get_config` sentence comes with the `get_config` task)
- `tools/list`: only `find`, annotations `readOnlyHint: true`, `openWorldHint: false`
- command line: `--version` prints `easy-peasy-mcp 0.1.0, spec 1` and exits; no other arguments; an unknown argument
  prints the usage to stderr and exits with a non-zero code

### `find` tool

- input schema: `{ "type": "object", "properties": { "query": { "type": "string" } } }` with the description of
  `query` from TOOLS.md#input-schema; the other spec fields are added to the schema by the tasks that implement them
- the tool description is the cheat sheet of TOOLS.md#description reduced to what this task implements, ending with
  `Spec version 1.`
- request fields:
  - `query` — required by this task; a missing or empty `query` fails with `not_implemented` (the empty query, all
    files, comes later)
  - any other field from the spec (`version`, `dirs`, `queries`, `within`, `since`, `timeout`, `traversal`, `output`,
    `escape`, `query_options`, `kinds`, `git`, `languages`) fails with `not_implemented`
  - a field unknown to the spec fails with `invalid_option`

### Query subset (QUERY.md)

- exactly one condition: `name = <string>` or `name ~ <wildcard>`
- strings: `'...'` and `"..."` with the escapes `\'`, `\"`, `\\`, `\n`, `\t`; raw `` `...` `` (QUERY.md#strings)
- wildcards: `*`, `?`, `[abc]`, `[a-z]`, `[!abc]`, `{a,b}`, bracket escapes `[*]`, `[?]`, `[[]`, `[{]`; the pattern
  matches the whole name (QUERY.md#wildcards)
- keywords are case-insensitive (`NAME ~ '*.rs'`), whitespace and line breaks between tokens are free
- case sensitivity follows the platform (`path_case = native`): sensitive on Linux, insensitive on macOS and Windows
- names are compared in Unicode NFC form
- the condition gets the id `#1`
- implicit `type = file`: only regular files are returned, no directories, no symlinks

### Query errors

Full spec format (OUTPUT.md#errors), the same code, message, position and hint the finished server gives, for errors
the subset can produce:

| Code                  | Example                         |
|-----------------------|---------------------------------|
| `unexpected_token`    | `name ~ '*.rs' 'x'`             |
| `unexpected_end`      | `name ~`                        |
| `unknown_target`      | `filename = 'a.rs'`             |
| `unknown_operator`    | `name contains 'a'`             |
| `unterminated_string` | `name = 'abc`                   |
| `invalid_wildcard`    | `name ~ '[a-'`                  |

Hints list the alternatives of the full spec, e.g. `unknown_operator` lists `=, has, starts, ends, ~, ~~, =~`.

Valid spec syntax outside the subset — other targets, other operators, modifiers, labels, `not` / `and` / `xor` /
`or`, brackets, `line( ... )`, `near( ... )`, `#` comments — fails with `not_implemented`, pointing at the first
unsupported token.

### `not_implemented`

- a tool error (`isError: true`) in the format of OUTPUT.md#errors:
  `error[not_implemented]: <feature> is not implemented yet`, with the query line and `^^^` marker when the feature is
  in the query
- added to the request errors table of OUTPUT.md#errors in this task (a new error code is not a breaking change,
  TOOLS.md#versioning)

### Search

- the working directory is the directory the server was started in (CONFIG.md#server: empty `allowed_roots` — the
  start directory, `default_dirs = ["."]`)
- the built-in traversal defaults (CONFIG.md#traversal): unlimited depth, symlinks not followed, hidden entries
  skipped, ignore files `.gitignore`, `.ignore`, `.aiignore` respected in every directory, also outside a git
  repository, `git_excludes` (`.git/info/exclude` and the global `core.excludesFile`)
- `skip_generated`: files of kind `generated` are skipped — the built-in `kinds.generated` wildcards and the
  `generated_markers` in the first `generated_marker_lines` lines (CONFIG.md#kinds)
- a file reachable several times (hard links) is returned once, under the path found first (OUTPUT.md#duplicates)

### Output

The default text format at the default detail `lines` (OUTPUT.md#text-format), which for a name-only query is the
list of files:

- the header `files: N  lines: 0  matches: 0  result: rK`
- the legend `#1 <condition as written in the query>`
- an empty line, then the files grouped under directory headers (`group_dirs = true`), each file with `[#1]` and
  no counts (they are 0)
- paths relative to the working directory, in the native form of the platform
- result ids `r1`, `r2`, ... per server session; this task only numbers them, the result cache comes with
  `within` / `since`
- files ordered by `sort = relevance`: this task orders by the full path, which is the relevance order when all files
  have equal scores (OUTPUT.md#relevance)

## Out of scope

Each item fails with `not_implemented`, or is a known gap listed below, until its task:

- configuration: config file, command line options, `get_config`, `update_config`, `allowed_roots`, `dirs`
- every request field except `query`, all `output` options, the JSON format
- the empty query, every target except `name`, every operator except `=` and `~`, modifiers, labels, logical
  operators, brackets, `line( ... )`, `near( ... )`, comments in the query
- content search, encodings, archives, binary detection
- `timeout`, progress, cancellation, concurrency limits (`server.threads`)

## Known gaps

The finished server answers differently in these cases; examples of this task must not cover them:

- **empty result** — the finished server adds the funnel and hints (`funnel = auto`); this task returns only the
  header
- **more than `max_files` (100) files** — the finished server truncates and returns a cursor, and adds the funnel
  above 1000 files; this task lists all files
- **relevance** — the finished server ranks by kind, depth and recency; this task orders by path
- **unreadable entries** — the finished server reports `permission_denied` warnings; this task skips them silently
- **`instructions` and the tool description** describe only the implemented subset

## Examples constraints

`1__EXAMPLES__MVP.md` must be correct for the finished server (AGENTS.md#2-examples--n__examples__namemd), so:

- every example matches at least one file and fewer than 100
- the matched files of an example have equal relevance: the same kind, the same depth and the same modification time;
  then the finished server orders them by path, as this task does
- no unreadable entries, no symlinks in the fixtures
- fixtures should contain what the defaults skip — hidden files, ignored files, generated files, directories whose
  name matches — to show they are not returned
- cover: exact name, `*`, `?`, `[...]`, `[!...]`, `{a,b}`, bracket escapes, each string form and escape, keywords in
  upper case, files in nested directories (directory groups), and each query error listed above

## Acceptance criteria

- every example in `1__EXAMPLES__MVP.md` passes as a test
- unit tests cover the lexer, the parser, wildcard matching and every error code of this task, `not_implemented`
  included
- an end-to-end test spawns the release binary, runs `initialize`, `tools/list` and a `find` over stdio
- the definition of done of AGENTS.md is met
- the release binary works when registered in an MCP client (e.g. `.mcp.json` of Claude Code)
