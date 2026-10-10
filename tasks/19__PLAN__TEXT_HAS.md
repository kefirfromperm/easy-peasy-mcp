# Plan for task 19: `text has` in file scope

## Design

- `content.rs` (new): `Content` — the decoded text of a file (UTF-8, invalid bytes replaced; encodings come with
  task 25) normalized to `\n` line breaks (a `\r` before `\n` is removed, so CRLF files work like LF ones and a
  value with `'\n'` matches across their lines), with the byte offsets of the line starts; `position(offset)` →
  line and column in characters.
- `query/eval.rs`: `Filter::Content { id, order, regex }` — `has` compiles to the escaped value, `(?i)` with `/i`;
  `FileValues.content: ContentState` — `NotRead` (unknown: the cheap metadata conditions are evaluated first),
  `None` for directories and symlinks (false: no content), `Text(&Content)`.
- `contributors` returns the contributing leaves, so the search gets both the ids and the content conditions which
  produce fragments (OUTPUT.md#fragments: only conditions which made the file match, never under `not`).
- `search`: the query is evaluated first with `NotRead`; when the result is not `False` and the query has content
  conditions, the file is read and evaluated again. Fragments: every match (`find_iter`) of every contributing
  content condition, by byte range of the normalized text; identical ranges merge into one fragment with all ids.
  `Found.content: Option<FileMatches>` with the fragments and the counts (matched lines: lines touched by a
  fragment; matches: fragments).
- `output`: the header counts are the totals; per file `lines: N  matches: M` when not 0; matched lines `N:text` in
  blocks, `--` between blocks of non-adjacent lines; markers `⟦ids:` / `⟧` inserted at the ends of merged parts
  (overlapping fragments merge into one part with all ids, a multiline fragment opens on its first line and closes
  on its last).
- `query/subset.rs`: `text` with `has`; `/i` on it.
- `Needs.content`; `description.rs`: `text has`.

## Tests

- examples E19.1–E19.10 (fixture `content`)
- unit: `Content` (CRLF, positions in characters, last line without a break), `Filter::Content` states, the
  contributing leaves, fragment merging and line rendering with markers (overlaps, identical, multiline), blocks

## Steps

1. failing unit tests
2. content, filter, contributions, search (two-phase), output
3. description

## Risks

- the whole file is read into memory; `max_file_size` (task 54) bounds it later
