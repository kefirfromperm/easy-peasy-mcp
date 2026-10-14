# Summary of task 129: `replace` tool

An agent can replace text in the files it found in one call: the matches of a group of the query get a new text made
from a template which can use the matched text and the groups of a regex. The other conditions only select the files.

```json
{ "query": "lang = rust and (text has 'serde_json' /w) @old and text has 'apple'", "replace": { "@old": "simd_json" } }
```

It is the fourth write tool; the selection, `expect`, `dry_run` and the response frame are the shared rules of
task 125.

## What was built

- **replace** ([TOOLS.md#replace](../TOOLS.md#replace)): listed with `server.allow_write` only, after `copy`;
  `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: false`, `openWorldHint: false`
- **The selection**: `query` like the other write tools; `result` searches the files of the result again with the
  `find` request of the result (query, traversal, kinds, languages, escape, query_options) through `within`, so the
  groups are the ones of that query
- **What is replaced** ([TOOLS.md#what-is-replaced](../TOOLS.md#what-is-replaced)): a key is a whole group (`@name`,
  `#3`): a condition, `line( ... )`, `near( ... )` or a bracket group; its matches are the fragments of the file with
  its id, so `line( ... )` / `near( ... )` give only the matched lines and `/c` skips comments. Overlaps in one group:
  the first start, then the longer one; between two keys: `replace_conflict`, the file is skipped
- **Errors**: `'replace' is required`, `'replace' must be an object of groups and templates`, a wrong value, a wrong
  `naming`, `unknown_group` positioned in the key, a sub-group as a key, a group without a text condition,
  `'expect_matches' must be a whole number`; `count_mismatch` for `expect_matches` before anything is written
- **Templates** ([TOOLS.md#templates](../TOOLS.md#templates)): `{@old}`, `{@old.1}`, `{@old.fname}`, `{{`, `}}`;
  `invalid_template` for another group, an unknown or empty placeholder, an unclosed `{`, an unmatched `}`, with the
  placeholders of the group as the hint
- **`/n`**: the new text in the naming style of each match (separator and the case of every word; one word gives
  camelCase, PascalCase or `UPPER_SNAKE`); `"naming": false` turns it off. **`/i`**: the template as it is.
  **`/e`**: the literal parts of the template escaped on the level of the match in its first forms
- **Writing** ([TOOLS.md#writing](../TOOLS.md#writing)): every file planned before anything is written; the text must
  be the text of the search (`file_changed`); the encoding and the BOM kept, a file with invalid bytes, one which
  doesn't round-trip or a new character the encoding can't write skipped with `unencodable`; `\r\n` line breaks kept,
  a new line break written like the first one of the file; a temporary file next to the file with its permissions,
  renamed into place; a symlink, a directory or a file without matches is `unchanged`, never written through
- **Response**: `replaced: N  matches: N  unchanged: N ...`, per changed file `matches: N` and the runs of changed
  lines `-<n>:` / `+<n>:` with the new line numbers; `output.max_lines` cuts them with `… N more lines`; JSON with
  `matches`, `changes` (`line`, `old`, `new_line`, `new`) and `more_lines`
- **Warnings** `replace_conflict` and `unencodable` in OUTPUT.md; the spec of the write tools, CONFIG.md, README.md and
  QUERY.md name the tool

## Architecture

- `src/replace.rs` (new): `parse` (`Spec`), `result_request`, `Template`, `compile` (`Rule`), `replacements`,
  `plan` (`Planned`, `Plan`), `apply_raw` (normalized offsets → raw text), `changes` (the diff runs), `replace` (plan,
  `expect_matches`, write), `write_file`
- `src/content.rs`: `decode_raw` (`Raw`: the text before the line breaks are normalized, the encoding, the BOM, the
  errors) and `encode`; `decode` builds on `decode_raw`
- `src/query`: `TextPattern` got `naming` and `escape_style`; `Filter::find` and `Filter::has_text`;
  `Query::sub_group_names`; `escape::escape_first`; `naming::restyle`
- `src/write.rs`: `parse` takes the output fields of the tool; `Acted` got `matches` and `changes`, `Outcome` got
  `max_lines` (set only by `replace`), the text and JSON formats write them
- `src/engine.rs`: `try_replace`, `replace` in `TOOLS` and `WRITE_TOOLS`; `src/description.rs`, `src/mcp.rs`: the
  description, the schema and the annotations; `src/warning.rs`: the two codes

A request flows: `mcp` → `Engine::call_with("replace")` → `write::parse` → `replace::parse` → the query parsed and
`replace::compile` → `Engine::run` (with `within` for a result) → the matches taken out of the found files →
`write::check_expect` → `write::select` → `replace::replace` → `Outcome::to_text` / `to_json`.

## Deviations from the plan and the task

- the keys are checked in the order of their names, not of the request: a JSON object has no order and `serde_json`
  sorts it (a spec fix of its own)
- a file of a result is checked by its stamp (the shared rule of task 125) and, like a file of `query`, by its text
  against the search, instead of the fingerprint of the matched lines which the task sketch named
- a binary file is not skipped with a warning: its content is unknown, it has no matches and is `unchanged`; with
  `binary = search` its text is written like any other if it round-trips
- `output` of `replace` takes `max_lines` only; the other limits of `find` don't apply to the changed lines

## Tests

- examples E129.1–E129.22 (`tests/startup.rs`; E129.2 in `tests/examples.rs`): the tool list, without writing, the
  example of the task, a named regex group, `/n`, `/i`, `naming: false`, `line( ... )`, `/c`, two groups, overlapping
  groups, a template with another group, the errors of the keys, a result with a changed file, an unchanged file,
  `windows-1252` with an unencodable and an encodable replacement, UTF-16 with a BOM, CRLF, `/e`, `expect_matches`,
  `max_lines`, JSON
- E125.3, E126.1, E127.1 list `replace`; `tests/mcp_stdio.rs` checks the seven tools against TOOLS.md
- unit tests: `replace::tests` (parse errors, templates, escaping, overlaps, CRLF, diff runs, encodings, the written
  file), `write::tests::changed_lines_of_replace_cut_by_max_lines`, `naming::tests::the_style_of_the_match`,
  `escape::tests::the_first_form_on_every_level`

## Limitations and known gaps

- the matches of a result are the ones of its query now: a configuration changed since the result (e.g. by
  `update_config`) may find other matches
- a lone `\n` in a file with `\r\n` line breaks can't be told from a normalized one: a multi-line match keeps the
  line breaks of the file in its new text
- the diff shows whole lines without a limit of their length
- `output.groups` (task 130) will show the groups in `find`

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --allow-write
```

```json
{"query": "name ~ '*.rs' and text =~ `fn (?<f>\\w+)_old\\(` @def", "replace": {"@def": "fn {@def.f}_new("}, "dry_run": true}
```

## Version

`0.131.0`
