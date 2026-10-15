---
name: implement-next-experiment
description: Run the next run of the easy-peasy A/B experiment from EXPERIMENT.md - implement the next not implemented task twice in parallel worktrees, one agent with easy-peasy only, one with grep/sed/find only, then compare cost, quality and friction. Use when the user asks for the next experiment or to implement a task "both ways".
---

# Implement the next experiment

The design is [EXPERIMENT.md](../../../EXPERIMENT.md): read it first, it holds the arms, the exact agent prompts, the
metrics and the follow-up. This skill is the checklist of one run.

## 1. Prepare

- Clean `master`, nothing uncommitted.
- The installed server must be the repo version: `./install.sh --build` if `easy-peasy-mcp --version` differs from
  `Cargo.toml`, then ask the user to reconnect through `/mcp` and check the `get_config` header.
- Pick the task: the number the user named, or the first ⬜ row of `tasks/README.md` whose dependencies are all ✅.
  Its task file must already be on `master`.

## 2. Launch both arms

- One message, two `Agent` calls, both with `isolation: "worktree"`, run in the background.
- Prompts: the Arm A and Arm B prompts of EXPERIMENT.md#prompts, with the task number, task file and branch names
  (`task/N-name-ep`, `task/N-name-grep`) filled in. Don't add hints about the task: both arms get the same text
  besides the tool rules.
- Tell the user both are running; don't help or steer them while they work, and don't predict their results.

## 3. Collect

When each completion notification arrives, write down `subagent_tokens`, `tool_uses`, `duration_ms` and the agent's
tooling log. Then, for each branch:

- `git log --oneline master..BRANCH`, `git diff --stat master...BRANCH` (all files and `src/`)
- fmt, clippy `-D warnings`, `cargo test --all-targets` in its worktree
- rule breaks: a tool the arm forbids (from the report and the agent's transcript)
- read both diffs: the same feature? spec rules, edge cases, examples, tests that differ

## 4. Compare and report

- The metrics table of EXPERIMENT.md#runs, then quality, the friction of each arm, and which branch to merge.
- Total tokens are the whole run; say so next to the table.
- For each easy-peasy friction: reproduce it on the installed server, then propose it as a task, or say why not
  (behaves as specified, out of scope). Create the tasks when the user agrees (AGENTS.md#new-tasks).
- Append the run to EXPERIMENT.md#runs (`### Run K — task N <title>, server X, <date>`) with the table, the notes and
  the outcome, and commit it on `master`.

## 5. Merge and clean up (after the user picks a branch)

1. Review the chosen branch line by line; run the definition of done (AGENTS.md#definition-of-done) on it.
2. `git merge --no-ff BRANCH -m "Merge task N: <title>"`, rename the branch to `task/N-name`.
3. `./install.sh --build`, check the version, tell the user to reconnect through `/mcp`.
4. Remove both worktrees (check they have no uncommitted changes first), delete the other arm's branch (tell the user
   its last commit, to restore it) and the `worktree-agent-*` branches of this run only.
