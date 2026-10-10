# Task 15: `lang` target

Status: implemented, see [15__SUMMARY__LANG_TARGET.md](15__SUMMARY__LANG_TARGET.md)

Depends on: task 9

## Goal

Find files by language: `lang = rust`, `lang in (typescript, javascript)`.

## Scope

- the built-in language table: extensions, full names, shebangs of files without an extension
- `lang = unknown` for other files
- `=`, `!=`, `in ( ... )`, values case-insensitive, `unknown_value` with a `did you mean` hint
- shebang detection reads only the first line

## Spec

- [QUERY.md#lang](../QUERY.md#lang)
- [CONFIG.md#built-in-languages](../CONFIG.md#built-in-languages)

## Out of scope

- changing the table: tasks 70–77

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)

## Examples

`15__EXAMPLES__LANG_TARGET.md` covers:

- extensions, a `Dockerfile`, a script with a `python3` shebang, `lang = unknown`, `lang != rust`
- `lang = rusty`

## Acceptance criteria

- every example in `15__EXAMPLES__LANG_TARGET.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
