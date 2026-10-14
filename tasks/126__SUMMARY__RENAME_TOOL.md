# Summary of task 126: `rename` tool

An agent which found files with `find` can rename or move them in one call: every file of a result or of a query
gets a new path made from a template, with the whole plan checked for conflicts before anything changes. It is the
second write tool; the enabling, the selection, `expect`, `dry_run` and the response are the shared rules of task 125.

## What was built

- **rename** ([TOOLS.md#rename](../TOOLS.md#rename)): listed with `server.allow_write` only, after `delete`;
  `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: false`, `openWorldHint: false`; without writing
  allowed `write_disabled`
- **The target** ([TOOLS.md#the-target](../TOOLS.md#the-target)): `to` with `{dir}`, `{name}`, `{stem}`, `{ext}`,
  `{path}` of the old path relative to the working directory, `{{` / `}}`; `pattern` (Rust regex, checked like `=~`,
  anchored at both ends) with its groups `{1}`, `{2}`, `{word}`, an unmatched group empty, a path which doesn't match
  skipped with `pattern_mismatch`; the target normalized (empty segments and `.` dropped, `..` removes the segment
  before it), an empty one skipped with `invalid_target`, one outside of the allowed roots or an allowed root itself
  with `permission_denied`, the old path itself listed as `unchanged`; the missing directories created (not in a dry
  run)
- **Errors of the template**: `invalid_template` with the position in `to` (relative target, unknown placeholder,
  missing group, empty placeholder, unclosed `{`, unmatched `}`, a trailing `/`, in this order), the hint listing the
  placeholders and the groups; `invalid_regex` with the position in `pattern`; `invalid_option` for `to`, `pattern`,
  `on_conflict` and a named group with the name of a placeholder. In JSON the field of the position is `to` or
  `pattern`
- **Conflicts** ([TOOLS.md#conflicts](../TOOLS.md#conflicts)): `same_target`, `selected`, `exists` and with
  `overwrite` `directory`; `on_conflict = "fail"` (default) fails with `rename_conflict` listing them (at most 10 in
  the text, all in JSON as `conflicts`), also in a dry run; `skip` skips them with `target_conflict`; `overwrite`
  moves an existing file or symlink to the trash and lists the entry as `replaced`, the other conflicts fail
- **Renaming**: in path order until the deadline or the cancellation; `fs::rename`, across file systems
  (`ErrorKind::CrossesDevices`) a copy of the tree (symlinks as links) and the source deleted permanently, a failed copy
  removed; a target which appeared after the check is skipped with `target_conflict`; no trash for a replaced target
  is `trash_unavailable`
- **Response** ([TOOLS.md#result-4](../TOOLS.md#result-4)): `renamed: N  unchanged: N  skipped: N ...`, every entry
  `<old> -> <target>` (the target with the prefix of the working directory), `  replaced`, `<old>  unchanged`; JSON
  `"to"`, `"replaced": true`, `"unchanged": true`; an unchanged entry counts as done for `isError`
- new codes: errors `invalid_template`, `rename_conflict`; warnings `pattern_mismatch`, `invalid_target`,
  `target_conflict` ([OUTPUT.md](../OUTPUT.md#warnings))

## Architecture

- `src/template.rs` (new): `Template::parse` / `expand`, `Groups` (the groups of a compiled pattern and the hint),
  `normalize`
- `src/rename.rs` (new): `parse` (`RenameRequest`: template, anchored pattern, `OnConflict`), `rename` with `How`
  (allowed roots, stop, trash, dry run): `plan` per entry → `Step` (`Missing`, `Unchanged`, `Move`), `conflicts`,
  then `act` / `move_entry` / `copy_tree`
- `src/write.rs`: `Entry` has its working directory and the prefix of its shown path (`rel()`); `check` (archive
  member, allowed roots, the stamp of a result) is shared by `act` and `rename`; `Outcome::skip`; `Acted` has `to`,
  `replaced`, `unchanged`; `remove` moved here from the engine; `RENAME_FIELDS`
- `src/error.rs`: `Report.field` (the JSON name of the position) and `Report.details` (boxed lines before the hint
  and their JSON field); `QueryError::into_field_report`
- `src/engine.rs`: `rename` in `TOOLS`, `write_tool` (the shared `write_disabled` and JSON errors), `selection` (the
  settings, the deadline, the entries of a result or a query) shared by `try_delete` and `try_rename`
- `src/description.rs`, `src/mcp.rs`: the description, the schema and the annotations of TOOLS.md

A request flows: `mcp` → `Engine::call_with("rename")` → `write::parse` → `rename::parse` → `selection` →
`write::check_expect` → `write::select` → `rename::rename` → `Outcome::to_text` / `to_json`.

## Deviations from the plan

- `Report` gets one boxed `Details` instead of two fields, so the error stays small (`clippy::result_large_err`)
- the priority of the reasons of one entry, which the spec doesn't fix: `same_target`, then `selected`, then
  `exists` / `directory`; with `overwrite` the error lists only the conflicts it can't resolve
- a `pattern` without groups gives the hint `groups need a 'pattern'`, like no `pattern`

## Tests

- examples E126.1–E126.14 (`tests/startup.rs`; E126.2 in `tests/examples.rs`) and E125.3 with `rename` listed
- `tests/mcp_stdio.rs`: the five tools of TOOLS.md with `--allow-write`
- unit tests: `template` (every placeholder on `src/app/main.test.ts`, `README`, `.gitignore`, a directory; groups by
  number and name, an unmatched group; `{{ }}`; every error with its position; `normalize`), `rename` (the conflicts
  exists, same target, selected, directory; skip and overwrite; the copy of a tree with a symlink and a failed copy;
  the options), `write` (targets, replaced, unchanged in both formats, `isError` with only unchanged entries), `error`
  (a position in `to`, the lines before the hint, the JSON field names)

## Limitations and known gaps

- the copy across file systems is not run by the examples (one temp file system); the unit test calls the copy
  directly
- case-only renames on case-insensitive file systems: `unchanged` is decided by the canonical paths, so `A.txt ->
  a.txt` may be seen as unchanged there
- a target inside a selected directory which is moved in the same call is not detected as a conflict
- one run of the startup examples failed once under load and passed on every rerun; the failing example wasn't
  captured

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --allow-write
```

```json
{"query": "name ~ '*.js'", "to": "{dir}/{stem}.ts", "dry_run": true}
```

## Version

`0.128.0`
