# MCP Tools

How the server talks to an MCP client: transport, versioning, tool definitions, results, errors, progress,
cancellation and timeouts. The query is described in [QUERY.md](QUERY.md), the output in [OUTPUT.md](OUTPUT.md), the
configuration in [CONFIG.md](CONFIG.md).

- [Transport](#transport)
- [Versioning](#versioning)
- [Tools](#tools)
  - [find](#find)
  - [get_config](#get_config)
  - [update_config](#update_config)
- [Results](#results)
- [Errors](#errors)
- [Timeout](#timeout)
- [Progress](#progress)
- [Cancellation](#cancellation)
- [Concurrency](#concurrency)
- [Session example](#session-example)

## Transport

Local **stdio** only. The client starts the server as a child process and talks JSON-RPC 2.0 over its stdin and
stdout, one message per line.

- stdout carries only protocol messages
- logs go to stderr (`--log-level`, see [CONFIG.md](CONFIG.md#command-line))
- the server stops when stdin is closed
- the MCP protocol revision is negotiated in `initialize`

Server capabilities announced in `initialize`:

```json
{
  "capabilities": { "tools": { "listChanged": false } },
  "serverInfo": { "name": "easy-peasy-mcp", "version": "0.1.0" },
  "instructions": "File search: find + grep in one query. Call get_config first to learn allowed roots and defaults. Spec version 1."
}
```

## Versioning

Two independent versions:

| Version          | Example  | Covers                                                                         |
|------------------|----------|--------------------------------------------------------------------------------|
| server version   | `0.1.0`  | the program, semantic versioning; in `serverInfo` and `--version`              |
| spec version     | `1`      | the contract with the agent: query language, request fields, response format   |

The spec version is an integer, it grows only on a breaking change.

| Breaking — new spec version                                | Not breaking — same spec version                     |
|------------------------------------------------------------|------------------------------------------------------|
| a field, operator, target or modifier removed or renamed   | a new optional request field or output option        |
| a changed meaning of existing syntax or field              | a new operator, target, modifier, escape style       |
| a changed default which changes the results                | a new warning or error code                          |
| a changed text format layout                               | a new response field in JSON                         |
|                                                            | tuned relevance weights: the order of `sort = relevance` is a ranking, not a contract |

- A request can pin the spec version with `"version": 1`. Without it the current version is used.
- The server supports the current spec version and may keep older ones; `get_config` lists them in the first comment
  line of its result: `# easy-peasy-mcp 0.1.0, spec 1, supported specs: 1`.
- An unsupported version fails with `unsupported_version` and the list of supported ones:
  `error[unsupported_version]: the spec version 2 is not supported`, hint `supported spec versions: 1`; a value which
  is not a whole number of at least 1 is `invalid_option`: `'version' must be a whole number of at least 1`.
- The JSON response has `"version"`. The text response doesn't, to save tokens: the agent knows what it asked for.
- The `find` description and the server `instructions` say which spec version they describe.

## Tools

| Tool            | Annotations                                                         | Meaning                          |
|-----------------|---------------------------------------------------------------------|----------------------------------|
| `find`          | `readOnlyHint: true`, `openWorldHint: false`                        | search files                     |
| `get_config`    | `readOnlyHint: true`, `openWorldHint: false`                        | the effective configuration      |
| `update_config` | `readOnlyHint: false`, `destructiveHint: false`, `idempotentHint: true`, `openWorldHint: false` | change the configuration in memory |

`update_config` is not listed when `server.allow_update = false`; a call made anyway fails with `update_disabled`.
`find` stays `readOnlyHint: true` with `output.file`: it writes only its own responses into `server.output_dir`, never
the files it searches.

### find

#### Description

The description is what the agent reads to learn the query language, so it is a compact cheat sheet. The full text
sent in `tools/list`:

```
Search files by name, path, metadata and content in one query (find + grep in one). Read-only. Spec version 1.
Call get_config once first to learn the allowed roots and defaults.

QUERY: conditions joined with not, and, xor, or (this precedence) and brackets. Empty query = all files.
  name ~ '*.rs'                  wildcard on the file name: * ? ** [abc] [!abc] {a,b}
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator
  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)
  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it
  size > 1M  modified > -7d  created  accessed  type = dir  empty
  lang = rust   kind in (source, test)   git = changed|branch|staged|untracked
  name ~~ 'cfgldr'               fuzzy file name (also path ~~)
  line(text has 'a' and not text has 'b')    conditions on the same line; len > 120 works inside line()
  near(text has 'lock', text has 'await', 3[, ordered][, closest|first])   lines within 3 of each other
Modifiers after a text value: /i ignore case, /w whole word, /m multiline regex, /c skip comments,
  /n any naming style ('parse query' = parseQuery, parse_query, PARSE_QUERY ...),
  /e(backslash|quote_doubling|xml|url, 0..2|any) also match the escaped text (JSON inside JSON etc.)
Labels: text has 'x' @name — the output shows which condition matched.
Strings: '...' or "..." with \ escapes; `...` raw, best for regex.
Dates: 2026-10-01, -30m -2h -7d -2w. Sizes: 512 10K 5M 1G.

OUTPUT: output.detail = exists | count | facets | files | lines (default); output.context = N lines around.
output.budget = N tokens: the server picks the richest detail which fits. By default files are sorted by relevance.
Text response with the default options: "42:" matched line, "41-" context line, "--" gap, ⟦ids:text⟧ a match,
[#1 @x] matched conditions, ⇥N removed indentation; empty or huge results come with a funnel and hints how to fix
the query. output.omit = ['legend', 'line_numbers', ...] or 'all' leaves those parts out.
If the response has next_cursor, pass it back as output.cursor with the same request to get the next page.
Responses with count detail or higher have a result id (r12), unless the result cache is off: within = "r12"
searches only its files, since = "r12" returns only the changes.
queries = [{label, query}, ...] runs several queries in one call.
output.file = 'x.txt' writes the response to a file under the output dir and returns its path; for big results.

EXAMPLES
  name ~ '*.py' and text has 'import requests'
  not text has 'license' /i                                   files without a match
  name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')
  text has `"role": "admin"` /e(backslash, any)               also inside stringified JSON
  kind = source and name has 'config loader' /n               a file by a vague name
  git = changed and text has 'dbg!('                          leftovers in files with uncommitted changes
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
    "query": { "type": "string", "description": "Search query, see the tool description. Empty: all files." },
    "queries": {
      "type": "array", "minItems": 1,
      "description": "Several queries in one call instead of `query`.",
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["query"],
        "properties": {
          "label": { "type": "string" },
          "query": { "type": "string" },
          "dirs": { "$ref": "#/properties/dirs" },
          "timeout": { "$ref": "#/properties/timeout" },
          "traversal": { "$ref": "#/properties/traversal" },
          "output": { "$ref": "#/properties/output" }
        }
      }
    },
    "within": { "type": "string", "description": "Result id: search only the files of that result." },
    "since": { "type": "string", "description": "Result id: return only the differences to that result." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max search time, e.g. '10s'. '0s': no limit." },
    "traversal": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "min_depth": { "type": "integer", "minimum": 0 },
        "max_depth": { "type": "integer", "minimum": 0 },
        "follow_symlinks": { "type": "boolean" },
        "hidden": { "type": "boolean" },
        "ignore_files": { "type": "array", "items": { "type": "string" } },
        "git_excludes": { "type": "boolean" },
        "ignore_paths": { "type": "array", "items": { "type": "string" } },
        "exclude": { "type": "array", "items": { "type": "string" } },
        "path_case": { "enum": ["native", "sensitive", "insensitive"] },
        "binary": { "enum": ["skip", "search", "exclude"] },
        "max_file_size": { "type": "string" },
        "encoding": { "type": "string" },
        "fallback_encoding": { "type": "string" },
        "archives": { "enum": ["none", "compressed", "all"] },
        "max_archive_size": { "type": "string" },
        "skip_generated": { "type": "boolean" }
      }
    },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "detail": { "enum": ["exists", "count", "facets", "files", "lines"] },
        "budget": { "type": "integer", "minimum": 100, "description": "Max response size in tokens." },
        "chars_per_token": { "type": "number", "exclusiveMinimum": 0 },
        "funnel": { "enum": ["auto", "always", "never"] },
        "facets": { "type": "array", "items": { "enum": ["dir", "ext", "lang", "kind", "git", "condition"] } },
        "facet_depth": { "type": "integer", "minimum": 1 },
        "facet_limit": { "type": "integer", "minimum": 1 },
        "meta": { "type": "boolean" },
        "context": { "type": "integer", "minimum": 0 },
        "before": { "type": "integer", "minimum": 0 },
        "after": { "type": "integer", "minimum": 0 },
        "path_fragments": { "type": "boolean" },
        "highlight": { "enum": ["auto", "none", "columns", "markers"] },
        "markers": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 },
        "max_files": { "type": "integer", "minimum": 1 },
        "max_lines": { "type": "integer", "minimum": 1 },
        "max_matches": { "type": "integer", "minimum": 1 },
        "max_matches_per_file": { "type": "integer", "minimum": 1 },
        "max_line_length": { "type": "integer", "minimum": 20 },
        "max_output_per_file": { "type": "integer", "minimum": 100 },
        "max_output": { "type": "integer", "minimum": 100 },
        "dedent": { "type": "boolean" },
        "blank_context": { "type": "boolean" },
        "group_dirs": { "type": "boolean" },
        "group_repeated": { "type": "integer", "minimum": 0 },
        "sort": { "type": "string", "pattern": "^(relevance|(path|modified|size|matches) (asc|desc))$" },
        "path_style": { "enum": ["relative", "absolute"] },
        "omit": {
          "anyOf": [
            { "type": "array", "items": { "enum": ["counts", "result", "legend", "funnel", "hints", "conditions", "file_counts", "match_ids", "line_numbers", "gaps", "indent", "warnings"] } },
            { "enum": ["all"] }
          ],
          "description": "Parts of the response to leave out, or \"all\"."
        },
        "cursor": { "type": "string" },
        "file": {
          "type": ["string", "boolean"],
          "description": "Write the response to this file under server.output_dir and return its path; true: a name like r12.txt."
        }
      }
    },
    "escape": {
      "type": "object",
      "additionalProperties": false,
      "properties": { "max_level": { "type": "integer", "minimum": 0 } }
    },
    "query_options": {
      "type": "object",
      "additionalProperties": false,
      "description": "The [query] section of the configuration.",
      "properties": { "fuzzy_min_score": { "type": "number", "minimum": 0, "maximum": 1 } }
    },
    "kinds": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "generated_markers": { "type": "array", "items": { "type": "string" } },
        "generated_marker_lines": { "type": "integer", "minimum": 1 },
        "generated": { "type": "array", "items": { "type": "string" } },
        "test": { "type": "array", "items": { "type": "string" } },
        "doc": { "type": "array", "items": { "type": "string" } },
        "config": { "type": "array", "items": { "type": "string" } }
      }
    },
    "git": {
      "type": "object",
      "additionalProperties": false,
      "properties": { "base": { "type": "string" } }
    },
    "languages": {
      "type": "object",
      "description": "Languages, replace the configured languages with the same name.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "extensions": { "type": "array", "items": { "type": "string" } },
          "names": { "type": "array", "items": { "type": "string" } },
          "shebangs": { "type": "array", "items": { "type": "string" } },
          "line_comments": { "type": "array", "items": { "type": "string" } },
          "block_comments": { "type": "array", "items": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 } },
          "nested_block_comments": { "type": "boolean" },
          "strings": { "type": "array", "items": { "type": "string" } },
          "source": { "type": "boolean" }
        }
      }
    }
  }
}
```

All fields are optional; `query` and `queries` exclude each other. The `[query]` section of the configuration is
passed as `query_options`, because `query` is the query itself. Missing fields come from the configuration, see
[CONFIG.md](CONFIG.md#where-the-values-come-from). The meaning of the `traversal` fields is in
[CONFIG.md](CONFIG.md#traversal), of the `output` fields in [OUTPUT.md](OUTPUT.md#options).

#### Result

The response of [OUTPUT.md](OUTPUT.md#response), in the format of `output.format`; with `output.file` its summary,
see [OUTPUT.md](OUTPUT.md#output-to-a-file).

### get_config

#### Description

```
Returns the effective configuration of the search server as TOML: allowed roots, default directories, traversal and
output defaults, limits, supported spec versions. Read-only. Call it once before the first find.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "section": { "enum": ["server", "traversal", "output", "escape", "query", "kinds", "git", "languages"] },
    "sources": { "type": "boolean", "description": "Add where each value comes from." }
  }
}
```

#### Result

TOML, the same format as the config file, see [CONFIG.md](CONFIG.md#get_config).

### update_config

#### Description

```
Changes the search server configuration in memory until the server stops: any value except the read-only ones of
server (allowed_roots, allow_update, output_dir, max_output_file_size). `set` is TOML like the config file, e.g.
"output.context = 2" or "[output]\ncontext = 2\nmax_files = 30". Lists are replaced, not merged. Atomic: if any value
is invalid, nothing changes. To change a value for one search only, pass it in the find request instead.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "set": { "type": "string", "description": "TOML with the values to change, like the config file, e.g. 'output.context = 2'." },
    "reset": { "type": "array", "items": { "type": "string" }, "description": "Keys like 'output.context', or ['all']." }
  }
}
```

#### Result

TOML with the changed keys and their old values in comments, see [CONFIG.md](CONFIG.md#update_config).

## Results

The result of every tool is one text content item:

```json
{
  "content": [ { "type": "text", "text": "files: 1  lines: 1  matches: 1  result: r1\n#1 text has 'TODO remove'\n\nsrc/\nmain.rs  [#1]  lines: 1  matches: 1\n12:// ⟦#1:TODO remove⟧\n" } ],
  "isError": false
}
```

- plain text, no `structuredContent`: the compact text format is the point, it saves tokens
- with `output.format = "json"` the text item contains the JSON document
- warnings are part of the response text, they don't make `isError` true
- a timeout is not an error: the partial result is returned with a `timeout` warning

## Errors

Two kinds:

| Kind              | When                                                              | How                                        |
|-------------------|-------------------------------------------------------------------|--------------------------------------------|
| tool error        | invalid query, invalid option, directory not allowed, bad cursor, unsupported version, ... | a normal result with `isError: true` |
| protocol error    | unknown tool, malformed JSON-RPC                                  | a JSON-RPC error                           |

Tool errors are results, not JSON-RPC errors, so the agent sees the message and the hint, if any, and can fix the
request.
Malformed JSON-RPC is JSON which isn't a valid message: it gets `-32600 Invalid request`. A line which isn't JSON at
all is ignored (logged at the `debug` level), like the other MCP SDKs do: it has no id to answer, and answering it
could start an error storm with a peer which echoes the answer back.
Input which doesn't match the schema is also a tool error (`invalid_option`) for the same reason. In a batch an
invalid query gets its error in its own section and the others still run; `isError` is true only if all queries
fail, see [OUTPUT.md](OUTPUT.md#batch).

```json
{
  "content": [ { "type": "text", "text": "error[unknown_operator]: unknown operator 'contains'\n  name ~ '*.rs' and text contains 'x'\n                         ^^^^^^^^\n  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~\n" } ],
  "isError": true
}
```

The error format is in [OUTPUT.md](OUTPUT.md#errors), the query error codes in [QUERY.md](QUERY.md#errors).

## Timeout

- the request field `timeout` (e.g. `"10s"`), otherwise `server.timeout` from the configuration (default `30s`)
- `"0s"` — no limit; durations are a whole number and `ms`, `s` or `m`, see [CONFIG.md](CONFIG.md#server); an invalid
  one is `error[invalid_option]: 'timeout' must be a duration like "10s" (units ms, s, m)`
- the time counts from the moment the request arrives, query parsing included
- when it expires the search stops and the results found so far are returned with `counts_exact: false`,
  `truncated: true`, `truncated_by: ["timeout"]` and a `timeout` warning; there is no `next_cursor`, because the
  unsearched part is unknown — narrow the query or raise the timeout

## Progress

If the request has a progress token, the server sends progress notifications about once a second while searching.

```json
{ "jsonrpc": "2.0", "id": 7, "method": "tools/call",
  "params": { "name": "find", "arguments": { "query": "text has 'req-7f3a9c'" }, "_meta": { "progressToken": "p7" } } }
```

```json
{ "jsonrpc": "2.0", "method": "notifications/progress",
  "params": { "progressToken": "p7", "progress": 12400.0, "message": "scanned 12400 files, 37 matched, 3.2s" } }
```

- `progress` is the number of scanned files (a JSON number with a fraction: MCP declares it a float), it only
  grows: the entries whose conditions were evaluated, matched or not; the message has the files matched so far and
  the time since the request arrived with one decimal
- `total` is not sent: the size of the tree is unknown until the walk ends
- no notifications for searches shorter than one second

## Cancellation

The client cancels a running search with:

```json
{ "jsonrpc": "2.0", "method": "notifications/cancelled", "params": { "requestId": 7, "reason": "user aborted" } }
```

- the walk and all search threads stop as soon as possible
- no response is sent for the cancelled request, as MCP requires
- a cancellation for an unknown or finished request is ignored

## Concurrency

- several `find` calls may run at the same time; they share the thread pool of `server.threads`
- every `find` takes a snapshot of the configuration when it starts; `update_config` during a search doesn't affect it
- `update_config` calls are applied one after another

## Session example

```
→ initialize
← serverInfo easy-peasy-mcp 0.1.0, capabilities: tools
→ tools/list
← find, get_config, update_config
→ tools/call get_config {}
← allowed roots ["/home/me/projects/app"], defaults, spec 1
→ tools/call find { "query": "lang = rust and text has 'unwrap(' /c", "output": { "detail": "count" } }
← files: 14  lines: 52  matches: 57  result: r1
→ tools/call find { "within": "r1", "query": "text has 'unwrap(' /c", "output": { "detail": "files", "sort": "matches desc", "max_files": 5 } }
← 5 files with the most unwraps, truncated: yes (max_files), result: r2
→ tools/call find { "dirs": ["src/parser"], "query": "text has 'unwrap(' /c", "output": { "context": 2 } }
← matched lines with context
```

Count first, then list the files, then look at the lines: cheap questions first, the expensive output only where it is
needed.
