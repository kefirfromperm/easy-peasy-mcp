# Summary of task 29: `/c` ignore matches in comments

Server version: `0.29.0`.

## What was built

- `/c` on every operator on `text`, in file scope and inside `line( ... )` and `near( ... )` (QUERY.md#comments): a
  match whose first character is inside a comment is dropped; a match before a line comment on the same line counts.
- The comment syntax of the built-in languages (CONFIG.md#comment-syntax, CONFIG.md#built-in-languages): line
  comments, block comments, nested block comments and string delimiters; the longest marker wins (`--[[` before
  `--`); comment markers inside a string are text; `"` and `'` strings end at the line end, `` ` `` strings may span
  lines; an unterminated block comment runs to the end of the file.
- Files of an unknown language are searched as is.
- The `find` description shows `/c`.

## Architecture

- `languages.rs`: `Language` has `line_comments`, `block_comments`, `nested_block_comments` and `strings`.
- `comments.rs`: `comment_ranges(text, language)` scans the normalized content and returns the sorted byte ranges of
  the comments, markers included.
- `content.rs`: `Content::with_comments(language)` keeps the ranges, `in_comment(offset)` is a binary search;
  `LineRef` is a line with its offset, used by the line scope evaluation.
- `query/eval.rs`: `Filter::Content` holds a `TextPattern` (regex, per line, `/w`, `/c`) with `content_matches` and
  `line_matches`; `find_all` rejects matches by `/w` and `/c` and goes on one character after a rejected start.
- `Needs.comments`: the search detects the language and computes the comments only for queries with `/c`.

## Deviations from the plan

- The regex and its flags moved into `TextPattern` instead of adding a fourth flag to `Filter::Content`.
- `LineRef` refers to its `Content` instead of carrying a slice of comment ranges.

## Tests

- examples E29.1–E29.6 (fixture `comments`)
- unit: the scanner (`comments.rs`), offsets in comments (`content.rs`), `/c` in file scope, with `/w`, with `=~`
  per line and `/m`, in `line( ... )` and `near( ... )` (`query::tests::skip_comments`)

## Limitations and known gaps

- The scanner is not a parser: raw strings with other delimiters (`r#"..."#`), heredocs and similar are not known.
- Changing the comment syntax in the configuration comes with tasks 73–76.

## Try it

`{ "query": "name ~ '*.rs' and not text has 'unsafe' /c" }`
