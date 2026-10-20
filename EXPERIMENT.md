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
each branch base when comparing. The cause (run 5): a worktree starts from `origin/HEAD`, and `master` had not been
pushed since that commit; from run 6 `master` is pushed after every merge, before the build and install.

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

### Run 4 — task 152 the number of matches left out when it is the number of lines, server 0.150.0, 2026-10-16

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 173K | 193K |
| Tool calls | 88 | 107 |
| Time | 26.0 min | 27.5 min |
| Search output received | ~75–85K chars (1 `get_config`, ~22 `find`, 6 `replace`); ~10K of it two `replace` responses that print a 5K-char line whole | ~110–130K chars (~70 commands: ~38 `grep`, ~15 `sed -n`, `awk`, `ls`, `cat`); ~15K greps of test logs |
| Commits | 7 | 7 |
| Diff | 38 files, +1042/−560 (src: 4 files, +73/−17) | 38 files, +1203/−574 (src: 4 files, +147/−25) |
| Examples | 9 (E152.1–E152.9) | 8 (E152.1–E152.8) |
| Tests | 386 pass (all targets) | 387 pass (all targets) |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ once: a leftover `grep -c` in a commit command | ✅ |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone. The schemas of task 156
(−9.4K chars) were loaded by A in this run.

- **Base:** both worktrees started on 662f4cf (task 155), before task 156 was merged, so both bumped to 0.150.0,
  which task 156 already took; the merged branch needs a rebase on master (E117.2 and the `find` description conflict)
  and 0.151.0.
- **Quality:** both are complete and green, with a new `omit` part in the spec 2 defaults, JSON unchanged, the same
  earlier examples fixed (E145.5 with `show`, E155.6 cursor, E155.7 sizes) and one line in the tool description. The
  rule differs, the choice the task left to the spec step:
  - A (`equal_matches`): every count on its own: a file's `matches: N` is left out when it is its `lines: N`, the
    header's when it is the header's `lines: N`, `: 1` of `count_by = line` only while the line number is shown.
    Saves more in mixed results; a left-out count says "one match per line" only for its own file. Not handled: a
    file whose lines have 0 and 2 matches (a multiline match) is `lines: 2` and reads as one match per line.
  - B (`match_counts`): all or nothing: the counts are left out only when every matched line of the whole result has
    exactly one match (cursor pages, `max_files` and the budget don't change it), so a response without `matches:`
    means one match per line everywhere; the 0-and-2 case keeps the counts. A whole new OUTPUT.md section with the
    edge cases; twice the code. Quirk: `: 1` of `count_by = line` belongs to `line_counts` and `match_counts`, so
    `show = ["line_counts"]` alone doesn't bring it back (in the spec).
- **A's friction:** `replace` prints a changed line whole: a 20-char change in the 4.8K-char JSON line of E117.2 is
  ~9.6K chars of dry run (reproduced on 0.150.0, no `max_line_length` in the write tools); a legend with
  `highlight = "none"` (reproduced, task 154); `repeated:` with 3 locations (as specified, `repeated_locations`);
  `detail = count` without a text condition gives `lines: 0` (as specified); no backreferences in Rust regex (out of
  scope); finding failing examples meant parsing the cargo log (out of scope).
- **B's friction:** greps over the startup test log matched single-line JSON-RPC messages (one 41KB result); whole
  sections read with `sed -n` ranges; the worktree guard refused a `for` loop around `sed -i`, a shell variable passed
  to python and a heredoc; its own block parser broke on fences inside a response and had to be redone.
- **Outcome:** the first run where A is cheaper: −11% tokens, −18% tool calls, −1.5 min. Run 3 was +10% for A, almost
  all of it the tool schemas that task 156 has since halved.
- **Merged:** B (the whole-result rule, the multiline 0-and-2 case handled), rebased on master after task 156: the
  conflicts in TOOLS.md, `src/description.rs` and E117.2 (the `omit` enums of the old schema are now the list of
  parts in its description, with `match_counts`), E156.1, E156.5 and E156.6 updated for the new rule, version
  0.151.0. Task 159 (long changed lines of `replace` cut around the change).

### Run 5 — task 153 a hint for `=` with a wildcard pattern, server 0.151.0, 2026-10-20

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 133K | 100K |
| Tool calls | 78 | 48 |
| Time | 15.9 min | 11.1 min |
| Search output received | ~25–30K chars (1 `get_config`, 19 `find` calls, 21 queries); ~8K of it one over-broad `count_by = file` | ~75–85K chars in all (~22 search commands: ~13 `grep`, ~8 `sed -n`, `ls`, `find`); most of it whole reads of AGENTS.md, OUTPUT.md, QUERY.md, `src/hints.rs` |
| Commits | 7 | 6 |
| Diff | 14 files, +401/−13 (src: +92/−6) | 14 files, +377/−12 (src: +71/−4) |
| Examples | 8 (E153.1–E153.8) | 9 (E153.1–E153.9) |
| Tests | lib 362, 387 in all targets pass | lib 361 + all targets pass |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ ~13 `grep`/`head`/`tail` on cargo output | ✅ |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone.

- **Base:** both worktrees started on 662f4cf (task 155) again, before tasks 156 and 152 were merged, and neither
  reset to master; both bumped to 0.150.0. The merged branch needs a rebase (conflicts only in `Cargo.toml`,
  `Cargo.lock`, `tasks/README.md`), version 0.152.0, and its examples re-run under task 152's `match_counts` rule.
- **Quality:** the same design in the same place: one more variation of task 103 in `variations`
  (`src/query/mod.rs`), `~` in place of `=` for `name` / `path` when the value has `*`, `?`, `[` or `{`, right after
  `has instead of =`, the modifiers kept, `text =` left out, `src/hints.rs` unchanged, the same `has_wildcard`.
  Both also drop the similar-names hint for a wildcard value. They differ in:
  - lookups: A also drops the similar directories of a directory with a wildcard (`path = '**/x.rs'`,
    `path starts 'src/*/'`); B only the similar names, so `path = '**/x.rs'` still looks up the directory `**`
  - `/n`: B leaves the variation out with `/n` explicitly; A keeps it, and the existing rule (a variation that doesn't
    parse is not tried) leaves it out, which A writes into the spec
  - the spec note: A in QUERY.md#wildcards (inline: the spec test reads untagged blocks as queries, which cost A a fix
    commit), B in QUERY.md#gotchas; B leaves one OUTPUT.md line of 151 columns
  - examples: B has one more (`/i` kept, its own example); both test a file really named `*.rs` (Linux only)
- **A's friction:** `dirs` can't be a file (`error[dir_not_found] … is not a directory`, reproduced; as specified,
  `path = '…'` does it); the hints of a huge result with `max_files = 1` set on purpose (`dir:`, `lang:`,
  `narrow with:`, reproduced on 0.151.0: as specified, the threshold is 10 × `max_files`); the legend of a
  `line(A or B or C)` (task 154); `(cut)` of `max_output_per_file` led to a Read (as specified); the worktree guard
  refused a `cd … && python3 - <<EOF` heredoc.
- **B's friction:** a two-stage grep for JSON examples with a funnel found nothing; finding a function took a grep and
  two wide `sed -n` ranges; `ls tests/fixtures` printed 152 names.
- **Outcome:** A cost +33% tokens, +63% tool calls and +4.8 min, the widest gap since run 1, on the smallest task so
  far: a short task leaves the fixed costs (tool schemas, absolute worktree paths, a spec fix commit) less work to
  pay back. A's branch is the more complete one (the directory lookups, the spec note in #wildcards).

### Run 6 — task 154 the legend of a `line(...)` without one entry per sub-condition, server 0.152.4, 2026-10-20

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 137K | 145K |
| Tool calls | 65 | 65 |
| Time | 13.7 min | 18.1 min |
| Search output received | ~22K chars (1 `get_config`, 13 `find`, 5 `replace`: 2 dry runs) | ~45–50K chars (~22 search commands: ~14 `grep`, ~8 `sed -n`, `ls`, `cat`/`head`, `wc`; ~8 Read); ~25K of it the whole AGENTS.md |
| Commits | 6 | 6 |
| Diff | 12 files, +556/−15 (src: 3 files, +158/−6) | 12 files, +636/−15 (src: 3 files, +160/−7) |
| Examples | 7 (E154.1–E154.7) | 7 (E154.1–E154.7) |
| Tests | 393 pass (all targets) | 393 pass (all targets) |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ 3 slips: `ls tests/*.rs`, `wc -l`, `tail`/`grep` on cargo output | ✅ |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone.

- **Base:** both worktrees started on 551112a, the current master: pushing master after every merge (from run 6)
  fixed the stale base of runs 2–5.
- **Quality:** the same design in the same place: `Written` and `written_legend()` in `src/output/omit.rs`, the
  legend filtered in `respond` (`src/engine.rs`) over the whole result, a label always kept, a sub-group
  (`@def.fname`) bringing its condition, `show = ["legend"]` and spec 1 unchanged, no earlier example changed. They
  differ in:
  - `highlight = "none"`: A changes the legend rule (the ids of the matches count only with a highlight that writes
    them), so the legend part is left out whole and a labelled condition goes with it; B filters the entries and
    leaves out a legend with none left, so `@w text has '…'` stays alone (reproduced on both binaries). The task
    keeps labels out of scope: B follows it
  - `facets` detail: B lists the ids the `condition` facet writes (`facet_limit` respected); A lists the conditions
    of all files, so `facet_limit = 1` gives `#1 #2 #3` for a facet line that shows only `#1` (reproduced)
  - B also mentions the rule in the JSON format section; B leaves one OUTPUT.md prose line of 163 columns; A leaves
    a stale doc comment on its test legend (`@x text =~ ...`)
- **A's friction:**
  - the version bump `replace` on `path ~ 'Cargo.{toml,lock}'` silently skipped `Cargo.lock`:
    `traversal.skip_generated` (default `true`) treats it as generated, and a non-empty result says nothing about
    skipped files, even with `show = ["warnings"]` (reproduced); `cargo build` updated it instead
  - `name in ('Cargo.toml','Cargo.lock')` → `operator_not_allowed` with no hint to `name ~ '{a,b}'` (reproduced)
  - the spec 2 limits (`max_matches_per_file = 10`, `max_output_per_file = 800`) cut heading listings (as
    specified); `⇥N` lines between scattered matches (as specified, dedent); no way to print a whole file (out of
    scope, Read); the task's own bug seen live (a six-entry legend with `highlight = "none"`)
- **B's friction:** `grep -rn -i legend src` gave 127 hits, mostly tests, cut to 170 columns by hand; listing the
  spec 2 example files needed a `for` loop with `grep -q`; the worktree guard refused a `cd; python3` heredoc; it
  wrote its own MCP client to get exact responses for the examples (A used the tests); one flaky
  `startup_examples` failure under load, not reproduced.
- **Outcome:** A cheaper on every count for the second time: −6% tokens, the same tool calls, −4.4 min, with half
  the search output. B's branch is the more faithful one (labels kept with `highlight = "none"`, the facet limit).
- **Merged:** B, with a commit that wraps two OUTPUT.md lines at 120 columns. Version 0.153.0. Tasks 161 (the
  skipped files a non-empty result names) and 162 (a hint for `in` with a list of names).

### Run 7 — task 157 paths relative to the only working directory in the write tools, server 0.153.0, 2026-10-20

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 148K | 133K |
| Tool calls | 74 | 60 |
| Time | 18.0 min | 12.4 min |
| Search output received | ~18–22K chars (1 `get_config`, ~30 `find`, no write tool) | ~45–55K chars (~22 search commands: ~16 `grep`, ~10 `sed -n`, `ls`, `find`, `wc`, `cat`; ~6 Read); ~25K of it the whole AGENTS.md |
| Commits | 6 | 8 |
| Diff | 12 files, +686/−19 (src: 3 files, +176/−12) | 16 files, +652/−16 (src: 2 files, +162/−7) |
| Examples | 9 (E157.1–E157.9, the `one-dir` fixture of task 148) | 7 (E157.1–E157.7, a new fixture `write-one-dir`) |
| Tests | lib 370 + all targets pass | lib 370 + all targets pass |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ 2 slips: a `grep -c` in a commit command, `grep -E` on `cargo build` output | ✅ (by its report) |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone.

- **Base:** both worktrees started on cdc05a8, the current master.
- **Quality:** the same design and almost the same code: `Selected` carries the working directories, `write_output`
  takes the outcome by value and applies task 148's `in_dirs`, `Outcome.in_dir` and `Outcome::relative_to` in
  `src/write.rs`, a lexical relative path with `..` for a target outside, the `in:` line only when the response writes
  a path (entries or warnings), JSON `"dirs"`, the paths of `rename_conflict` errors unchanged. They differ in:
  - the configuration: A applies the rule only with the configured `path_style = relative` and without a configured
    `show = ["dir"]`, and a configured `omit = ["dir"]` drops the `in:` line, as in `find`; B always writes it in
    spec 2 and says so ("the write tools have no `path_style` and no `omit`"). Reproduced on both binaries with a
    `--config`: with `path_style = "absolute"` A keeps the absolute headers, B writes `in:` (the task leaves
    `path_style = absolute` out of scope)
  - A's relative path strips the prefix first, then normalizes both; B's compares the components as they are
  - B also updates the paths a write tool accepts (TOOLS.md) and the JSON note; A states `dirs` before `files` (true
    by the alphabetical key order, which B checked with `cargo tree`)
- **A's friction (all reproduced on 0.153.0, as specified):** an outline query cut by `max_matches_per_file = 10`
  although `max_output_per_file` was raised; a query without a text condition shows no lines with `context`, so
  dumping small files needed `line(text =~ "^")` (Read is the tool for it); `name in (...)` (task 162); labels show
  no matched condition with `highlight = "none"` (as specified).
- **B's friction:** spec sections read by `grep -n '^#'` and guessed `sed -n` ranges that over-fetched; a grep for
  absolute `dirs` in earlier examples needed `-A1` around the request; the worktree guard refused a compound command
  with a shell variable; JSON key order needed `cargo tree` and a failing test.
- **Outcome:** B cheaper: A cost +11% tokens, +23% tool calls and +5.6 min, with less than half of B's search
  output: the extra cost is not in the search responses. No new tasks.
- **Merged:** A (the configured `path_style`, `show` and `omit` of `find` respected, 9 examples on the fixture of
  task 148), with a commit that corrects the deviations of its summary. Version 0.154.0.

### Run 8 — task 158 the counts of the only file are not repeated, server 0.154.0, 2026-10-20

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 156K | 156K |
| Tool calls | 96 | 92 |
| Time | 19.0 min | 20.6 min |
| Search output received | ~25–30K chars (1 `get_config`, 29 `find`: 3 errors, 3 batches of two; no write tool); 32 Read | ~95–100K chars (~32 search commands: ~17 `grep`, ~10 `sed -n`, `ls`, `awk`, `head`/`cat`/`wc`; no Read); ~25K of it OUTPUT.md sections, ~17K example failures, ~12K one `ls` of absolute paths |
| Commits | 6 | 7 (a red-tests commit) |
| Diff | 25 files, +465/−99 (src: 2 files, +68/−9) | 27 files, +574/−93 (src: 4 files, +148/−2) |
| Examples | 6 (E158.1–E158.6) | 7 (E158.1–E158.7) |
| Tests | 396 pass (all targets) | 396 pass (all targets) |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ once: `cargo build \| tail -20` | ✅ |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone.

- **Base:** both worktrees started on 935cc85, the current master.
- **Quality:** the same rule, spec section `OUTPUT.md#one-file` and the same earlier examples changed (13 example
  files and `tests/mcp_stdio.rs` byte for byte the same, E155.6 cursor and E155.7 size included): in spec 2 text
  format the only file of the whole result (not of a page) is written by its whole path, without `file_counts` and
  without a directory header; with `omit = ["counts"]` it keeps its counts; `show = ["file_counts"]` brings them back;
  JSON, spec 1 and `since` unchanged. They differ in:
  - `count_by = line` (out of scope in the task): A applies the rule (`Cargo.toml` and its line counts), recorded as a
    deviation; B leaves it unchanged and has an example of it
  - `show = "all"`: B writes the spec 1 layout (`./` and the counts), as the spec of `show = "all"` says; A keeps the
    whole path ("a layout of spec 2"), only the counts come back. Both reproduced on the built binaries
  - code: A adds `Differ::one_file` and `FileCounts` to the omit rules in `src/output/omit.rs`, `group_dirs` off in
    `src/engine.rs` (with `#[allow(clippy::struct_excessive_bools)]`); B adds `output::OneFile` to `Response` and
    applies it in `push_file`, which adds a line to every test constructor of `Response`
  - B extends a prose line of OUTPUT.md that was already over 120 columns
- **A's friction:**
  - `line(text has 'fn ' /w)` found nothing: a `/w` value ending with a space can never match (as grep `-w`); the
    funnel shows 0 without a hint (reproduced)
  - `traversal.ignore_files = false` → `must be a list of strings`, no hint that `[]` turns the files off
    (reproduced); `traversal.gitignore` lists the valid keys
  - an escaped quote in a query failed with `unexpected_token`: not reproduced (the JSON escaping of the agent)
  - `max_output_per_file` cut a heading listing after `max_matches_per_file` was raised; `⇥N` lines without
    context (both as specified); test logs in the scratchpad are outside the allowed roots (written to `target/`)
  - the task's own repetition seen in almost every one-file response
- **B's friction:** a first fix script rewrote ~110 spec 1 example files (reverted; spec 1 files needed a
  `grep -L`); the examples test stops at the first failing step, so a grep of its log under-reported; the worktree
  guard refused a `sed` loop with computed ranges, a heredoc with `git` and a pattern with "hash"; the batch cursor
  change took several greps through three files and a python FNV check.
- **Outcome:** a tie: the same tokens (156.0K vs 155.8K), A +4 tool calls and −1.6 min, with less than a third of B's
  search output. B's branch keeps to the task's scope (`count_by = line`) and to the spec of `show = "all"`.
- **Merged:** B, with a commit that wraps an OUTPUT.md line at 120 columns. Version 0.155.0. Tasks 163 (a hint for
  `/w` with a value that starts or ends with a space) and 164 (a hint for `false` given to a list option).

### Run 9 — task 159 long changed lines of `replace` cut around the change, server 0.155.0, 2026-10-20

| | A: easy-peasy only | B: grep/sed/find |
|---|---|---|
| Total agent tokens | 176K | 151K |
| Tool calls | 89 | 72 |
| Time | 17.5 min | 15.7 min |
| Search output received | ~18K chars (1 `get_config`, 20 `find`: 1 error, 4 `replace`: 2 dry runs, 1 error); 23 Read | ~75K chars in all (~32 commands: ~16 `grep`, 4 `sed -n`, 5 `ls`, ~5 `head`/`tail`, `cat`, `awk`; 11 Read); ~10K of it searches, the rest AGENTS.md, TOOLS.md, OUTPUT.md sections |
| Commits | 7 | 7 |
| Diff | 18 files, +613/−40 (src: 6 files, +208/−29) | 17 files, +592/−46 (src: 6 files, +207/−36) |
| Examples | 8 (E159.1–E159.8) | 8 (E159.1–E159.8, a new fixture `long-lines`) |
| Tests | 398 pass (all targets) | 398 pass (all targets) |
| Clippy, fmt | ✅ | ✅ |
| Kept to its tool rules | ❌ ~5 slips: `wc -l`, `head` on `git log`/`git diff`, `grep`/`head`/`tail` on cargo output | ✅ |

Total tokens are the whole run (reading, thinking, editing, cargo), not the search output alone.

- **Base:** both worktrees started on f9d5168, the current master.
- **Quality:** almost the same code: `Change.old_parts` / `new_parts` (the byte ranges of the matches and of the new
  texts in each line, a deleted match an empty part), a helper `parts_in`, `windows` of `src/output/lines.rs` taking
  byte ranges, a shared `cut_plain`, `Outcome.max_line_length`, `max_line_length` in the `replace` schema without a
  description (with one the schema is over the 1900 characters of `schemas_stay_small`, both found). They differ in:
  - JSON: A cuts `old` and `new` like the text, without a `"cut"` flag (`find`'s JSON has one), so code can't tell a
    cut line; B keeps them whole, as the task suggested ("its fields are read by code")
  - spec 1: A always writes the lines whole and ignores a `max_line_length` of the request; B cuts them when the
    request sets it (the field is new, spec 1's default unchanged)
  - the spec: A writes a section of its own, TOOLS.md#long-changed-lines, with a worked example; B puts the rule
    into the `replace` result bullets (TOOLS.md#result-6, the anchor of the 7th "Result" heading) and one line in
    OUTPUT.md#long-lines, fixes a stale `result-5` comment and leaves two TOOLS.md prose lines over 120 columns
- **A's friction:**
  - a `replace` dry run on `Cargo.toml` and `Cargo.lock` silently selected only `Cargo.toml`
    (`traversal.skip_generated`): task 161, already written in run 6
  - `near(line(...), ...)` → `nested_line` with a clear hint (reproduced, as specified); `max_matches_per_file = 10`
    cut heading lists, `files` detail writes `lines:`, `count_by = file` without `matches:` when they equal the
    lines, `⇥N` lines (all as specified); the scratchpad is outside the allowed roots (out of scope)
- **B's friction:** the anchor of the 7th "#### Result" in TOOLS.md counted by hand; broad greps of OUTPUT.md mostly
  irrelevant; spec sections read by guessed `sed -n` ranges of 6–22K characters; the worktree guard refused a `cd`
  with a heredoc and a `for` loop around `sed`.
- **Outcome:** B cheaper: A cost +16% tokens, +24% tool calls and +1.8 min, with a quarter of B's search output.
  No new tasks.
- **Merged:** B (JSON lines whole, spec 1 cut only on request), with a commit that wraps two TOOLS.md lines at 120
  columns. Version 0.156.0.
