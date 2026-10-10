# Task 70: `languages.<name>.extensions`

Status: implemented, see [70__SUMMARY__LANGUAGES_EXTENSIONS.md](70__SUMMARY__LANGUAGES_EXTENSIONS.md)

Depends on: task 15, task 36

## Goal

Add or override a language: `[languages.terraform] extensions = ["tf", "tfvars"]`.

## Scope

- the key `extensions` in the `[languages.<name>]` section of the config file, default the built-in table
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `languages.<name>.extensions` replaces it for one request
- the request object `languages` is added to the `find` input schema by this task, with this key only; the other keys of the spec are added by their tasks
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- a `[languages.<name>]` section with the same name replaces the built-in language completely; a new name adds a language
- the request field `languages` replaces languages one by one
- an extension listed in two languages is an error at start (and `invalid_value` / `invalid_option`)
- `lang`, `kind = source` and `/c` use the changed table

## Spec

- [CONFIG.md#languages](../CONFIG.md#languages)
- [CONFIG.md#built-in-languages](../CONFIG.md#built-in-languages)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`70__EXAMPLES__LANGUAGES_EXTENSIONS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `70__EXAMPLES__LANGUAGES_EXTENSIONS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
