# Task 137: An absolute working directory written once

Status: not implemented

Depends on: task 85, task 90, task 134

## Goal

A request with an absolute working directory costs no more tokens than the same request with a relative one. Today
with `path_style = relative` the paths are relative to the working directory, but the working directory itself is
written as it was given in the request ([OUTPUT.md#paths](../OUTPUT.md#paths)); an absolute one is written in front
of every path with `group_dirs = false`, in every directory header with `group_dirs = true`, and in the paths of the
warnings:

```
/home/me/projects/app/.claude/worktrees/agent-1/src/cache.rs  [#2]
/home/me/projects/app/.claude/worktrees/agent-1/src/output/json.rs  [#1]
```

Found by trying the server: in the parallel runs of task 125 an agent worked in a git worktree outside the default
directory and had to give absolute `dirs`; every path of every response came back absolute, and every response
carried `binary_skipped` / `decode_errors` warnings with long absolute paths of the test fixtures.

## Scope

A spec change first (its own commit; OUTPUT.md, the examples of the paths), then the code.

- with `path_style = relative` and one working directory other than `.`, the working directory is written once, and
  every path, directory header and warning path is relative to it; the exact form is fixed in the spec step, e.g. a
  line `in: /home/me/projects/app/.claude/worktrees/agent-1/` after the header (JSON: `"dir"`), then `src/cache.rs`
- with several working directories every one is written once, as the header of its files (the order of the files
  across the working directories is fixed in the spec step: relevance can interleave them), instead of before every
  path
- the line with the working directory is a part of [`omit`](../OUTPUT.md#omit) (e.g. `dir`): **every part of the
  output is optional**; without it the paths stay relative, which the agent asked for, and the spec says so
- `path_style = absolute` is unchanged: every path absolute
- a relative working directory (`src`, `../api`) is handled the same way: written once, the paths relative to it;
  `.` is not written, as today
- `within`, `since` and the result ids: the paths of a kept result are written as in its response, as today; a
  `path` condition still sees the path relative to the working directory
- the repeated lines, the funnel, the hints and the facets use the same relative paths
- a breaking change of the text format? Decided in the spec step by [TOOLS.md#versioning](../TOOLS.md#versioning);
  if it is, it waits for the next spec version or is behind an option

## Out of scope

- a shorter form of `path_style = absolute`
- the paths inside the `find` request itself
- changing the warnings themselves (which files get `binary_skipped`)

## Spec

- [OUTPUT.md#paths](../OUTPUT.md#paths), [OUTPUT.md#directory-groups](../OUTPUT.md#directory-groups),
  [OUTPUT.md#warnings](../OUTPUT.md#warnings), [OUTPUT.md#omit](../OUTPUT.md#omit)
- [TOOLS.md#versioning](../TOOLS.md#versioning)

## Examples

`137__EXAMPLES__WORKING_DIR_ONCE.md` covers:

- one absolute working directory with `group_dirs` `true` and `false`, in `files` and `lines` detail, text and JSON
- several working directories, absolute and relative
- the warnings of files under an absolute working directory
- the line with the working directory left out with `omit`, and with `omit = "all"`
- `path_style = absolute` unchanged
- `within` and `since` on a result of an absolute working directory

## Acceptance criteria

- the spec says how the working directory is written, and its part of `omit`, before the code is written
- every example in `137__EXAMPLES__WORKING_DIR_ONCE.md` passes as a test; the examples of all earlier tasks still pass
  (or are changed by a decided breaking change, each change named in the summary)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
