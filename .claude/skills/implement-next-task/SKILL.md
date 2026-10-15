---
name: implement-next-task
description: Implement the next not implemented task of easy-peasy-mcp from tasks/README.md, end to end - branch, examples, plan, red tests, code, summary, version bump, merge to master, reinstall. Use when the user says "implement the next task" or names a task number to implement.
---

# Implement the next task

The workflow is AGENTS.md (#workflow, #git, #definition-of-done); this skill is the checklist. Read the AGENTS.md
sections you need rather than working from memory.

## 1. Pick the task

- If the user named a task number, take it. Otherwise take the first ⬜ row in `tasks/README.md` whose `Depends on`
  tasks are all ✅.
- Read `tasks/N__TASK__NAME.md` and the spec sections it links. Start from a clean `master`.
- Call easy-peasy `get_config` (with a `section`) once and note the server version.

## 2. Work on the branch

`git switch -c task/N-name` from `master`, then commit in this order (at least 5 commits):

| # | Commit                                                                      |
|---|-----------------------------------------------------------------------------|
| 1 | spec change, if the task needs one (its own commit, spec only)               |
| 2 | `N__EXAMPLES__NAME.md` and the fixtures                                      |
| 3 | `N__PLAN__NAME.md`, task status `in progress` (also in `tasks/README.md`: 🚧) |
| 4 | tests: example tests and unit tests, failing for the right reason           |
| 5 | code that makes them pass (one or more commits), refactor with tests green  |
| 6 | `N__SUMMARY__NAME.md`, status `implemented` (README ✅), version bump in `Cargo.toml` |

- The spec is the contract: when it is ambiguous or the task leaves a choice to the spec step, decide it in the spec
  commit and tell the user the decision in the final report.
- Earlier tasks' examples that the change affects are updated in the same commits as the behaviour.
- Keep each commit about one step; if a commit picked up unrelated files, split it before merging.

## 3. Tools

Follow CLAUDE.local.md: easy-peasy `find` first for search, its write tools for query-selected edits (dry run first),
every `find` validated against the equivalent grep/find request, one row per search in `EASY_PEASY_STATS.local.md`.
A bug or a token saving found in easy-peasy becomes a new task right away (AGENTS.md#new-tasks), committed to
`master` after this task's merge.

## 4. Done, merge, install

1. All green:
   ```sh
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets
   cargo build --release
   ```
2. Try the release binary over MCP stdio with a request that shows the new behaviour.
3. `git switch master && git merge --no-ff task/N-name -m "Merge task N: <title>"` — without waiting to be asked.
4. `./install.sh --build`, check that `easy-peasy-mcp --version` names the version of `Cargo.toml`.

## 5. Report

Short, to the user:

- what was built and what stayed unchanged
- decisions the user may want to look at (choices the task left open, spec fixes made on the way)
- checks run and their results
- easy-peasy stats: searches, totals of both token columns, the ❌ cases, new tasks with number and title
- tell the user to reconnect the server with `/mcp` to get the new version
