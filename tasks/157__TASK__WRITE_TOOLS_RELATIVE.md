# Task 157: Paths relative to the only working directory in the write tools

Status: not implemented

Depends on: task 148, task 125, task 126, task 127, task 129

## Goal

Task 148 made `find` write one absolute working directory once (`in:`) and the paths relative to it. The write tools
still write the absolute directory in every directory header. Found in run 3 of the experiment (EXPERIMENT.md, task
151), reproduced on 0.147.0 in a worktree:

```
replace  dirs = ["/home/vsamolovskikh/myprojects/easy-peasy-mcp/.claude/worktrees/agent-af51ef0755b9c5946"]
         query = "path ~ 'src/description.rs' and text has 'the groups of a =~ regex,' @old", dry_run = true

dry run  would replace: 0  matches: 0  unchanged: 1

/home/vsamolovskikh/myprojects/easy-peasy-mcp/.claude/worktrees/agent-af51ef0755b9c5946/src/
description.rs  unchanged
```

`find` with the same `dirs` and query:

```
files: 1  lines: 1  matches: 1  result: r21
in: /home/vsamolovskikh/myprojects/easy-peasy-mcp/.claude/worktrees/agent-af51ef0755b9c5946/

src/
description.rs  lines: 1  matches: 1
```

The saving is the length of the working directory (~85 characters in a worktree) per directory header after the
first, in every response of `replace`, `rename`, `copy` and `delete` with one absolute working directory.

## Scope

A spec change first (its own commit; TOOLS.md, the response sections of the write tools), then the code.

- the rule of task 148 for the responses of `replace`, `rename`, `copy` and `delete` (the dry run and the real call):
  one working directory written once, the paths relative to it; several working directories are unchanged
- the spec step decides: the `in:` line in a write response, the target paths of `rename` and `copy`, the paths of
  skipped files in warnings, JSON
- the paths a write tool accepts (`result = "rN"`, paths copied from a response) still resolve
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- `find` (task 148); `path_style = absolute`

## Examples

`157__EXAMPLES__WRITE_TOOLS_RELATIVE.md` covers:

- a dry run and a real call of each write tool with one absolute working directory
- several working directories: unchanged
- `rename` / `copy` with a target in another directory

## Acceptance criteria

- the spec states the paths of the write responses before the code is written
- every example in `157__EXAMPLES__WRITE_TOOLS_RELATIVE.md` passes as a test; the examples of all earlier tasks still
  pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
