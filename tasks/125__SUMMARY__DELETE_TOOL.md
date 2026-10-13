# Summary of task 125: `delete` tool

An agent which found files with `find` can remove them in one call: the files of a result (`"result": "r12"`) or of
a query, moved to the trash of the system, so a mistake can be undone. It is the first write tool and lays the shared
ground for `rename` and `copy` (tasks 126, 127).

## What was built

- **Enabling** ([TOOLS.md#enabling](../TOOLS.md#enabling), [CONFIG.md#server](../CONFIG.md#server)): the read-only
  key `server.allow_write` (default `false`) from the config file or `--allow-write`, shown by `get_config` (also in
  the `# read-only:` line), refused by `update_config` (`read_only`). Without it `delete` is not listed and a call
  fails with `write_disabled`. `find` stays `readOnlyHint: true`; `delete` is `readOnlyHint: false`,
  `destructiveHint: true`, `idempotentHint: true`, `openWorldHint: false`
- **Selection** ([TOOLS.md#selection](../TOOLS.md#selection)): exactly one of `result` and `query` (with `dirs`,
  `traversal`, `kinds`, `languages`, `version`, evaluated like a `find` request); `expect` fails with
  `count_mismatch` before anything changes; entries inside a selected directory go with it; result entries whose size
  or modification time changed, or which vanished, are skipped with `file_changed`; paths outside of the allowed roots
  (canonical directory, name not resolved) and an allowed root itself with `permission_denied`; archive members with `archive_member`; symlinks
  are acted on themselves; a query which times out changes nothing
- **Dry run** ([TOOLS.md#dry-run](../TOOLS.md#dry-run)): the same response with `dry run` in the header
- **Response** ([TOOLS.md#response-of-a-write-tool](../TOOLS.md#response-of-a-write-tool)): `deleted: N  skipped: N
  failed: N  permanent  dry run`, the entries grouped like `find` (`output.group_dirs`), failed entries with the kind of their
  error, the warnings; the JSON format with `output.format = "json"`; `isError` when nothing of a selection with entries was
  done (every entry skipped or failed, or the timeout before the first); the deadline and the cancellation stop between entries
- **delete** ([TOOLS.md#delete](../TOOLS.md#delete)): the trash of the system through the `trash` crate,
  `permanent: true` deletes (a directory recursively), no trash → `trash_unavailable`, never a silent permanent delete
- new codes: errors `write_disabled`, `count_mismatch`; warnings `file_changed`, `archive_member`,
  `trash_unavailable` ([OUTPUT.md](../OUTPUT.md#warnings))

## Architecture

- `src/write.rs` (new): `parse` (the fields of a write tool, the find part checked by `request::parse`),
  `Entry`, `check_expect`, `select`, `act` with `Act` (allowed roots, stop, operation, dry run), `Outcome` with its
  text and JSON formats
- `src/recycle.rs` (new): `Trash` (`System`, the test hooks `Dir` and `None`) and `TrashError`; the errors of the
  `trash` crate of the entry itself are failures, the others mean no trash
- `src/cache.rs`: `CachedFile` keeps `is_dir` and a `Stamp` (size and modification time, not following symlinks)
- `src/engine.rs`: `delete` in `TOOLS` and `tools()`, `try_delete` (the selection through `kept` or `run`, the
  operation `Trash::put` or `remove`), `EngineOptions.trash`
- `src/config`: `allow_write` with its source, `CommandLine.allow_write`; `src/main.rs`: `--allow-write` and
  `EASY_PEASY_TEST_TRASH`; `src/description.rs`, `src/mcp.rs`: the description, the schema and the annotations

A request flows: `mcp` → `Engine::call_with("delete")` → `write::parse` → the selection → `write::select` →
`write::act` → `Outcome::to_text` / `to_json`.

## Deviations from the plan

- after a comparison with two other implementations of the task (made in parallel with other search tools), two of
  their rules were taken: `isError` for a selection of which nothing was done, also when every entry was skipped (it
  was: only when something failed), and `permanent` in the header and the JSON; and one more safeguard: an allowed
  root itself is never changed (`write::may_change`)

- the spec example of the write response got the empty line after the header (amended in the spec commit)
- the usage line of the binary names `--allow-write`, so the usage in five earlier examples was updated
- E125.8 was fixed: a content search with detail `files` lists the counts of every file

## Tests

- examples E125.1–E125.22: E125.2 and E125.5 through the library, the rest are startup examples with
  `--allow-write`; the earlier `get_config` examples now show `allow_write`
- the example runners: a `text setup` block inside an example, the `remove` action, the trash hook (`trash` next to
  the fixture) and the `text trash` block (AGENTS.md)
- `tests/mcp_stdio.rs`: the tools match TOOLS.md with and without `--allow-write`
- unit tests: `write` (rejected selections, nested entries, formats with failed entries and the dry run),
  `recycle` (the hook paths, collisions, no trash), `config` (`allow_write` from the file and the command line)

## Limitations

- the real trash of the system is never exercised by the tests; the mapping of its errors is a best guess
- every kept result stats its files once more for their stamps
- restoring from the trash is out of scope; `rename` and `copy` are tasks 126 and 127

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --root ~/projects --allow-write
```

```json
{ "query": "name ~ '*.orig'", "dry_run": true }
```

then the same request without `dry_run`, or `{ "result": "r3", "expect": 12 }` after a `find`.

## Version

`0.127.0`
