# Task 38: `server.allowed_roots`

Status: not implemented

Depends on: task 34

## Goal

The server searches only inside the configured roots.

## Scope

- the key `allowed_roots` in the `[server]` section of the config file, default `[]` (the start directory)
- the command line option `--root <dir>`, repeatable, replaces the list from the file
- read-only: shown by `get_config`, rejected by `update_config` with `read_only` and the spec hint
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- `~` expanded, relative paths against the start directory, canonicalized at start; a root which doesn't exist stops the server
- every working directory is canonicalized and checked against the roots

## Spec

- [CONFIG.md#server](../CONFIG.md#server)
- [CONFIG.md#security](../CONFIG.md#security)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`38__EXAMPLES__SERVER_ALLOWED_ROOTS.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `38__EXAMPLES__SERVER_ALLOWED_ROOTS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
