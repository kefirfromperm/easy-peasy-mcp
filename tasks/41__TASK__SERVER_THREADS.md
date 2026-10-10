# Task 41: `server.threads`

Status: implemented, see [41__SUMMARY__SERVER_THREADS.md](41__SUMMARY__SERVER_THREADS.md)

Depends on: task 40

## Goal

Control the CPU the server uses.

## Scope

- the key `threads` in the `[server]` section of the config file, default `0` (number of CPU cores)
- shown by `get_config` (with its source), changed and reset by `update_config`
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the search thread pool; walking and file search are parallel
- results don't depend on the number of threads

## Spec

- [CONFIG.md#server](../CONFIG.md#server)
- [TOOLS.md#concurrency](../TOOLS.md#concurrency)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`41__EXAMPLES__SERVER_THREADS.md` covers:

- the same request with `threads = 1` and the default gives the same response

## Acceptance criteria

- every example in `41__EXAMPLES__SERVER_THREADS.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
