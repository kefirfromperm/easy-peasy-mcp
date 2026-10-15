# Task 148: Paths relative to the only working directory

Status: implemented, see [148__SUMMARY__ONE_DIR_RELATIVE.md](148__SUMMARY__ONE_DIR_RELATIVE.md)

Depends on: task 90, task 137

## Goal

With one absolute working directory (a git worktree, a project outside the default directory) `path_style =
relative`, the default, writes that directory before every path: in every directory header and in every location
of a [repeated line](../OUTPUT.md#repeated-lines). `in_dir` writes it once, but an agent has to know and pass it on
every call.

Found in the experiment of task 144 (one agent with easy-peasy only, in a worktree):
`{"dirs": ["/home/me/easy-peasy-mcp/.claude/worktrees/agent-1"], ...}` wrote the full worktree path in every header
and location; it was the largest waste of tokens of the run.

```
/home/vsamolovskikh/myprojects/easy-peasy-mcp/tasks/
147__TASK__INDENT_CARRIED.md
```

## Scope

A spec change first (its own commit; OUTPUT.md#paths), then the code.

- spec 2: with one working directory `relative` writes the paths relative to it and the directory once, like
  `in_dir` (the `in:` line, JSON `dirs`); several working directories are unchanged
- the spec step decides: whether this changes `relative` or the default becomes `in_dir`; whether a relative
  working directory other than `.` (`dirs: ["src"]`) gets the same rule; what `within` of an earlier result writes;
  the hints and warnings with paths
- the paths a write tool accepts from a response (`result = "rN"`, paths copied by an agent) still resolve
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- `absolute`; several working directories

## Examples

`148__EXAMPLES__ONE_DIR_RELATIVE.md` covers:

- one absolute working directory: `files` and `lines` detail, a repeated line, text and JSON
- one relative working directory, unchanged (the spec step decided it); `.`, unchanged
- two working directories, unchanged; `path_style = absolute`, unchanged
- `"version": 1`, unchanged

## Acceptance criteria

- the spec says how the paths of one working directory are written before the code is written
- every example in `148__EXAMPLES__ONE_DIR_RELATIVE.md` passes as a test; the examples of all earlier tasks still
  pass (a spec 2 example which changes is fixed with the reason in its commit)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
