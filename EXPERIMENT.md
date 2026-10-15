# EXPERIMENT.md

An A/B experiment on the agent's tooling: the same task of this repository is implemented twice, at the same time,
by two agents that differ only in the tools they may use for searching and editing. One uses only easy-peasy, the
other only classic shell tools. The runs are compared on cost, quality and the friction each tool set caused.

## Questions

1. **Cost.** Does an agent with easy-peasy spend fewer tokens, tool calls and minutes on a real task than an agent
   with `grep`, `find`, `ls` and `sed`?
2. **Quality.** Is the result as good: the same feature, spec, tests, clippy, the workflow of AGENTS.md followed?
3. **Friction.** Where did each tool set get in the way: wrong or empty results, too much output, retries,
   workarounds, broken tool rules? Each easy-peasy finding is a candidate task.

## Arms

| Arm | Name                | Search and listing                                   | Multi-file edits                                  | Branch                  |
|-----|---------------------|------------------------------------------------------|---------------------------------------------------|-------------------------|
| A   | easy-peasy only     | easy-peasy `find` (and `get_config`)                 | easy-peasy `replace`/`rename`/`copy`/`delete`     | `task/N-name-ep`        |
| B   | classic shell tools | `grep`, `find`, `ls`, `sed`, `awk`, `cat`, `head`, `tail`, `wc`, `git grep` | `sed`                       | `task/N-name-grep`      |

The same in both arms:

- the task, the base commit, the model and the workflow (AGENTS.md: task branch, plan, examples, tests first,
  summary, status, version bump, at least 5 commits, tests and clippy green, no merge, no push)
- `Read` for a file already located (with `offset`/`limit`), `Edit` for a single known edit, `Write` for new files
- `python3` scripts that edit or write files (several replacements in one call), not for searching or reading: run 3
  showed it saves calls, and arm B used it although its rules didn't allow it
- Bash for `cargo build/test/clippy/fmt`, `git` (not `git grep` in A) and running the built binary
- neither arm uses rustrover-index / intellij-index or the Glob/Grep tools
- neither arm validates easy-peasy results against grep or writes `EASY_PEASY_STATS.local.md`: this overrides
  CLAUDE.local.md for the experiment

## Setup

1. The installed server is the latest release: rebuild and reinstall it, reconnect through `/mcp`, and check that the
   `get_config` header shows the version of `Cargo.toml`.
2. Pick the next task that is `not implemented`, with its task file written.
3. Start both agents **in parallel** in one message, each with `isolation: "worktree"`, with the prompts below.
4. The parent session doesn't help either arm while they run.

### Worktree pitfall

The easy-peasy server's working directory is the main checkout, not the agent's worktree. Arm A must pass `dirs` with
the absolute path of its worktree on every call (and should never touch the main checkout). This makes arm A's paths
absolute, which costs tokens (see task 148, which writes such a directory once).

The agent worktree can start on an old commit, not on `master` (run 2: both started on 6ff6d67, task 130, without
the task file). Both agents noticed it and ran `git reset --hard master` on their branch first; check `git log -1` of
each branch base when comparing.

## Prompts

Replace `N`, the task file and the branch names.

### Arm A

```text
Implement task N of this repository: tasks/N__TASK__NAME.md. This is an experiment comparing tooling: you are the
"easy-peasy only" arm.

TOOL RULES (strict):
- For ALL searching, listing files, and finding text, use ONLY the easy-peasy MCP tools (mcp__easy-peasy__find, load
  via ToolSearch "select:mcp__easy-peasy__find,mcp__easy-peasy__get_config,mcp__easy-peasy__replace,
  mcp__easy-peasy__rename,mcp__easy-peasy__copy,mcp__easy-peasy__delete"). Do NOT use grep, rg, sed, awk, find, ls,
  cat, head, tail, git grep, Glob/Grep, or rustrover-index/intellij-index.
- Multi-file mechanical changes: easy-peasy replace/rename/copy/delete (dry_run first). Single known edits: Edit tool.
  New files: Write. Several edits at once: a python3 script that edits or writes files (not for searching or
  reading). Reading a file you located: Read tool (use offset/limit, read only what you need).
- Bash is allowed ONLY for: cargo build/test/clippy/fmt, git (status/diff/add/commit/branch, but not git grep),
  running the built binary, and python3 scripts that edit or write files.
- You work in a git worktree. The easy-peasy server's default dir is the MAIN checkout, not your worktree: always pass
  `dirs` with the absolute path of your worktree (run `pwd` once to learn it). Never touch the main checkout at
  /home/vsamolovskikh/myprojects/easy-peasy-mcp.
- You don't need to cross-validate easy-peasy results with grep (overrides CLAUDE.local.md for this experiment); don't
  write EASY_PEASY_STATS.local.md.
- Use output.highlight = "none" and pick detail (files/count/lines) per need to save tokens.

WORKFLOW: follow AGENTS.md and the task conventions of the repo (plan file, commits, summary, status, version bump, as
previous tasks did). Work on branch task/N-name-ep (create it in your worktree). At least 5 commits. All tests and
clippy must be green. Do NOT merge to master and do not push.

FINAL REPORT (keep it compact): branch name, commit list, what you implemented, test results, and a tooling log:
number of easy-peasy calls by tool, rough total characters of easy-peasy responses you received, any easy-peasy
bugs/awkwardness you hit (with the query), and anything the tool couldn't do that forced a workaround.
```

### Arm B

```text
Implement task N of this repository: tasks/N__TASK__NAME.md. This is an experiment comparing tooling: you are the
"classic shell tools" arm.

TOOL RULES (strict):
- For ALL searching, listing files, and finding text, use ONLY classic shell tools via Bash: grep, find, ls, sed, awk,
  cat, head, tail, wc, git grep. Do NOT use the easy-peasy MCP tools, rustrover-index/intellij-index, or the Glob/Grep
  tools. This overrides CLAUDE.local.md's "easy-peasy first" rule for this experiment; don't write
  EASY_PEASY_STATS.local.md.
- Edits: Edit tool, Write for new files, sed for multi-file mechanical changes, or a python3 script that edits or
  writes files (not for searching or reading). Read tool is allowed for reading a located file (use offset/limit,
  read only what you need).
- Bash also for cargo build/test/clippy/fmt and git.
- You work in a git worktree; work only there (run `pwd` once). Never touch the main checkout at
  /home/vsamolovskikh/myprojects/easy-peasy-mcp.

WORKFLOW: follow AGENTS.md and the task conventions of the repo (plan file, commits, summary, status, version bump, as
previous tasks did). Work on branch task/N-name-grep (create it in your worktree). At least 5 commits. All tests and
clippy must be green. Do NOT merge to master and do not push.

FINAL REPORT (keep it compact): branch name, commit list, what you implemented, test results, and a tooling log:
number of search/listing commands by tool (grep/find/ls/sed/...), rough total characters of their outputs you
received, and any case where the tools were awkward or gave too much/too little output.
```

## Measurements

| Metric                    | Source                                                                         |
|---------------------------|--------------------------------------------------------------------------------|
| total agent tokens        | `subagent_tokens` in the agent's completion notification                        |
| tool calls                | `tool_uses` in the notification                                                 |
| time                      | `duration_ms` in the notification                                               |
| search output received    | the agent's own tooling log (rough characters); for B, split searches from file reads |
| commits, diff size        | `git log --oneline master..BRANCH`, `git diff --stat master...BRANCH` (all files and `src/`) |
| tests, clippy             | `cargo test`, `cargo clippy --all-targets -- -D warnings` on each branch         |
| rule breaks               | the transcript / the report: a tool used that its arm forbids                   |
| design and spec           | read both diffs: the same feature? which edge cases, spec rules, examples differ? |

Total tokens are the whole agent run (reading, thinking, editing, cargo), not search output alone: they answer the
real question, what the task cost.

## Comparison and follow-up

1. A table of both arms with the metrics above, then the quality notes and the friction of each arm.
2. For every easy-peasy friction: reproduce it on the installed version, then decide: a bug or a token saving → a task
   (AGENTS.md#new-tasks); behaviour as specified, or out of easy-peasy's scope (e.g. filtering `cargo` output) → noted,
   no task.
3. The user picks the branch to merge. It gets a line-by-line review, the checks on it, a merge to master and a rename
   to `task/N-name`.
4. Cleanup: reinstall the server from master, delete the other branch (note its last commit for restoring), remove
   both worktrees and the `worktree-agent-*` branches of this run.

## Threats to validity

- One run per task: the variance between two runs of the same arm is unknown and may be as large as the difference.
- The arms start from the same commit but take different paths; one may do more work (more examples updated, an
  unrelated clippy fix) and pay for it.
- Arm A's worktree paths are absolute, which a session in the main checkout doesn't pay; the result is pessimistic
  for easy-peasy until task 148.
- The agent knows grep from training and easy-peasy only from its tool descriptions and the spec.
- Rule breaks (an arm falling back to a forbidden tool) blur the comparison; they are counted, not hidden.

## Runs

### Run 1 — task 144 `output.after = "block"`, server 0.140.0, 2026-10-15

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 221K | 168K |
| Tool calls | 159 | 91 |
| Time | 22.6 min | 21.9 min |
| Search output received | ~75–85K chars | ~70K chars (~15K from searches, the rest reading files) |
| Commits | 8 | 10 |
| Diff | 32 files, +818/−53 (src: 11 files, +235) | 40 files, +894/−68 (src: 18 files, +287) |
| Tests | lib 341 + examples pass | lib 343 + examples pass |
| Clippy | ❌ 17 errors in older tests (clippy 1.99 installed, the 1.98 pin needs rustup) | ✅ fixed in a separate commit |
| Kept to its tool rules | ❌ twice (a python edit, grep on cargo output) | ✅ |

- **Quality:** almost the same design (`After { Lines(n), Block }`, the same closing-line rule, a
  `… N more lines of the block` line with a `more_lines` omit part). A added a rule that a blank matched line opens no
  block; B updated more of the earlier tasks' examples.
- **A's friction:** absolute worktree paths repeated everywhere; `group_repeated` dropped the context lines asked for;
  `line(text = ...)` compares indentation and quietly found nothing; `omit: ["all"]` failed with an unclear error;
  replace templates can't keep indentation; nothing to filter command output.
- **B's friction:** a word search can't skip comments (~60 lines for 3 real uses); a first search too narrow missed
  `CONFIG.md`.
- **Outcome:** B merged. Tasks 148 (paths relative to the only working directory), 149 (a repeated line keeps its
  context), 150 (the error of "all" in a list of parts), 151 (the indentation in a replace template).

### Run 2 — task 150 "all" in a list of parts, server 0.146.0, 2026-10-15

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 122K | 98K |
| Tool calls | 62 | 51 |
| Time | 9.5 min | 11.6 min |
| Search output received | ~30–35K chars (15 easy-peasy calls) | ~12K chars (~20 commands); Read ~30K more |
| Commits | 6 | 6 |
| Diff | 10 files, +400/−18 (src: +49/−10) | 10 files, +382/−15 (src: +37/−7) |
| Examples | 8 (adds wrong values, a list with another part) | 6 |
| Tests | lib 357 + all targets pass | lib 356 + all targets pass |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ once (grep on cargo output), then `cut`/`head` | ✅ |

- **Quality:** the same decision (accept a list with `"all"` as `"all"`, minor bump to 0.147.0) and the same code: a
  helper `omit_list` in `src/config/keys.rs` in place of `are_parts`. B left the old doc line of `are_parts` above
  the new one. Neither updated the `find` schema (`src/description.rs`, TOOLS.md): its array items still exclude
  `"all"` (and `more_lines`, missing since task 144).
- **A's friction:** two over-broad queries cost ~21K of the ~33K chars: `text has 'show'` also matched `shown` (needs
  `/w`) and `name ~ '1[34]?__…'` (`?` is any character); `name = '149__TASK__*'` used `=` with wildcards and found
  nothing; the legend lists the sub-conditions of a `line(...)`.
- **B's friction:** an error text built by `format!` needed a second grep; long example lines needed `cut -c1-200`;
  a path in a shell variable was refused by the worktree guard.
- **Outcome:** a cheaper run than run 1 for both arms; easy-peasy still cost ~25% more tokens, mostly from the
  agent's own over-broad patterns rather than from the response format.
- **Merged:** A, with a commit that adds `"all"` and `more_lines` to the items of `omit` and `show` in the `find`
  schema (and E117.2). Version 0.147.0. Tasks 153 (a hint for `=` with a wildcard pattern) and 154 (the legend of a
  `line(...)` without one entry per sub-condition).

### Run 3 — task 151 `{indent}` in replace templates, server 0.147.0, 2026-10-15

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 131K | 120K |
| Tool calls | 77 | 63 |
| Time | 13.6 min | 13.0 min |
| Search output received | ~22–26K chars (1 `get_config`, 16 `find`, 2 `replace`) | ~60–65K chars (~22 commands); ~10–15K from searches, the rest `cat`/`sed -n` of AGENTS.md, TOOLS.md, examples |
| Commits | 9 | 6 |
| Diff | 14 files, +417/−12 (src: +78/−2) | 14 files, +534/−19 (src: +75/−12) |
| Examples | 4 (E151.1–E151.4) | 10 (E151.1–E151.10) |
| Tests | lib 359 + all targets pass | lib 358 + all targets pass |
| Clippy, fmt | ✅ | ✅ |
| Context: start → end | 31.5K → 131K (+15.2K from loading the 6 easy-peasy tool schemas) | 31.3K → 120K |
| All tool results | ~121K chars (Read 86K, find 16K, Bash 12K) | ~119K chars (Bash 100K, Read 18K) |
| Edits | 19 `Edit`, 6 `Write` | 3 `Edit`, 3 `Write`, 3 `sed -i`, 1 heredoc, 3 `python3` scripts |
| Kept to its tool rules | ❌ ~7 times (`grep`/`head` on cargo output), once `git ls-tree` to list files | ❌ 3 multi-replacement edits with `python3` (a 4th refused by the worktree guard) |

- **Quality:** the same design: `Part::Indent`, the leading spaces and tabs of the line where the match starts (the
  whole indentation also for a match inside or after it, the first line of a multiline match, a match at a line break
  belongs to the line it ends), no indentation after a template line break on its own, not escaped by `/e`. They
  differ in two places:
  - the hint: A adds `{indent}` to every `invalid_template` hint (E129.12 changed); B only to `unknown placeholder`
    and `empty placeholder`, not to a wrong group reference (E129.12 kept)
  - `/n`: B leaves `{indent}` out of a restyled `/n` match (its tabs would end up inside a word of the name) and
    says so in the spec; A leaves `/n` as before
  - A computes the indentation inside `expand`; B passes it in, which changes the signature and every test call
  - B's spec lost a paragraph break: the last `{indent}` bullet runs into "Errors (`invalid_template`) …"
    (TOOLS.md:1108); both leave one new line over 120 columns
- **A's friction:** `replace` writes the absolute directory as a header (`/home/…/agent-…/src/`) where `find` writes
  `in:` once and relative paths (reproduced); `count_by = "file"` with one file repeats the totals on its file line
  (reproduced); `⇥N` markers make code listings noisy (as specified). The base commit was stale again (6ff6d67); both
  arms reset to master.
- **B's friction:** structure-blind search: whole spec sections printed with `sed -n` ranges to find one table;
  the worktree guard refused three commands that only looked risky; checking literal tabs in examples needed
  `grep -cP '\t'`.
- **Why A cost more:** the tool schemas, not the searches. Loading the six easy-peasy tools (three never used) added
  15.2K tokens to A's context, more than the whole 11.7K difference; without it A grew ~4K less than B. Both received
  ~120K chars of tool results (both read AGENTS.md whole); A also read `src/template.rs` (12K, `rename` templates,
  out of scope) and paid for two wrong examples and a separate fmt commit. A's 14 extra calls are edits: one `Edit`
  per change, where B batched several replacements into one `python3` script.
- **Outcome:** the closest run so far: A cost ~10% more tokens (run 2: ~25%, run 1: ~30%), all of it the fixed cost
  of the tool schemas, which runs 1 and 2 paid too.
- **Merged:** B (10 examples, `/n` handled, E129.12 kept), with a commit that restores the paragraph break after the
  last `{indent}` bullet. Version 0.148.0. Tasks 156 (smaller tool descriptions and schemas), 157 (paths relative to
  the only working directory in the write tools), 158 (the counts of the only file are not repeated). From run 4 both
  arms may edit and write files with `python3` scripts.
