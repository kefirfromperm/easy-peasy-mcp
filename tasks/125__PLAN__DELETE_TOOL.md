# Plan for task 125: `delete` tool

## Design

The spec change is committed already ([TOOLS.md#write-tools](../TOOLS.md#write-tools),
[TOOLS.md#delete](../TOOLS.md#delete), [CONFIG.md#server](../CONFIG.md#server)).

### Enabling

- `config`: `server.allow_write` like `allow_update`: `FileValues.allow_write`, `Settings.allow_write` with its
  source (default `false`), `CommandLine.allow_write` (`--allow-write` in `main.rs`), a line in `get_config` after
  `allow_update`, an entry of `READ_ONLY` (`allow_write is set only at start`)
- `engine`: `Engine::TOOLS` gets `delete` last; `tools()` lists it only with `allow_write`; `call_with` answers
  `write_disabled` without it
- `description`: `DELETE_DESCRIPTION`, `delete_schema()` (the find schema parts reused); `mcp`: the tool with
  `readOnlyHint: false`, `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false`

### Selecting files — `src/write.rs` (new, shared by the write tools)

- `WriteRequest`: `Selection::Result(id)` or `Selection::Query(find arguments)`, `expect`, `dry_run`, `permanent`,
  the output values (`format`, `group_dirs`) as configuration values; `parse(arguments, fields)` checks the fields,
  the exclusivity of `result` and `query`, the query-only fields, and passes the find part to `request::parse` (so
  `version`, `dirs`, `traversal`, `kinds`, `languages`, `timeout` have the messages of `find`)
- `Entry`: the full path, the shown path, the member, whether it is a directory, the kept `Stamp`
- `cache::CachedFile` keeps `is_dir` and a `Stamp` (size and modification time of `symlink_metadata`), taken when
  the result is kept; `Stamp::of(path)` compares them when acting
- a result: the files of `Engine::kept`; a query: `Engine::run` with `detail = files`, `sort = path asc`,
  `funnel = never`, so the selection is searched like `find` (threads, timeout, progress, cancellation)
- `select`: sort by path, drop the entries inside a selected directory; `expect` counts before that
- `act`: per entry, stopping at the deadline or the cancellation: archive member → `archive_member`; outside of the
  roots (the canonical directory and the name) → `permission_denied`; a result entry with another stamp or none →
  `file_changed`; dry run → listed; else the operation of the tool, failed entries listed with
  `io::ErrorKind` text
- `Outcome` and its text and JSON formats: the header, the entries grouped by directory with
  `output::split` / `display_path`, the warnings of `Warnings`

### The trash — `src/recycle.rs` (new)

- `Trash` (public, a field of `EngineOptions`): `System` (default), `Dir(PathBuf)` and `None`, the test hooks;
  `main.rs` reads `EASY_PEASY_TEST_TRASH`
- `System` uses the `trash` crate (`trash::delete`): the freedesktop trash, the macOS Trash and the Recycle Bin
  behind one call; it is the well-known crate for this, writing the freedesktop `info` files and calling the system
  APIs on macOS and Windows by hand would be much more code. A `FileSystem` / `Os` error of the entry itself is a
  failure, the other errors (no trash, no home) are `trash_unavailable`
- `Dir` moves the entry to `<dir>/<path in its allowed root>` (`rename`), `None` is always unavailable

### Warnings and errors

- `WarningCode::FileChanged`, `ArchiveMember`, `TrashUnavailable`; `ErrorCode::WriteDisabled`, `CountMismatch`

## Rejected

- a separate trash crate per platform: more code, the `trash` crate covers all three
- stamping files lazily: the result must keep the state of the find, so it is stat'ed when it is kept
- library examples with a config block: the CLI argument of startup examples already states the setting

## Tests

- examples E125.1–E125.22 (`tests/examples.rs`, `tests/startup.rs`), with the new `text setup` block inside an
  example, the `remove` action, the trash hook and the `text trash` block in `tests/common`
- `tests/mcp_stdio.rs`: the tools of TOOLS.md with `--allow-write` too
- unit tests: `write::parse` errors, the nested entries dropped, the text and JSON formats (failed entries, dry run);
  `recycle::Trash::Dir` paths and collisions; config `allow_write` from the file and the command line

## Steps

1. tests (red), then config, engine, write, recycle, description, main
2. fix the earlier examples' unit tests of `get_config` (the read-only line)
3. definition of done, summary, version `0.127.0`

## Risks

- a `find` keeps a stat per matched file for its result: one more system call per file of the result
- the real trash is never tested automatically; the mapping of its errors is a best guess
