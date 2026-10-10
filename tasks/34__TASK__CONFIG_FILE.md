# Task 34: Config file

Status: implemented, see [34__SUMMARY__CONFIG_FILE.md](34__SUMMARY__CONFIG_FILE.md)

Depends on: task 1

## Goal

The server reads a TOML config file at start: the base for every configuration key task.

## Scope

- lookup order: `--config <path>`, `$EASY_PEASY_CONFIG`, the platform config directory (`$XDG_CONFIG_HOME/easy-peasy-mcp/config.toml` on Linux, the macOS and Windows paths of the spec); the first found is used, without a file the built-in defaults
- no project level config file is ever read
- an invalid file stops the server at start with an error naming the key and the line
- the value sources and their priority: defaults ← file ← command line ← `update_config` ← request
- keys are added by their own tasks; until then a key the spec defines but the server doesn't implement yet stops the server with an error that names the key as not implemented — never silently ignored
- an unknown key or section is an error

## Spec

- [CONFIG.md#config-file](../CONFIG.md#config-file)
- [CONFIG.md#where-the-values-come-from](../CONFIG.md#where-the-values-come-from)
- [CONFIG.md#command-line](../CONFIG.md#command-line)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- empty result: the funnel and hints of `funnel = auto` come with tasks 102–103
- more than 100 matched files: `max_files` truncation and the cursor come with tasks 94 and 101
- relevance order comes with task 93; until then files are ordered by path, so the matched files of an example must have equal relevance (same kind, depth and modification time)
- output transformations which are on by default and come later: dedent (task 84), repeated lines (task 86), long line cuts (task 98), per file and total output limits (tasks 95–100) — examples have no common indentation in matched lines, no line repeated in 3 or more files, lines shorter than 300 characters and few matches

## Examples

`34__EXAMPLES__CONFIG_FILE.md` covers:

- config files are startup inputs, not `find` requests: examples here are a `config` block, the command line and the expected start error (text on stderr, exit code); the example format of AGENTS.md is extended for that in this task

## Acceptance criteria

- every example in `34__EXAMPLES__CONFIG_FILE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
