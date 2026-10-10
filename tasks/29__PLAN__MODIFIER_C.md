# Plan for task 29: `/c` ignore matches in comments

## Design

- `languages.rs`: `Language` gets `line_comments`, `block_comments`, `nested_block_comments`, `strings` from the
  built-in table (the spec now lists the string delimiters).
- `comments.rs` (new): `comment_ranges(text, language)` — the scanner of CONFIG.md#comment-syntax over the
  normalized content: outside of strings and comments the longest marker starts a string, a line comment or a block
  comment; strings end at their unescaped delimiter (`"` / `'` also at the line end); nested block comments count
  their depth; the result is the sorted byte ranges of comments, markers included.
- `Content` keeps the comment ranges (empty for unknown languages) and answers `in_comment(offset)`; `LineRef { text,
  start, comments }` gives line scope conditions their position in the content.
- `query/eval.rs`: `Filter::Content.skip_comments`; `content_matches` and the per-line matching drop matches which
  start inside a comment; `eval_line` and its helpers take a `LineRef`.
- `Needs.comments` (with the language): the search computes the comment ranges only for queries with `/c`.
- `query/subset.rs`: `/c` is supported; `description.rs`: `/c skip comments`.

## Tests

- examples E29.1–E29.6 (fixture `comments`)
- unit: the scanner for line, block, nested block comments, strings with escapes, markers in strings, the longest
  marker (`--[[` before `--`), unterminated strings and comments; `/c` matching in file and line scope

## Steps

1. failing unit tests
2. language data, scanner, content, filter, search, subset
3. description

## Risks

- the scanner is not a parser: raw strings with other delimiters (`r#"..."#`) and heredocs are not known; a match is
  then wrongly kept or dropped — the cost of a language table instead of grammars
