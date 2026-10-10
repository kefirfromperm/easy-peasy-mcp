# Task 19: `text has` in file scope

Status: in progress

Depends on: task 6

## Goal

Search the content: `name ~ '*.rs' and text has 'unwrap('` returns the files and the matched lines.

## Scope

- `text has` on the whole file content; a value with `'\n'` can match across lines
- files are read as UTF-8; lines are split by `\n`, a trailing `\r` is removed
- `/i` on `text` (Unicode simple case folding)
- the default detail `lines`: per file the counts `lines: N  matches: M`, the matched lines `N:text`, the fragments marked with `⟦ids:text⟧` (`highlight = auto` in text), overlapping fragments merged
- header counts: files, matched lines, matches; `path` / `name` fragments are not counted
- which conditions produce fragments: only those which made the file match, never under `not`
- content conditions are false for directories

## Spec

- [QUERY.md#text-operators](../QUERY.md#text-operators)
- [QUERY.md#file-scope](../QUERY.md#file-scope)
- [OUTPUT.md#fragments](../OUTPUT.md#fragments)
- [OUTPUT.md#highlight](../OUTPUT.md#highlight)
- [OUTPUT.md#counts](../OUTPUT.md#counts)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Out of scope

- binary files and unknown content — task 20
- encodings other than UTF-8 — task 25

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`19__EXAMPLES__TEXT_HAS.md` covers:

- one and several matches on a line, a match in several files, a case-insensitive match
- `not text has` (like `grep -L`), `text has` combined with `name`
- a value with `\n`, CRLF files

## Acceptance criteria

- every example in `19__EXAMPLES__TEXT_HAS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
