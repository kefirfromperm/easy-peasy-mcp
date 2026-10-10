# Summary of task 19: `text has` in file scope

Server version: `0.19.0`.

## What was built

- `text has` on the whole content (QUERY.md#file-scope), `/i` with Unicode simple case folding; a value with `'\n'`
  matches across lines; `\r\n` line breaks work like `\n`.
- Content conditions are false for directories and symlinks.
- The default detail `lines` (OUTPUT.md#text-format): the header totals of files, matched lines and matches; per file
  `lines: N  matches: M` (left out when 0) and the matched lines `N:text` in blocks with `--` between blocks of
  non-adjacent lines; fragments marked `⟦ids:text⟧`, overlapping fragments one marked part with all ids, identical
  fragments one fragment, a multiline fragment opened on its first line and closed on its last.
- Fragments only from the conditions which made the file match (OUTPUT.md#fragments), never under `not`.
- The `find` description shows `text has` and the line format.
- `.gitattributes` keeps fixture bytes (CRLF) on every platform.

## Architecture

- `content.rs`: `Content` — decoded text (UTF-8, invalid bytes replaced) with normalized line breaks and line starts.
- `query/eval.rs`: `ContentState` (`NotRead` → unknown, `Absent` → false, `Text`), `Filter::Content` (a regex of
  the escaped value); `contributing_leaves` gives the contributing conditions themselves.
- `search`: two-phase evaluation — without the content first, the file is read only when the result isn't false;
  `FileMatches` with the content, the merged fragments and the matched line count.
- `output/lines.rs`: `Fragment`, `merge_identical`, `matched_lines`, `render` (blocks and markers by byte offsets of
  the normalized text).

## Deviations from the plan

None.

## Tests

- examples E19.1–E19.10 (fixture `content`)
- unit: `Content` (CRLF, trailing line break, positions, invalid UTF-8), rendering (markers, blocks, overlaps,
  multiline), counts, merging identical fragments, the subset

## Limitations and known gaps

- binary files are searched as text until task 20; other encodings — task 25
- dedent, context, long line cuts and output limits — tasks 80–100

## Try it

`{ "query": "name ~ '*.rs' and text has 'unwrap('" }`
