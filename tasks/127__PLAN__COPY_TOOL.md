# Plan for task 127: `copy` tool

## Design

The spec change is committed already ([TOOLS.md#copy](../TOOLS.md#copy), [OUTPUT.md#errors](../OUTPUT.md#errors),
[OUTPUT.md#warnings](../OUTPUT.md#warnings)). `copy` is `rename` with another last step, so it shares its module.

### The tool — `src/rename.rs`

- `Mode` (`Rename`, `Copy`) in `How`: the name of the count, the error code of the conflicts (`rename_conflict`,
  `copy_conflict`) and its hint (`renames` / `copies the other files`), and what is done with a planned entry
- `parse` stays shared: `to`, `pattern`, `on_conflict` are the same fields
- `conflicts`: a new `Reason::Inside` (`inside`, `the target is inside the copied directory`) for `Copy`, when the
  entry is a directory and the target is below its old path; checked after `same_target` and `selected`, before
  `exists`; `overwrite` doesn't resolve it
- acting with `Copy`, in path order until the deadline or the cancellation: the missing directories of the target,
  `copy_tree` into a temporary name next to the target (`.<name>.<pid>-<n>.tmp`), then a target which appeared →
  the temporary copy removed and `target_conflict`; a replaced target → `Trash::put` (`trash_unavailable`, the
  temporary copy removed); `fs::rename` into place; a failed copy or rename removes the temporary copy
- `copy_tree` keeps the modification times: of a file and of a symlink after it is written, of a directory after its
  tree; the permissions of a file (`fs::copy`) and of a directory. The cross file system move of `rename` uses it too,
  so a moved tree keeps its times as well

### Shared parts

- `write::COPY_FIELDS` (the fields of `rename`); the engine's `try_rename` becomes `try_transfer(mode)`, `copy` is
  listed after `rename` in `Engine::TOOLS` and `WRITE_TOOLS`
- `ErrorCode::CopyConflict` after `RenameConflict`
- `description`: `COPY_DESCRIPTION`, `copy_schema()`; `mcp`: `copy` with `idempotentHint: true`
- the warnings are the ones of `rename`; no new code

### Dependencies

- `filetime` moves from the dev-dependencies to the dependencies: std can set the modification time of an open file
  only, not of a symlink, and opening a directory needs platform flags on Windows; `filetime` does both on every
  platform and is already used by the tests

## Rejected

- a separate `copy` module duplicating the plan and the conflicts: the two tools differ only in the last step
- copying straight to the target: a failed copy would leave a partial file, the spec asks for a temporary name
- `selected` resolved for copies (the source isn't changed by a copy, so `a -> b` with `b` selected is harmless when
  `b` is copied first): the order would matter again; it stays a conflict like in `rename`

## Tests

- examples E127.1–E127.12 (`tests/startup.rs`, E127.2 in `tests/examples.rs`); E125.3 and E126.1 list `copy`
- `tests/mcp_stdio.rs`: the six tools of TOOLS.md with `--allow-write`
- unit tests in `rename`: a copied tree keeps contents, times, permissions and links and leaves no temporary file;
  a failed copy leaves nothing; the `inside` conflict and its hint; overwrite with `Copy` keeps the source;
  `error`: the code `copy_conflict`

## Steps

1. tests (red)
2. `error`; `rename` (mode, inside, copy step, times); `write`; `engine`, `description`, `mcp`; `Cargo.toml`
3. definition of done, summary, version `0.129.0`

## Risks

- the modification time of a symlink can't be set on every platform (`filetime` returns an error where it can't):
  the error is ignored for symlinks only, as the spec allows
- the temporary name could collide with a file of the user: the pid and a counter make it unlikely, and a temporary
  name which exists is a failed entry, never overwritten
