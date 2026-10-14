# Summary of task 127: `copy` tool

An agent which found files with `find` can copy them in one call: back them up before a change or make new files
from existing ones, every copy at a path made from a template. It is the third write tool; the selection, `expect`,
`dry_run` and the response are the shared rules of task 125, the target and the conflicts the ones of task 126.

## What was built

- **copy** ([TOOLS.md#copy](../TOOLS.md#copy)): listed with `server.allow_write` only, after `rename`;
  `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false`; without writing
  allowed `write_disabled`
- **The target**: `to`, `pattern`, the placeholders, the groups, the normalization and their errors exactly like
  `rename` ([TOOLS.md#the-target](../TOOLS.md#the-target)); a target which is the old path is listed as `unchanged`
- **Conflicts**: `exists`, `same_target`, `selected`, `directory` of `rename` and the new `inside` (a directory copied
  into itself, which `overwrite` doesn't resolve); `on_conflict` `fail` / `skip` / `overwrite`; the error is the new
  `copy_conflict`, with the hint `on_conflict = "skip" copies the other files...`
- **Copying**: in path order until the deadline or the cancellation; the missing directories of the target created;
  the entry copied under a hidden temporary name next to the target (`.<name>.<pid>-<n>.tmp`) and renamed into place,
  a failed copy removed; a target which appeared meanwhile skipped with `target_conflict`, a replaced one moved to
  the trash right before the rename (`trash_unavailable` without a trash)
- **What a copy keeps**: the content byte for byte, the modification time of files, directories and (where the
  platform can set it) symlinks, the permissions of files and directories; a directory with its whole tree, hidden
  and ignored files too; a symlink as a symlink with the same text; `traversal.max_file_size` doesn't apply
- **Response**: `copied: N  unchanged: N  skipped: N ...`, entries `<old> -> <target>`, `  replaced`,
  `<old>  unchanged`; JSON `"copied"` and the entries of `rename`

## Architecture

- `src/rename.rs`: `Mode` (`Rename`, `Copy`) in `How` chooses the count (`renamed` / `copied`), the error code and
  its hint, and the last step: `move_entry` or `copy_into_place` (with `temporary_name`); `Reason::Inside`;
  `copy_tree` keeps the times (`filetime`) and the permissions, shared with the cross file system move of `rename`
- `src/engine.rs`: `copy` in `TOOLS` and `WRITE_TOOLS`; `try_rename` became `try_transfer(mode)`
- `src/error.rs`: `ErrorCode::CopyConflict`
- `src/description.rs`, `src/mcp.rs`: `COPY_DESCRIPTION`, `copy_schema()` (the schema of `rename` with its own
  descriptions of `to` and `dry_run`), the annotations
- `Cargo.toml`: `filetime` moved from the dev-dependencies to the dependencies

A request flows: `mcp` → `Engine::call_with("copy")` → `write::parse` → `rename::parse` → `selection` →
`write::check_expect` → `write::select` → `rename::rename` (`Mode::Copy`) → `Outcome::to_text` / `to_json`.

## Deviations from the plan

- no `COPY_FIELDS`: the fields are the ones of `rename`, so `RENAME_FIELDS` serves both
- the moved trees of `rename` across file systems keep their times too, since the copy function is shared

## Tests

- examples E127.1–E127.12 (`tests/startup.rs`; E127.2 in `tests/examples.rs`); E125.3 and E126.1 list `copy`
- `tests/mcp_stdio.rs`: the six tools of TOOLS.md with `--allow-write`
- unit tests in `rename`: a copied file and tree keep contents, times and links and leave no temporary file; the
  permissions (Unix); a failed copy leaves nothing; `overwrite` keeps the source; the `inside` conflict with its hint,
  and a sibling with the same name prefix which is not inside

## Limitations and known gaps

- the modification time of a symlink is kept only where the platform can set it; a failure there is ignored
- copying a selected file onto another selected file is the conflict `selected`, even where the order would make it
  harmless
- a target inside a selected directory which is copied in the same call is not detected as a conflict (like in
  `rename`)

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --allow-write
```

```json
{"query": "name ~ '*.rs'", "to": "{dir}/{stem}.bak.{ext}", "dry_run": true}
```

## Version

`0.129.0`
