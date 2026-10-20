# Summary of task 157: Paths relative to the only working directory in the write tools

Server version: `0.154.0`.

## What was built

- In spec 2 `replace`, `rename`, `copy` and `delete` with one working directory given as an absolute path write it
  once, as `find` does since task 148
  ([TOOLS.md#one-working-directory-of-a-write-tool](../TOOLS.md#one-working-directory-of-a-write-tool)): the line
  `in: <dir>/` right after the header, the entries, the directory headers, the targets of `rename` and `copy` and the
  paths of the warnings relative to it; JSON `"dirs": ["<dir>/"]`. The dry run and the real call alike.
- A target outside of the working directory is written with `..` (`src/a.rs -> ../other/a.rs`), as `to` resolves it,
  so every path of the response resolves against the working directory again.
- `result = "rN"`: the working directory is the one of the kept result.
- Unchanged: a relative working directory and `.`, several working directories, `"version": 1`, the configured
  `show = ["dir"]`, `path_style` other than `relative`, the paths of the conflict errors. The configured
  `omit = ["dir"]` leaves out the `in:` line, the paths stay relative. An empty selection has only the header.
- Spec: one commit (TOOLS.md, the versioning table, OUTPUT.md#one-working-directory). A change of a spec 2 rule,
  allowed since spec 2 has not been used outside this repository (TOOLS.md#versioning).

## Architecture

- `engine::Selected` carries the working directories of the selection (the query run or the kept result);
  `try_replace` takes them the same way.
- `engine::write_output` takes the outcome by value and, when `engine::in_dirs` of task 148 gives one working
  directory with `path_style = relative`, calls `write::Outcome::relative_to(dir, line)`.
- `Outcome::relative_to` rewrites `Acted::shown` and `Acted::to` with `write::relative` (a lexical difference of the
  normalized paths) and strips the warnings (`Warnings::strip_prefix`); `Outcome::in_dir` holds the directory of the
  `in:` line. `to_text` and `to_json` write it when the response has entries or warnings about entries.
- The entries keep their full shown paths while the tool acts: `replace` finds the matches of a file by it, and the
  conflict errors and the trash use it, so nothing else changed.

## Deviations from the plan

- none

## Tests

- examples E157.1–E157.9 (startup examples, fixture `one-dir` of task 148): a dry run and a real call of each tool,
  a target in another directory and outside of the working directory, a result with a `file_changed` warning, a path
  copied from a response, JSON, several working directories, a relative one, spec 1, an empty selection
- unit `write::tests::one_working_directory_is_written_once` (text, JSON, without the line, empty) and
  `write::tests::relative_targets`
- no earlier example changed: the write examples use relative working directories

## Limitations and known gaps

- none known

## Try it

`{"dirs": ["/home/me/app/.claude/worktrees/agent-1"], "query": "name = 'b.rs'", "to": "{dir}/d.rs", "dry_run": true}`
with `rename` — `in: /home/me/app/.claude/worktrees/agent-1/` once, then `src/` and `b.rs -> src/d.rs`.
