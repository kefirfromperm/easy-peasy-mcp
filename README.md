# Easy-Peasy MCP Server
The main goal is to allow to an agent to find a file or files with complex criteria.

I will use RUST

Candidate crates: `ignore` (parallel directory walking, `.gitignore` support), `grep-regex` / `grep-searcher` (fast
content search, binary detection), `globset` (wildcards).

## Commands

- find
- get config
- update config

## Request
The request contains fields

- working directory
- traversal options
- search criteria
- output format

### Working directory
One or more paths to search in. Every path must be inside one of the allowed roots (see Configuration).

### Traversal options

- min depth / max depth
- follow symlinks (default: no; loops are detected, symlinks can't escape the allowed roots)
- include hidden files (default: no)
- respect ignore files: `.gitignore`, `.ignore`, global git excludes (default: yes)
- exclude paths: wildcards for directories/files that are pruned and never walked (e.g. `target/`, `node_modules/`)
- binary files: skip / search / list only (default: skip for content conditions)
- max file size to scan content (bigger files are matched only by metadata conditions)

### Search criteria

Search criteria is a query with multiply conditions which can be combined with AND, OR, XOR and NOT logical operators.

The query is a JSON expression tree, so it maps directly to the MCP tool input schema, e.g.

```json
{
  "and": [
    { "name": { "wildcard": "*.rs" } },
    { "modified": { "after": "-7d" } },
    { "not": { "path": { "contains": "/tests/" } } },
    { "line": { "and": [
      { "content": { "contains": "unwrap", "case_sensitive": false } },
      { "not": { "content": { "starts_with": "//" } } }
    ] } }
  ]
}
```

#### Condition scope

Every condition has an explicit scope:

- **file scope** — conditions on file metadata and on the whole file content. `file content contains A AND file content
  contains B` means A and B can be on different lines. `NOT file content contains X` means files without a match
  (like `grep -L`).
- **line scope** — a `line { ... }` group inside a file-level expression. All conditions inside the group must be true
  for the same line. `NOT` inside the group means lines without a match (like `grep -v`). A file matches the `line`
  group if at least one of its lines matches it.

#### Condition targets

A condition can be applied on

- file path
- file name with extension
- file type: file, directory, symlink
- file modification date, creation date, access date
- file size
- empty (empty file or empty directory)
- file content (file scope)
- line length (line scope)
- line content (line scope)
- proximity: content A within N lines of content B (file scope)

Directories can be in the result only via metadata conditions (path, name, type, dates, empty). Content conditions
never match a directory.

#### Condition operators

- exact match
- contains
- starts with
- ends with
- regex
- wildcard (for the file path and file name with extension)
- algebraic operators for sizes and dates: `=`, `!=`, `<`, `<=`, `>`, `>=`, between
- dates are absolute (ISO 8601) or relative (`-2h`, `-7d`)
- sizes accept units (`10K`, `5M`)

Text operator options

- case sensitive (default: yes)
- whole word
- multiline (regex may match across lines; for file scope content only)

Escape operators can be applied for the string content in conditions

Exclude block to allow to support comments in programming languages: matches inside comments (and optionally string
literals) are ignored. Comment syntax is defined per file extension in the configuration (line comment like `//`, `#`,
block comment like `/* */`). Built-in defaults exist for common languages.

#### Encoding

Files are read as UTF-8. Invalid byte sequences are replaced (lossy) and do not stop the search.

### Output format

Output can contain fields

- exists YES/NO (true if at least one file is found)
- files count
- line count
- matches count
- truncated YES/NO (true if any of the max limits is hit; counts above still contain the real totals)
- next cursor (to continue the search from the place where it was truncated)

Per file fields

- path (relative to the working directory or absolute)
- file type
- file size
- file modification date
- line count
- matches count
- matched fragments with
- matched fragments line numbers
- the whole lines of matched fragments with lines before and after with size how many lines before and after have
  to show

Also output format includes

- max file count
- max line count
- max matches count
- max matches per file
- sort by: path, modification date, size, matches count; ascending / descending
- path style: relative / absolute
- cursor (from the previous truncated response)

## Configuration

The configuration contains default values for

- allowed roots (the server never reads outside of them; `update config` can't widen them, they are set only in the
  config file or the command line at server start)
- working directory (by default it's only current working directory)
- traversal options
- output format
- comment syntax per file extension
- search timeout
