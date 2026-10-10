# Task 22: Regex operator `=~`

Status: in progress

Depends on: task 21

## Goal

Match by regular expression: ``text =~ `fn \w+_test\(` ``, ``name =~ `^v\d+`` ``.

## Scope

- `=~` on `text`, `path`, `name`; Rust `regex` syntax, linear time
- on `text` without `/m` the regex is applied to every line, `^` / `$` are line start and end
- raw strings `` `...` `` are the recommended form, quoted strings work with their escapes
- `/i` with `=~`
- `invalid_regex` with the spec hint (no look-arounds, no backreferences)

## Spec

- [QUERY.md#regex](../QUERY.md#regex)
- [QUERY.md#strings](../QUERY.md#strings)
- [QUERY.md#errors](../QUERY.md#errors)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`22__EXAMPLES__REGEX.md` covers:

- per line regex with anchors, several matches per line, on `name` and `path`
- a look-around error, an invalid regex

## Acceptance criteria

- every example in `22__EXAMPLES__REGEX.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
