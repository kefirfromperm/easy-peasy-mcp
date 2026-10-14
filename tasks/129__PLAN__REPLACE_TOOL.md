# Plan of task 129: `replace` tool

## Design

A fourth write tool. The selection, `expect`, `dry_run`, the allowed roots and the response frame are the shared code
of `src/write.rs`; what is new is the plan of the new text of a file and writing it back.

### Request

- `write::parse` gets the fields of the tool's `output` as a parameter: `replace` adds `max_lines`, which is taken out
  of the `find` request of the selection
- `src/replace.rs` (new): `parse(arguments)` → `Spec { keys: Vec<Key>, expect_matches, max_lines }`, every `Key` with
  its reference as written, its template text and `naming`; the errors of `replace` and `expect_matches`
- `Spec::compile(&Query)` → `Vec<Rule>`: the key checked with `Query::group` (`unknown_group` positioned in the key),
  a sub-group rejected, a group without a text condition rejected (`Filter::has_text(id)` in `query::eval`), the
  template parsed against the key and its sub-groups (`invalid_template`)

### Selection with matches

- `query`: `Engine::run` as today; the `search::Found` files keep their `FileMatches` (content and fragments)
- `result`: the kept arguments of the result (query, traversal, kinds, languages, escape, query_options, version) with
  `within = rN` and the output of the selection → `Engine::run`; the entries are the result's entries with their
  stamps (`file_changed`), their matches looked up by shown path and member
- the query is parsed once before the search to check `replace`, so a wrong key fails before any file is read

### Plan of a file (`replace::plan_file`)

1. `write::check` (archive member, allowed roots, stamp of a result entry)
2. a symlink, a directory, a file without matches of the keys → `unchanged`
3. the matches: the fragments whose ids have a key; per key the overlaps resolved (first start, then longer); between
   keys an overlap → `replace_conflict`
4. read the bytes, `content::decode_raw` (new, the decode of `decode` without the CRLF normalization): the encoding, the
   BOM length, the raw text, the errors. Errors → `unencodable`. `Content::from_text(raw)` must have the text of the
   search → else `file_changed`
5. every match: the template filled in (`{@g}`, `{@g.1}` from the `SubMatch` of the fragment), line breaks of the
   template made the line break of the file, then `/n` (`naming::restyle(new, matched)`) or `/e`
   (`escape::escape_first(style, text, level)` on the literal parts) by the text pattern of the match's condition
   (`TextPattern` gets `naming: bool` and `escape_style: Option<Style>`)
6. the normalized offsets mapped to the raw text (`\r` removed before `\n` counted), the new raw text built
7. encoded back: UTF-8 as is, UTF-16/32 by hand, other encodings with `encoding_rs` (unmappable → `unencodable`); the
   old raw text must encode to the old bytes, else `unencodable`
8. the diff runs: the old lines touched by matches (a multi-line match spans its lines), consecutive lines joined; the
   new line numbers by the line breaks added or removed before the run

Then `expect_matches` against the sum over the planned files, then the writes in path order (unless `dry_run`), the
deadline checked between files: the new bytes to `.<name>.<pid>-<n>.tmp` next to the file with the permissions of the
file, then renamed into place; a failure removes the temporary file and lists the entry as failed.

### Response

`write::Acted` gets `matches: Option<usize>` and `changes: Vec<Change>`; `Outcome::to_text` / `to_json` with
`replace` write `matches: N` in the header and the changes per file, `max_lines` cutting the `-`/`+` lines and adding
`… N more lines` (JSON `more_lines`). The shared parts stay as they are for the other tools.

### Registration

`Engine::TOOLS` and `WRITE_TOOLS` get `replace`; `description::REPLACE_DESCRIPTION` and `replace_schema()` (the text of
TOOLS.md); `mcp.rs` annotations `readOnlyHint: false, destructiveHint: true, idempotentHint: false,
openWorldHint: false`; `ErrorCode` unchanged (the codes exist), `WarningCode` gets `ReplaceConflict` and `Unencodable`
in the order of OUTPUT.md.

## Decisions

- the files are planned completely before any write, so `expect_matches` can fail without a change; rejected: writing
  while planning
- a file is checked by its text, not only its stamp, between the search and the write, which covers `query` too
- the whole text is re-encoded and the old text must round-trip, rather than patching bytes: one code path for every
  encoding
- no new dependency: `encoding_rs` encodes, the temporary name of `rename::temporary_name` is reused

## Tests

- examples E129.1–E129.22 (`tests/startup.rs`; E129.2 in `tests/examples.rs`); E125.3, E126.1, E127.1 list `replace`
- `tests/mcp_stdio.rs`: the seven tools with `--allow-write`
- unit tests in `replace`: the parse errors; a template with braces, references and errors; the naming styles
  (camel, Pascal, snake, screaming, kebab, dotted, spaced, one word); escaping on levels 0–2; overlaps in a group and
  between keys; CRLF offsets; the diff runs with added and removed lines; encode round trips (UTF-8 with a BOM,
  UTF-16LE/BE, windows-1252, an unmappable character)
- `write`: the header and the lines of `replace`, the cut by `max_lines`

## Steps

1. red: examples and unit tests
2. `content::decode_raw`, `TextPattern` flags, `Filter::has_text`
3. `src/replace.rs`: parse, compile, plan, encode, write
4. engine, description, mcp, warnings
5. the earlier examples which list the tools; docs; clippy, fmt

## Risks

- mapping the normalized offsets back to the raw text: a lone `\r` is text, only `\r\n` is a line break
- the matches of a result searched again may differ from the result when the configuration changed in between; the
  text check catches only file changes, which is what the spec promises
