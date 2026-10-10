# Task 25: Encoding detection

Status: not implemented

Depends on: task 20

## Goal

Search files in UTF-16, UTF-32 and UTF-8 with BOM like plain UTF-8; invalid bytes don't break the search.

## Scope

- `encoding = auto`: BOM for UTF-8, UTF-16 LE/BE, UTF-32 LE/BE; UTF-16 without BOM by the zero byte pattern; otherwise UTF-8
- the BOM / UTF-16 checks run before binary detection, so UTF-16 text is not binary
- invalid UTF-8 bytes are replaced with `�` and the response has a `decode_errors` warning
- the encoding of every non-UTF-8 file is shown in the output (`encoding` in JSON, after the counts in text)
- columns count Unicode characters of the decoded text; the output is UTF-8

## Spec

- [CONFIG.md#encoding](../CONFIG.md#encoding)
- [OUTPUT.md#fields](../OUTPUT.md#fields)
- [OUTPUT.md#text-format](../OUTPUT.md#text-format)

## Out of scope

- `traversal.encoding` — task 55
- `fallback_encoding` — task 56

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`25__EXAMPLES__ENCODING_AUTO.md` covers:

- fixture files with each BOM and UTF-16 LE without BOM, created by the test setup
- a file with invalid UTF-8 and its warning

## Acceptance criteria

- every example in `25__EXAMPLES__ENCODING_AUTO.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
