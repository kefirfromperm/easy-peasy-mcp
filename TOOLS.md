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
  - [delete](#delete)
  - [rename](#rename)
  - [copy](#copy)
- [Write tools](#write-tools)
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
| `delete`        | `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false` | move files to the trash or delete them |
| `rename`        | `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: false`, `openWorldHint: false` | rename or move files |
| `copy`          | `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false` | copy files |
| `replace`       | `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: false`, `openWorldHint: false` | replace text in files |

`update_config` is not listed when `server.allow_update = false`; a call made anyway fails with `update_disabled`.
`delete`, `rename`, `copy` and `replace` are [write tools](#write-tools): they are listed only when `server.allow_write = true`.
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
Labels: text has 'x' @name — the output shows which condition matched; (... or ...) @name labels a bracket group.
Strings: '...' or "..." with \ escapes; `...` raw, best for regex.
Dates: 2026-10-01, -30m -2h -7d -2w. Sizes: 512 10K 5M 1G.

OUTPUT: output.detail = exists | count | facets | files | lines (default); output.context = N lines around.
output.count_by = file | line with detail = count: the counts of every file, or of every matched line too.
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
        "count_by": { "enum": ["total", "file", "line"], "description": "detail = count: the totals, or also the counts per file or per line." },
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
        "repeated_locations": { "type": "integer", "minimum": 0, "description": "The locations written for a repeated line; 0: all of them." },
        "sort": { "type": "string", "pattern": "^(relevance|(path|modified|size|matches) (asc|desc))$" },
        "path_style": { "enum": ["relative", "absolute", "in_dir"], "description": "in_dir: the working directory written once, the paths relative to it." },
        "omit": {
          "anyOf": [
            { "type": "array", "items": { "enum": ["counts", "result", "dir", "legend", "funnel", "hints", "conditions", "file_counts", "match_ids", "groups", "line_numbers", "line_counts", "gaps", "indent", "locations", "warnings"] } },
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
server (allowed_roots, allow_update, allow_write, output_dir, max_output_file_size). `set` is TOML like the config file, e.g.
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

### delete

#### Description

```
Deletes files: moves them to the trash of the system (Trash, Recycle Bin), so a mistake can be undone. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
The query selects files only, like find: type = dir selects directories, deleted with everything in them;
  type = link symlinks, deleted themselves, never followed.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be deleted and changes nothing. Use it for a selection you haven't seen.
permanent = true: deletes instead of moving to the trash. It can't be undone.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
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
    },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "dry_run": { "type": "boolean", "description": "List what would be deleted, change nothing." },
    "permanent": { "type": "boolean", "description": "Delete instead of moving to the trash. Can't be undone." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" }
      }
    }
  }
}
```

`result` and `query` exclude each other, one of them is required; `dirs`, `traversal`, `kinds` and `languages` work
only with `query`. The other fields are optional; `output.format` and `output.group_dirs` default to the
configuration. The rules of the selection, the dry run and the response are in [Write tools](#write-tools).

#### Result

The [response of a write tool](#response-of-a-write-tool), counted as `deleted`.

- the entries are moved to the trash of the system: the freedesktop trash on Linux, the Trash on macOS, the Recycle
  Bin on Windows; a directory is moved as one item
- `permanent: true` deletes them instead; a directory is removed recursively, without following the symlinks in it
- where there is no trash (a file system without one, no home directory) the entry is skipped with
  `trash_unavailable`, never deleted permanently without `permanent: true`
- `permanent` must be a boolean: `'permanent' must be a boolean`

### rename

#### Description

```
Renames or moves files: every file gets a new path made from a template. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
to = "...": the new path, relative to the working directory, / as the separator. Placeholders:
  {dir} {name} {stem} {ext} {path}   parts of the old path: src/a.test.ts = src, a.test.ts, a.test, ts, src/a.test.ts
  {1} {2} {word}                     groups of pattern; {{ and }} are braces
  e.g. "{dir}/{stem}.tsx", "archive/{path}"
pattern = "regex": must match the whole old path, e.g. "src/(.*)_test\.rs" with to = "tests/{1}.rs".
  A file which doesn't match is skipped with a warning.
Missing directories are created. The query selects files only, like find: type = dir selects directories, moved
  with everything in them; type = link symlinks, moved themselves.
on_conflict: a target which exists, two files to one target, a target which is another selected file:
  "fail" (default): nothing is renamed, the error lists them; "skip": those files are skipped;
  "overwrite": an existing file is moved to the trash and replaced, a directory never.
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be renamed and changes nothing. Use it for a selection you haven't seen.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "required": ["to"],
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
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
    },
    "to": { "type": "string", "minLength": 1, "description": "The new path, relative to the working directory: {dir} {name} {stem} {ext} {path}, groups {1} {word} of pattern." },
    "pattern": { "type": "string", "description": "Regex on the old path, the whole path must match; its groups are {1}, {2}, {name} in to." },
    "on_conflict": { "enum": ["fail", "skip", "overwrite"], "description": "A target which exists or collides. Default: fail." },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "dry_run": { "type": "boolean", "description": "List what would be renamed, change nothing." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" }
      }
    }
  }
}
```


`to` is required; `result` and `query` exclude each other, one of them is required; `dirs`, `traversal`, `kinds` and
`languages` work only with `query`. The other fields are optional; `output.format` and `output.group_dirs` default to
the configuration. The rules of the selection, the dry run and the response are in [Write tools](#write-tools).

#### The target

`to` is a template of the new path of every selected entry, relative to the working directory of the entry (of a
result: the working directory of its `find`), with `/` as the separator on every platform. Its placeholders are the
parts of the old path relative to the working directory:

| Placeholder | Value                                          | `src/app/main.test.ts` | `README` | `.gitignore` |
|-------------|------------------------------------------------|------------------------|----------|--------------|
| `{dir}`     | the directory; empty in the working directory  | `src/app`              |          |              |
| `{name}`    | the name                                       | `main.test.ts`         | `README` | `.gitignore` |
| `{stem}`    | the name without the last extension            | `main.test`            | `README` | `.gitignore` |
| `{ext}`     | the last extension without the dot, or empty   | `ts`                   |          |              |
| `{path}`    | the whole path                                 | `src/app/main.test.ts` | `README` | `.gitignore` |

- a name which starts with a dot and has no other dot has no extension; a directory has the same placeholders
- `{{` and `}}` are literal braces
- `pattern` — a regex ([Rust syntax](QUERY.md#regex), like `=~`) on the old path relative to the working directory,
  with `/`: the whole path must match (`^` and `$` are implied), case-sensitive unless it has `(?i)`; its groups are
  `{1}`, `{2}`, ... and its named groups `{word}` in `to`; a group which didn't take part in the match is empty. An
  entry whose path doesn't match is skipped with `pattern_mismatch`
- after the placeholders are replaced, empty segments and `.` are dropped and `..` removes the segment before it,
  without following symlinks: `{dir}/{stem}.tsx` in the working directory is `main.tsx`. A target which is empty
  after that is skipped with `invalid_target`
- a target outside of the allowed roots or an allowed root itself is skipped with `permission_denied` (the part of
  it which exists is canonicalized); `..` may leave the working directory as long as the target stays inside
- a target which is the old path is not renamed: the entry is listed as `unchanged`
- the missing directories of a target are created (not in a dry run)

Errors of the template and the pattern, before anything is changed, with the position in the field like the errors of
a query ([OUTPUT.md](OUTPUT.md#errors)):

| Error              | Example `to`       | Message                         | Hint                                                     |
|--------------------|--------------------|---------------------------------|----------------------------------------------------------|
| `invalid_template` | `{dir}/{base}.ts`  | `unknown placeholder 'base'`    | `the placeholders are dir, name, stem, ext, path` and `; the groups of 'pattern' are 1, 2, word`, or `; groups need a 'pattern'` |
| `invalid_template` | `{3}.rs`           | `the group 3 doesn't exist`     | the same                                                 |
| `invalid_template` | `{}.rs`            | `empty placeholder`             | the same                                                 |
| `invalid_template` | `{name.bak`        | `unclosed '{'`                  | `write '{{' for a brace`                                 |
| `invalid_template` | `name}.bak`        | `unmatched '}'`                 | `write '}}' for a brace`                                 |
| `invalid_template` | `/tmp/{name}`      | `the target must be relative`   | `the target is relative to the working directory`        |
| `invalid_template` | `archive/`         | `the target ends with '/'`      | `end it with a name, e.g. 'archive/{name}'`              |
| `invalid_regex`    | `pattern = "(a"`   | `invalid regex`                 | the error of the regex, as for `=~`                      |

The position of an unknown placeholder or group is the placeholder with its braces, of an unclosed `{` from it to the
end, of `}` and `/` the character; a regex error marks the whole pattern. In JSON the field of the position is `to` or
`pattern` instead of `query`. `to` which is not a non-empty string is `'to' must be a non-empty string`, `pattern`
which is not a string `'pattern' must be a string`, a named group with the name of a placeholder
`the group 'name' of 'pattern' has the name of a placeholder` (all `invalid_option`); without `to`:
`error[invalid_option]: 'to' is required`, hint `the new path, e.g. '{dir}/{stem}.bak'`.

#### Conflicts

The whole plan is checked before anything changes. An entry conflicts when its target:

| Reason        | Text                                  | When                                                                 |
|---------------|---------------------------------------|----------------------------------------------------------------------|
| `exists`      | `exists`                              | something exists at the target (a symlink is not followed) which is not an entry of the selection |
| `same_target` | `the same target as another file`     | another entry of the selection has the same target                   |
| `selected`    | `the target is another selected file` | the target is the old path of another entry of the selection         |
| `directory`   | `a directory is never replaced`       | with `overwrite`: the target is a directory, or the entry is a directory and the target exists |

`on_conflict` (default `fail`; another value is `'on_conflict' must be one of: fail, skip, overwrite`):

- `fail`: a conflict fails the call with `rename_conflict`, nothing is renamed, also in a dry run
- `skip`: the conflicting entries are skipped with `target_conflict`, the others are renamed
- `overwrite`: an `exists` conflict with a file or a symlink is resolved: the existing entry is moved to the trash
  (like `delete`) and the entry takes its place, listed as `replaced`; the other conflicts fail the call like `fail`

The entries skipped before (archive members, changed files, `pattern_mismatch`, ...) and the unchanged ones don't
conflict. The error lists the conflicting entries in path order, at most 10, then `… N more`; the paths are written
like in the response, the targets like their entries:

```
error[rename_conflict]: targets conflict; nothing was changed
  src/a.ts -> src/a.tsx: exists
  src/b.ts -> out/x.ts: the same target as another file
  src/c.ts -> out/x.ts: the same target as another file
  hint: on_conflict = "skip" renames the other files, "overwrite" replaces the existing ones
```

With `overwrite` the hint is `on_conflict = "skip" renames the other files`. In JSON the error has every conflict,
after `message`: `"conflicts": [{"path": "src/a.ts", "to": "src/a.tsx", "reason": "exists"}, ...]`.

#### Result

The [response of a write tool](#response-of-a-write-tool), counted as `renamed`, with `unchanged: N` after it when
it is not 0. Every entry is written as `<old> -> <target>`, the target as the path of the entry would be written: from
the working directory, with its prefix in the response (`dirs: ["src"]`: `src/a.ts -> src/b.ts`), a directory ending
with the separator; then `  replaced` for a replaced target, or the entry is `<old>  unchanged`:

```
renamed: 2  unchanged: 1

src/
a.ts -> src/a.tsx
b.ts -> src/b.tsx  replaced
c.tsx  unchanged
```

- the entries are renamed in path order; within one file system with an atomic rename, across file systems (the
  operating system refuses the rename) by a copy, then the source is deleted permanently; a failed copy is removed and
  leaves the source untouched
- a symlink is moved itself; a relative symlink keeps its text, so it may point elsewhere after the move
- a target which appeared after the check (it exists when the entry is renamed, without `overwrite`) is skipped with
  `target_conflict`, never replaced
- with `overwrite`, where there is no trash for the existing target the entry is skipped with `trash_unavailable`
- an unchanged entry counts as done for `isError`
- JSON: `"renamed"`, then `"unchanged"` when not 0; every entry has `"to"` after `"path"`, `"replaced": true` for a
  replaced target, an unchanged entry `"unchanged": true` instead of `"to"`

```json
{
  "version": 1,
  "renamed": 2,
  "unchanged": 1,
  "files": [
    { "path": "src/a.ts", "to": "src/a.tsx" },
    { "path": "src/b.ts", "to": "src/b.tsx", "replaced": true },
    { "path": "src/c.tsx", "unchanged": true }
  ]
}
```

### copy

#### Description

```
Copies files: every file gets a copy at a path made from a template. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
to = "...": the path of the copy, relative to the working directory, / as the separator. Placeholders:
  {dir} {name} {stem} {ext} {path}   parts of the old path: src/a.test.ts = src, a.test.ts, a.test, ts, src/a.test.ts
  {1} {2} {word}                     groups of pattern; {{ and }} are braces
  e.g. "{dir}/{stem}.bak.{ext}", "backup/{path}"
pattern = "regex": must match the whole old path, e.g. "src/(.*)\.rs" with to = "old/{1}.rs".
  A file which doesn't match is skipped with a warning.
Missing directories are created. The query selects files only, like find: type = dir selects directories, copied
  with everything in them; type = link symlinks, copied as symlinks.
The content, the modification time and the permissions are kept; a failed copy leaves nothing behind.
on_conflict: a target which exists, two files to one target, a target which is another selected file:
  "fail" (default): nothing is copied, the error lists them; "skip": those files are skipped;
  "overwrite": an existing file is moved to the trash and replaced, a directory never.
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be copied and changes nothing. Use it for a selection you haven't seen.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "required": ["to"],
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
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
    },
    "to": { "type": "string", "minLength": 1, "description": "The path of the copy, relative to the working directory: {dir} {name} {stem} {ext} {path}, groups {1} {word} of pattern." },
    "pattern": { "type": "string", "description": "Regex on the old path, the whole path must match; its groups are {1}, {2}, {name} in to." },
    "on_conflict": { "enum": ["fail", "skip", "overwrite"], "description": "A target which exists or collides. Default: fail." },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "dry_run": { "type": "boolean", "description": "List what would be copied, change nothing." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" }
      }
    }
  }
}
```


`to` is required; `result` and `query` exclude each other, one of them is required; `dirs`, `traversal`, `kinds` and
`languages` work only with `query`. The other fields are optional; `output.format` and `output.group_dirs` default to
the configuration. The rules of the selection, the dry run and the response are in [Write tools](#write-tools).

The target is made like the one of `rename` ([The target](#the-target)): the same placeholders, `pattern`, errors
(`invalid_template`, `invalid_regex`, `invalid_option`) and warnings (`pattern_mismatch`, `invalid_target`,
`permission_denied`); a target which is the old path is not copied, the entry is listed as `unchanged`. The conflicts
are the ones of `rename` ([Conflicts](#conflicts)) and one more:

| Reason   | Text                                        | When                                                 |
|----------|---------------------------------------------|------------------------------------------------------|
| `inside` | `the target is inside the copied directory` | the entry is a directory and the target is inside it |

`overwrite` doesn't resolve it. A conflict which isn't resolved fails the call with `copy_conflict`, nothing is
copied, also in a dry run; the error is written like `rename_conflict`, with the hint
`on_conflict = "skip" copies the other files, "overwrite" replaces the existing ones` (with `overwrite`:
`on_conflict = "skip" copies the other files`):

```
error[copy_conflict]: targets conflict; nothing was changed
  src/a.ts -> src/a.bak.ts: exists
  hint: on_conflict = "skip" copies the other files, "overwrite" replaces the existing ones
```

#### Result

The [response of a write tool](#response-of-a-write-tool), counted as `copied`, with `unchanged: N` after it when it
is not 0; the entries are written like the ones of `rename` ([Result](#result-4)): `<old> -> <target>`, then
`  replaced` for a replaced target, or `<old>  unchanged`:

```
copied: 2

src/
a.ts -> src/a.bak.ts
b.ts -> src/b.bak.ts  replaced
```

- the entries are copied in path order; the source is never changed
- a file keeps its content byte for byte, its modification time and its permissions as far as the platform keeps them;
  `traversal.max_file_size` doesn't limit the copy
- a directory is copied with its whole tree, every file, directory and symlink in it, also the hidden and ignored
  ones: the traversal options of the request don't apply inside it; the directories keep their modification times
- a symlink is copied as a symlink with the same text, never followed; it keeps its modification time where the
  platform can set it
- the copy is written under a temporary name next to the target (a name starting with `.` and ending with `.tmp`), a
  directory with its whole tree, then renamed into place: a failed copy is removed and leaves no partial target
- a target which appeared after the check (it exists when the copy is renamed into place, without `overwrite`) is
  skipped with `target_conflict`, never replaced; with `overwrite` the existing target is moved to the trash after
  the copy is written, right before it is renamed into place, and where there is no trash the entry is skipped with
  `trash_unavailable`
- an unchanged entry counts as done for `isError`
- JSON: `"copied"`, then `"unchanged"` when not 0; the entries like the ones of `rename`

```json
{
  "version": 1,
  "copied": 2,
  "files": [
    { "path": "src/a.ts", "to": "src/a.bak.ts" },
    { "path": "src/b.ts", "to": "src/b.bak.ts", "replaced": true }
  ]
}
```

### replace

#### Description

```
Replaces text in files: the matches of a group of the query get a new text made from a template. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, searched again with the query of that find
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
replace = {"@old": "template"}: the matches of the group @old are replaced; a group is a condition, line( ... ),
  near( ... ) or ( ... ) @name of the query. The other conditions only select the files.
  {@old} the match, {@old.1} {@old.fname} the groups of a =~ regex; {{ and }} are braces
  e.g. query "text =~ `fn (?<f>\w+)_old\(` @def", replace {"@def": "fn {@def.f}_new("}
Only the matches which made the file match are replaced: in line( ... ) and near( ... ) the matched lines;
  matches skipped by /c stay. /n writes the new text in the naming style of each match: 'parse query' /n to
  'build request' turns parseQuery into buildRequest, PARSE_QUERY into BUILD_REQUEST. /e escapes it like the match.
The encoding, the BOM, the line breaks and the permissions of a file are kept.
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
expect = N: fails without changing anything unless the selection has N files; expect_matches = N: N matches.
dry_run = true: shows the changed lines and writes nothing. Use it for a selection you haven't seen.
```

#### Input schema

```json
{
  "type": "object",
  "additionalProperties": false,
  "required": ["replace"],
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
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
    },
    "replace": {
      "type": "object", "minProperties": 1,
      "description": "Group of the query -> template of the new text, e.g. {\"@old\": \"new_{@old.1}\"}.",
      "additionalProperties": {
        "anyOf": [
          { "type": "string" },
          {
            "type": "object",
            "additionalProperties": false,
            "required": ["template"],
            "properties": {
              "template": { "type": "string" },
              "naming": { "type": "boolean", "description": "false: a /n match gets the template as it is." }
            }
          }
        ]
      }
    },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "expect_matches": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless this many matches are replaced." },
    "dry_run": { "type": "boolean", "description": "Show the changed lines, write nothing." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" },
        "max_lines": { "type": "integer", "minimum": 1, "description": "Max changed lines shown." }
      }
    }
  }
}
```

`replace` is required; `result` and `query` exclude each other, one of them is required; `dirs`, `traversal`, `kinds`
and `languages` work only with `query`. The other fields are optional; `output.format`, `output.group_dirs` and
`output.max_lines` default to the configuration. The rules of the selection, the dry run and the response are in
[Write tools](#write-tools).

With `result` the files of the result are searched again with the `find` request of the result, like `within`
([OUTPUT.md](OUTPUT.md#result-ids)): its query, `traversal`, `kinds`, `languages`, `escape` and `query_options`; the
groups of `replace` are the groups of that query. A file of the result which doesn't match it any more has no matches.

#### What is replaced

`replace` maps a group of the query to the template of its new text. A key is a [group
reference](QUERY.md#group-references) to a whole group: `@name` or `#3`, a condition, `line( ... )`, `near( ... )` or
a bracket group `( ... ) @name`. A value is the template, or an object `{"template": "...", "naming": false}`.

The matches of a group in a file are its fragments ([OUTPUT.md](OUTPUT.md#fragments)): the fragments of the file whose
ids have the group.

- only the matches which made the file match: the matches of a `text` condition inside `line( ... )` or
  `near( ... )` only in the lines which matched there; a line of `line( ... )` without a text match is replaced as a
  whole (`line(len > 120) @long`, without its line break); a condition under `not` has no matches
- a match which `/c` skips (it starts in a comment) is not a match and stays
- a group of the query which is not a key of `replace` stays as it is; it only selects the files
- matches of one group which overlap: the one which starts first is replaced, at the same start the longer one; the
  other one stays
- matches of two keys which overlap (also a group and a condition inside it): the file is skipped with
  `replace_conflict`, nothing in it is changed
- a selected entry without a match of the keys (a file selected by another branch of `or`, a directory, a symlink,
  a binary file) is listed as `unchanged`; a symlink is never written through, also with
  `traversal.follow_symlinks`

Errors of `replace`, before anything is changed (`invalid_option` unless named):

| Error | When | Message | Hint |
|-------|------|---------|------|
| `invalid_option` | no `replace` | `'replace' is required` | `the groups of the query and their new text, e.g. {"@old": "new"}` |
| `invalid_option` | not an object, or empty | `'replace' must be an object of groups and templates` | the same |
| `invalid_option` | a value which is not a string or `{template, naming}` | `'replace.@old' must be a template or {"template": "...", "naming": false}` | |
| `unknown_group` | `@x` for `text has 'a' @a` | `unknown group '@x'` | `the groups of the query: @a` |
| `invalid_option` | `@def.1` | `'@def.1' is a sub-group; replace the whole group` | `e.g. {"@def": "...{@def.1}..."}` |
| `invalid_option` | `@src` of `(name ~ '*.rs') @src` | `the group '@src' has no text condition` | `replace changes the text of files; rename changes their names` |
| `invalid_option` | `expect_matches` not a whole number | `'expect_matches' must be a whole number` | |

The keys are checked in the order of their names (a JSON object has no order). The position of `unknown_group` is the key; in JSON the field of
the position is `replace` with the key as its text. `expect_matches: N` fails the call with `count_mismatch` when the
files which are not skipped have another number of matches to replace, before anything is written:
`error[count_mismatch]: the selection has 5 matches, expected 4`, hint
`nothing was changed; check the selection with dry_run: true`.

#### Templates

| Part               | Value                                                                                   |
|--------------------|-----------------------------------------------------------------------------------------|
| `{@old}`, `{#3}`   | the text of the match of the group                                                      |
| `{@old.1}`, `{@old.fname}` | a capture group of the match of a `text =~` condition, empty when it didn't take part |
| `{{`, `}}`         | a literal brace                                                                         |
| a line break       | the line break of the file: its first one, `\n` in a file without one                   |

A template refers only to the group of its key and to its capture groups: a match of another group is not tied to this
one. Errors (`invalid_template`), with the position in the template, in JSON the field `replace` with the template as
its text:

| Example `@old`       | Message                                   | Hint                                                     |
|----------------------|-------------------------------------------|----------------------------------------------------------|
| `{@x}_v2`            | `'{@x}' is not '@old' or its sub-group`   | `the placeholders of '@old' are {@old}, {@old.1}, {@old.fname}` |
| `{name}`             | `unknown placeholder 'name'`              | the same                                                 |
| `{}`                 | `empty placeholder`                       | the same                                                 |
| `{@old`              | `unclosed '{'`                            | `write '{{' for a brace`                                 |
| `@old}`              | `unmatched '}'`                           | `write '}}' for a brace`                                 |

The hint lists the group and its capture groups like the hint of `unknown_group`. The position of a placeholder is
the placeholder with its braces, of an unclosed `{` from it to the end, of `}` the character.

**`/n`.** A match of a condition with `/n` gets the new text — the template with its placeholders filled in — in the
naming style of the match: the new text is split into words like the value of `/n`
([QUERY.md](QUERY.md#n--any-naming-style)) and written with

- the separator between the words of the match: `_`, `-`, `.` or a space; none when they are joined
- the case of the word of the match at the same position, after the last word of the match the case of its last
  word: `UPPER` (an upper case letter and no lower case one), `Capitalized` (an upper case first letter), `lower`
- a match of one word: `lower` writes the words in camelCase, `Capitalized` in PascalCase, `UPPER` joined by `_`

`'parse query' /n` with `build request`: `parseQuery` → `buildRequest`, `ParseQuery` → `BuildRequest`, `parse_query`
→ `build_request`, `PARSE_QUERY` → `BUILD_REQUEST`, `parse-query` → `build-request`. `"naming": false` writes the new
text as it is. A condition with `/i` gets the new text as it is: the case of the match is not adapted. A `naming`
which is not a boolean is `'replace.@old.naming' must be a boolean`.

**`/e`.** A match found on the escape level n of a style ([QUERY.md](QUERY.md#escaping)) gets the literal text of the
template escaped n times in that style, every character in its first form of the table of
[Styles](QUERY.md#styles) (`'` stays `'` in `backslash`, a space is `%20` in `url`); the text of the placeholders is
written as matched, it is escaped already.

#### Writing

- the files are planned in path order before anything is written: read, decoded with the encoding of the search, their
  matches replaced in memory; then `expect_matches` is checked, then they are written in path order
- a file whose text differs from the text of the search is skipped with `file_changed`
- a file keeps its encoding and its BOM; a file which has bytes invalid in its encoding (`decode_errors`), whose text
  doesn't encode back to the same bytes, or whose new text has a character which its encoding can't write is skipped
  with `unencodable`
- the line breaks of the unchanged text stay as they are (`\r\n` too); a new line break is the line break of the file
- a file is written to a hidden temporary file next to it (a name starting with `.` and ending with `.tmp`) with the
  permissions of the file, then renamed into place: a failed write leaves the file untouched and no temporary file;
  the file gets a new modification time
- a file whose new text is its old text (`{@old}` as the template) is listed as `unchanged`, it isn't written
- the timeout and a cancellation stop between files: while planning, nothing is written (the response has no entries
  and the `timeout` warning); while writing, the response lists the files written so far

#### Result

The [response of a write tool](#response-of-a-write-tool), counted as `replaced`: the files changed, then
`matches: N`, the matches replaced in them (also `matches: 0`), then `unchanged: N` when it is not 0. Every changed
file is written with its matches, then its changed lines like a diff: for every run of lines with replaced matches
(runs which touch are joined) the old lines `-<n>:<text>`, then the new lines `+<n>:<text>` with their numbers in the
new file; an unchanged file is `<name>  unchanged`:

```
replaced: 2  matches: 3  unchanged: 1

src/
a.rs  matches: 2
-3:use serde_json::Value;
+3:use simd_json::Value;
-9:    serde_json::to_string(&v)
+9:    simd_json::to_string(&v)
b.rs  matches: 1
-1:serde_json = "1"
+1:simd_json = "1"
c.rs  unchanged
```

- the lines are written as they are, without highlight, dedent or a limit of their length
- `output.max_lines` (default: the configuration, 500) is the most `-` and `+` lines of the response: the files after
  it are written without their lines, and the entries end with `… N more lines`
- an unchanged entry counts as done for `isError`
- JSON: `"replaced"`, `"matches"`, then `"unchanged"` when not 0; a changed file has `"matches"` and `"changes"`, each
  `{ "line", "old", "new_line", "new" }` with the lines as arrays, an unchanged one `"unchanged": true`; the lines over
  `max_lines` are left out of `changes` and counted in `"more_lines"` after `files`

```json
{
  "version": 1,
  "replaced": 1,
  "matches": 2,
  "files": [
    {
      "path": "src/a.rs",
      "matches": 2,
      "changes": [
        { "line": 3, "old": ["use serde_json::Value;"], "new_line": 3, "new": ["use simd_json::Value;"] },
        { "line": 9, "old": ["    serde_json::to_string(&v)"], "new_line": 9, "new": ["    simd_json::to_string(&v)"] }
      ]
    }
  ]
}
```

## Write tools

Tools which change files: `delete`, `rename`, `copy`, `replace`. They share how they are enabled, how they select files, the dry run and the
response.

### Enabling

- `server.allow_write` (default `false`, read-only: only the config file and `--allow-write` set it, see
  [CONFIG.md](CONFIG.md#server)); `update_config` can't change it (`read_only`)
- with `allow_write = false` the write tools are not listed in `tools/list`, and a call made anyway fails with
  `error[write_disabled]: write tools are disabled`, hint `the configuration has server.allow_write = false`
- the server instructions and the tools which only read don't change: `find` stays `readOnlyHint: true`

### Selection

Exactly one of:

- `result: "r12"` — the files of a kept `find` result: all its matched entries, not only the shown ones, see
  [OUTPUT.md](OUTPUT.md#result-ids); the paths are the ones of the result. A result id which is not like `r12` is
  `'result' must be a result id like 'r12'`, an unknown or evicted one `result_expired` as for `within`; with
  `server.result_cache = 0` only `query` works
- `query` with `dirs`, `traversal`, `kinds`, `languages` and `version` — the same fields and meaning as in `find`; the
  query is evaluated like a `find` request and selects all the entries it matches, by default files only (`type = dir`
  selects directories, `type = link` symlinks, see [QUERY.md](QUERY.md#files-and-directories)): `name ~ '*.tmp'`
  doesn't select a symlink `link.tmp`, `name ~ '*.tmp' and type in (file, link)` does; the descriptions of `delete`,
  `rename` and `copy` say so. `traversal.follow_symlinks` is not the way to select a symlink: it follows every
  symlink of the walk

Both are `error[invalid_option]: 'result' and 'query' exclude each other`; neither is
`error[invalid_option]: 'result' or 'query' is required`, hint `pass the result id of a find, or a query`; `dirs`,
`traversal`, `kinds` or `languages` with `result` is `error[invalid_option]: '<field>' works only with 'query'`.
An unknown field is `invalid_option` with the expected fields, like in `find`.

- `expect: N` — the call fails with `count_mismatch` when the selection has another number of entries, before
  anything is changed: `error[count_mismatch]: the selection has 3 files, expected 2`, hint `nothing was changed;
  check the selection with dry_run: true`. The number counts the selected entries as `files:` of `find` does, a
  directory as one; a value which is not a whole number is `'expect' must be a whole number`
- the entries are acted on in path order, each one at most once; an entry inside a selected directory goes with the
  directory and is not acted on itself
- an entry of a result which changed since the result (another size or modification time) or no longer exists is
  skipped with `file_changed`, never acted on
- every path is checked against `server.allowed_roots` when the tool acts on it: its directory is canonicalized, its
  name is not resolved; outside of the roots it is skipped with `permission_denied`; an allowed root itself is never
  changed (with nested roots it is inside another one), it is skipped with `permission_denied` too
- a member of an archive can't be changed: it is skipped with `archive_member`
- symlinks are acted on themselves, never followed; a directory (`type = dir`) is acted on as a whole tree
- when the search of `query` stops at the timeout, nothing is changed: the response has no entries and the `timeout`
  warning

### Dry run

- `dry_run: true` checks the selection like the real call (`expect`, changed files, archives, allowed roots) and
  changes nothing; the response is the same as the real one, with `dry run` at the end of the header
  (`"dry_run": true` in JSON). Whether the trash is available and whether an entry can be changed are known only when
  acting
- `dry_run` defaults to `false`; the tool description tells the agent to use it for a selection it hasn't seen
- `dry_run` must be a boolean: `'dry_run' must be a boolean`

### Response of a write tool

The text format: a header with the counts, then the entries acted on, then the warnings, each part after an empty
line:

```
deleted: 3  skipped: 1

build/
cache/
src/
legacy.rs
old.rs

warning: file_changed (1): src/main.rs
```

- the header: the count of the tool (`deleted: N`, also `deleted: 0`; `rename` and `copy` add `unchanged: N`, `replace`
  adds `matches: N` and `unchanged: N`), then `skipped: N` (the entries skipped with a
  warning) and `failed: N` (the entries which failed) when they are not 0, then `permanent` for `permanent: true`, then
  `dry run` for a dry run
- the entries in path order, grouped by directory like `find` with `output.group_dirs`
  ([OUTPUT.md](OUTPUT.md#directory-groups)), directories ending with the separator; the paths are written like the
  paths of `find`; an empty selection has no entries part
- an entry which fails (permission, busy, vanished, ...) doesn't stop the others: it is listed with the kind of the
  error of the operating system, `old.rs  failed: permission denied`
- the skipped entries are only in the warnings ([OUTPUT.md](OUTPUT.md#warnings)), with their codes
- `isError` is true when nothing was done of a selection with entries: every entry was skipped or failed, or the
  timeout stopped the call before the first one; an empty selection is not an error, a call which did something
  isn't either
- the timeout of the request (`timeout`, else `server.timeout`, counted from the arrival of the call) and a
  cancellation stop between entries: the response lists what was done, with the `timeout` warning after a timeout; a
  cancelled call gets no response, see [Cancellation](#cancellation)

With `output.format = "json"` (`group_dirs` doesn't apply, the paths are full):

```json
{
  "version": 1,
  "deleted": 2,
  "skipped": 1,
  "failed": 1,
  "dry_run": true,
  "files": [
    { "path": "build/cache/" },
    { "path": "src/legacy.rs" },
    { "path": "src/old.rs", "error": "permission denied" }
  ],
  "warnings": [ { "code": "file_changed", "count": 1, "paths": ["src/main.rs"] } ]
}
```

Empty and default values are omitted: no `skipped` and `failed` when 0, no `permanent` and `dry_run` when false
(`"permanent": true` comes before `dry_run`), no `files` and `warnings` when there are none. With `format = "json"` an
error is the JSON document of [OUTPUT.md](OUTPUT.md#errors).

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
