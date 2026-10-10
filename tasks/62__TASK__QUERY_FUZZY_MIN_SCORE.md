# Task 62: `query.fuzzy_min_score`

Status: implemented, see [62__SUMMARY__QUERY_FUZZY_MIN_SCORE.md](62__SUMMARY__QUERY_FUZZY_MIN_SCORE.md)

Depends on: task 32, task 36

## Goal

Tune how loose `~~` is.

## Scope

- the key `fuzzy_min_score` in the `[query]` section of the config file, default `0.5`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `query_options.fuzzy_min_score` replaces it for one request
- the request object `query_options` is added to the `find` input schema by this task, with this key only; the other keys of the spec are added by their tasks
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- a number 0–1; the request field is `query_options` because `query` is the query itself

## Spec

- [CONFIG.md#query](../CONFIG.md#query)
- [QUERY.md#--fuzzy](../QUERY.md#--fuzzy)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`62__EXAMPLES__QUERY_FUZZY_MIN_SCORE.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `62__EXAMPLES__QUERY_FUZZY_MIN_SCORE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
