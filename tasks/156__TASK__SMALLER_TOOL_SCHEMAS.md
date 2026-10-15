# Task 156: Smaller tool descriptions and schemas

Status: not implemented

Depends on: task 125, task 126, task 127, task 129

## Goal

An agent pays for the descriptions and input schemas of the tools before its first search. In run 3 of the experiment
(EXPERIMENT.md, task 151) loading six easy-peasy tools through ToolSearch grew the agent's context from 31.5K to
46.7K tokens in one step: +15.2K, more than the whole difference of 11.7K tokens between the two arms. Three of the
six (`rename`, `copy`, `delete`) were never called.

`tools/list` of 0.148.0, characters:

| Tool            | Description | Input schema | Total  |
|-----------------|-------------|--------------|--------|
| `find`          | 3534        | 6377         | 10076  |
| `replace`       | 1554        | 3442         | 5187   |
| `copy`          | 1649        | 3244         | 5085   |
| `rename`        | 1544        | 3237         | 4975   |
| `delete`        | 864         | 2906         | 3936   |
| `update_config` | 461         | 289          | 920    |
| `get_config`    | 212         | 233          | 557    |
| total           | 9818        | 19728        | 30736  |

The schemas are two thirds of it, and much of them repeats: `traversal` (741), `languages` (637) and `kinds` (386)
are the same 1764 characters in each of the five tools that select files, ~7K characters in the four copies after
the first. `find`'s `output` is 3031 characters, most of it options an agent rarely sets. JSON schemas take more
tokens per character than prose (~2 characters per token here), so each 1K characters saved is ~0.5K tokens of every
session that loads the tools.

## Scope

A spec change first (its own commit; TOOLS.md, the tool descriptions in `src/description.rs`), then the code.

- measure the tokens of each tool as an agent loads it (the run 3 method: the context before and after the load)
- the spec step decides what to cut, e.g.:
  - the per-call configuration overrides (`traversal`, `kinds`, `languages`, `escape`, `query_options`) in the
    schemas of the write tools, or in all tools, with `update_config` / the configuration file as the way to set them,
    or a looser schema for them which names the keys without describing them
  - the rarely used `output` options of `find` described once in the description instead of one by one
  - descriptions of the write tools that repeat the query syntax of `find`
- a request with an option left out of a schema still works or fails with a clear error; spec 1 is unchanged
  (TOOLS.md#versioning decides whether a removed key is breaking)
- the saving per tool, in characters and in tokens, in the summary

## Out of scope

- how the client loads tools (deferred loading, ToolSearch); the server can't choose which tools an agent loads
- the response formats

## Examples

`156__EXAMPLES__SMALLER_TOOL_SCHEMAS.md` covers:

- `tools/list`: the new descriptions and schemas, with their sizes
- a request which sets an option the spec step moved out of a schema: its result or its error

## Acceptance criteria

- the spec states what each tool describes before the code is written
- every example in `156__EXAMPLES__SMALLER_TOOL_SCHEMAS.md` passes as a test; the examples of all earlier tasks still
  pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
