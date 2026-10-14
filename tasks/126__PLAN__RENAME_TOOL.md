# Plan for task 126: `rename` tool

## Design

The spec change is committed already ([TOOLS.md#rename](../TOOLS.md#rename), [OUTPUT.md#errors](../OUTPUT.md#errors),
[OUTPUT.md#warnings](../OUTPUT.md#warnings)).

### The template — `src/template.rs` (new)

- `Template::parse(to, groups)`: `to` into parts — literal text, a placeholder (`Dir`, `Name`, `Stem`, `Ext`,
  `Path`), a group by number or by name; `{{` and `}}` are literal. The errors in spec order: `/` at the start, then
  the braces from left to right (unclosed, unmatched, empty, unknown placeholder, a group which doesn't exist), then
  `/` at the end; each a `QueryError`-like report with the position in `to`
- `Groups`: the number of groups and the names of `pattern`, for the checks and the hint
- `Template::expand(rel, captures)`: the placeholders from the old path relative to the working directory (`/`
  separated; `stem` and `ext` like `Path::file_stem` / `extension`), the groups from the captures, empty when they
  didn't take part
- `normalize(text)`: the segments without empty ones and `.`, `..` removing the segment before it (kept at the
  start); `None` when nothing is left

### The tool — `src/rename.rs` (new)

- `RenameRequest`: `to: Template`, `pattern: Option<Regex>` (checked like `=~` with `query::user_regex`, then
  compiled anchored `^(?:...)$`), `OnConflict` (`Fail`, `Skip`, `Overwrite`); `parse(arguments)` reads these three
  fields, the shared fields go through `write::parse` with `RENAME_FIELDS`
- `plan(entries, ...)`: per entry, after the shared checks of `write`: `pattern_mismatch`, the target expanded and
  normalized (`invalid_target`), joined to the working directory, its existing part canonicalized and checked with
  `write::may_change` (`permission_denied`), the same path → unchanged
- `conflicts(planned, on_conflict)`: `selected` (the target is the checked path of another entry), `same_target`
  (two entries, one canonical target), `exists` / `directory` (`symlink_metadata` of the target); with `Fail`, or
  with `Overwrite` and anything but a replaceable `exists`, the `rename_conflict` report; with `Skip` the entries
  are skipped with `target_conflict`
- acting, in path order until the deadline or the cancellation: a target which appeared → `target_conflict`; a
  replaced target → `Trash::put` (`trash_unavailable`, or failed); `create_dir_all` of the parent; `fs::rename`, on
  `ErrorKind::CrossesDevices` a copy of the tree (symlinks as links) and the source removed permanently, a failed copy
  removed

### Shared parts — `src/write.rs`

- `Entry` gets the working directory and the prefix of its shown path, so a target can be made from the old path
- the checks of `act` before the operation (archive member, allowed roots, the stamp of a result) become
  `write::check`, used by `delete` and `rename`
- `Acted` gets `to: Option<PathBuf>`, `replaced` and `unchanged`; `Outcome::to_text` / `to_json` write
  `old -> target`, `  replaced`, `  unchanged` and the header `unchanged: N`; the count of the tool excludes the
  unchanged entries, `isError` counts them as done
- `RENAME_FIELDS`; `parse` takes the fields of the tool, `permanent` only where it is one of them

### Errors and warnings

- `Report` gets the name of the field of its position (`query`, `to`, `pattern`) and extra lines with their JSON
  (`conflicts`); `to_text` writes the lines before the hint
- `ErrorCode::InvalidTemplate`, `RenameConflict`; `WarningCode::PatternMismatch`, `InvalidTarget`, `TargetConflict`
  after `TrashUnavailable`, in the order of the table

### Engine, MCP

- `Engine::TOOLS` gets `rename` last; `tools()` lists the write tools only with `allow_write`; `call_with` runs both
  write tools through one wrapper (`write_disabled`, the JSON errors); the selection of `try_delete` becomes a shared
  method
- `description`: `RENAME_DESCRIPTION`, `rename_schema()`; `mcp`: `idempotentHint: false`
- no new dependency

## Rejected

- an order of renames which resolves chains (`a -> b`, `b -> c`): `selected` is a conflict instead; the plan stays
  checkable and the result doesn't depend on the order
- `renameat2(RENAME_NOREPLACE)` against a target which appears between the check and the rename: Linux only; the
  target is checked right before each rename instead
- replacing a target permanently: the replaced file goes to the trash, like `delete`, so `overwrite` can be undone

## Tests

- examples E126.1–E126.14 (`tests/startup.rs`, E126.2 in `tests/examples.rs`), E125.3 with `rename`
- `tests/mcp_stdio.rs`: the tools of TOOLS.md with `--allow-write`
- unit tests: `template` (every placeholder on `src/app/main.test.ts`, `README`, `.gitignore`, a directory; groups
  by number and name, an unmatched group; `{{ }}`; every error with its position; `normalize`), `rename` (the
  conflicts: exists, same target, selected, directory; the skip and overwrite resolutions; the copy across file
  systems through the copy function on one file system), `write` (the formats with targets, replaced, unchanged),
  `error` (a position in `to`, the lines before the hint, the JSON field names)

## Steps

1. tests (red)
2. `error`, `warning`; `template`; `write` shared parts; `rename`; `engine`, `description`, `mcp`
3. definition of done, summary, version `0.128.0`

## Risks

- the copy across file systems is not exercised by the examples (one temp file system); the unit test calls the copy
  directly
- case-only renames on case-insensitive file systems: the old and the new path are one file; `unchanged` is decided
  by the canonical paths, so `A.txt -> a.txt` may be seen as unchanged there (a known gap of the task)
