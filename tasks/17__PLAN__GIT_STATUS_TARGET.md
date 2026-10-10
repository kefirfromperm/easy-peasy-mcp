# Plan for task 17: `git` target: working tree statuses

## Design

- `git.rs` (new): `GitStatus` — the statuses of one file (`modified`, `staged`, `untracked`, `conflicted`) or
  `clean`; `Repo` — the root and the statuses of its files by `/`-separated path relative to the root, plus the
  tracked files; `GitIndex` — the repositories found so far, cached per directory, shared by the walk threads.
  - a file's repository: the nearest directory upwards which contains `.git` (a directory, or a file for worktrees
    and submodules), so nested repositories work; outside of one — `None`
  - the status: `git -C <root> -c core.fsmonitor=false -c core.quotepath=off status --porcelain=v2 -z
    --untracked-files=all --no-renames` — `1 XY` / `2 XY` entries: `X` ≠ `.` staged, `Y` ≠ `.` modified; `u` —
    conflicted; `?` — untracked; tracked files from `git ls-files -z`; a tracked file without an entry is clean
  - without the `git` program (or when it fails) the directory counts as outside of a repository
  - with `git_global_excludes` off (tests) git runs without the global and system configuration
- `query`: `Filter::Git { values, negated }` over `FileValues.git: Option<GitStatus>`; outside of a repository and
  for directories every `git` condition is false, also `!=`; `Needs.git`; `unknown_value` for git values; the subset
  supports `git` except the value `branch` (task 18), which is `not_implemented`.
- `warning.rs`: `not_a_git_repo` for entries evaluated by a query with `git` outside of a repository.
- `description.rs`: `git = changed|staged|untracked|modified|conflicted|clean`.

Rejected: `gix` (a large dependency tree for one status call) and `git2` (libgit2 needs a C toolchain); the `git`
program gives git's own semantics of excludes and index.

## Tests

- examples E17.1–E17.10 (fixture `gitrepo` built by the setup: merge conflict, staged, unstaged, both,
  untracked, clean, a directory outside)
- unit: parsing of porcelain v2 records (ordinary, unmerged, untracked, paths with spaces); `GitStatus` values;
  `Filter::Git` with `=`, `!=`, `in`, outside of a repository; the subset rejects `branch`

## Steps

1. failing unit tests
2. git module, filter, search, warnings
3. description

## Risks

- `git status` may run the clean filters of `.gitattributes` for files whose stat changed; repositories the agent
  works in are its own, but the plan notes it
