# Task 129: `replace` tool

Status: implemented, see [129__SUMMARY__REPLACE_TOOL.md](129__SUMMARY__REPLACE_TOOL.md)

Depends on: task 125, task 128

## Goal

An agent can replace text in the files it found in one call, without a shell or a file edit per file: the matches of
a [group](128__TASK__QUERY_GROUPS.md) of the query are replaced by a template which can use the matched text.

```json
{
  "query": "lang = rust and (text has 'serde_json' /w) @old and text has 'apple'",
  "replace": { "@old": "simd_json" }
}
```

replaces `serde_json` only in the Rust files which also have `apple`; `apple` stays.

## Scope

A spec change first (its own commit; TOOLS.md, OUTPUT.md), then the code. Enabling, selecting files (`result` or
`query`), `expect`, `dry_run`, the response and the errors are the shared rules of
[task 125](125__TASK__DELETE_TOOL.md).

### What is replaced

- `replace` — an object: a group reference (`@name`, `#3`) → a template; the matches of that group are replaced
- only the matches which made the file match: inside `line( ... )` / `near( ... )` only the matches in the fragments
  which matched; matches skipped by `/c` are not matches and stay
- a group which is used in the query but not in `replace` stays as it is
- overlapping matches of two replaced groups: `replace_conflict` for that file, the file is not changed
- `expect_matches: N` — optional, like `expect` but for the number of replaced matches

### Templates

- `{@name}`, `{#3}` — the matched text of a group; `{@name.1}`, `{@name.fname}` — a regex sub-group of the same
  match ([group references](128__TASK__QUERY_GROUPS.md#group-references)); `{{` and `}}` for literal braces
- a reference to a group which is not the replaced one or one of its sub-groups: `invalid_template` (a match of another
  group is not tied to this one)
- for a group of `/n` conditions (any naming style) the replacement is written in the style of each match:
  `'parse query' /n` → `'build request'` turns `parseQuery` into `buildRequest` and `PARSE_QUERY` into
  `BUILD_REQUEST`; `/n` can be turned off per group with `"@g": { "template": "...", "naming": false }`
- for a group of `/i` conditions the case of the match is not adapted: the template is written as it is

### Writing files

- the file keeps its encoding, BOM and line endings; a replacement which the encoding can't write: the file is
  skipped with `unencodable`
- a file is written to a temporary file next to it and renamed into place; permissions are kept
- with `result`: a file whose matched lines changed since the result (the fingerprint of the result) is skipped
- files inside archives and binary files are skipped with a warning
- `/e` matches (escaped text) are replaced by the template escaped the same way and at the same level
- annotations: `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: false`, `openWorldHint: false`

### Response

- the header: files changed, matches replaced, files skipped
- per file the changed lines before and after, like a diff (`-` / `+` with line numbers), within `output.max_lines`
  and the other limits; `dry_run` shows the same without writing

## Out of scope

- replacing in file names and paths: use `rename` ([task 126](126__TASK__RENAME_TOOL.md))
- multi-line replacement across `/m` regex matches beyond what the match covers
- an undo; the agent can run `dry_run` first or use git

## Spec

- [TOOLS.md#tools](../TOOLS.md#tools) — the tool, its description, input schema, annotations
- [QUERY.md#labels-and-condition-ids](../QUERY.md#labels-and-condition-ids) — groups
- [OUTPUT.md#errors](../OUTPUT.md#errors) — `replace_conflict`, `invalid_template`, `unencodable`

## Examples

`129__EXAMPLES__REPLACE_TOOL.md` covers:

- the example of the goal: only the group is replaced, the other condition stays
- a regex with named groups in the template (`fn {@def.fname}_v2(`)
- `/n` keeping the naming style; `/i`
- matches in `line( ... )`: only the matched fragments; `/c`: comments untouched
- overlapping groups; an invalid template; an unknown group
- a file changed since the result; an encoding which can't write the replacement; a UTF-16 file with a BOM; CRLF
- `/e` matches inside stringified JSON
- `dry_run`; `expect_matches`; the JSON format

## Acceptance criteria

- the spec describes the tool before the code is written
- every example in `129__EXAMPLES__REPLACE_TOOL.md` passes as a test; the examples of all earlier tasks still pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
