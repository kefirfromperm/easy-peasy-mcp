# Summary of task 33: Unreadable entries

Server version: `0.33.0`.

## What was built

- `permission_denied` (OUTPUT.md#warnings, CONFIG.md#security): entries the server can't read are skipped and
  reported, first in the order of the warnings; a directory path ends with `/`.
- When (spec clarified first, OUTPUT.md#warnings): a directory which can't be listed is skipped with everything below
  it, also by `type = dir`; a file which can't be opened is skipped when the server opens it — for its content when
  the other conditions don't decide, and always before it is returned; a file which doesn't match by its other
  conditions is not opened and gets no warning.
- The server never changes permissions; it has no call which could.
- The `find` description mentions the warning.

## Architecture

- `warning.rs`: `WarningCode::PermissionDenied`.
- `search/mod.rs`: the walk callback passes the walker's errors to `Walk::walk_error`, which turns a
  `PermissionDenied` I/O error with a path (`error_path` unwraps `WithDepth`, `WithPath`, `WithLineNumber`,
  `Partial`) into a warning; `Walk::readable` opens a matched file / lists a matched directory before it is returned;
  reading the content reports a `PermissionDenied` error. Other I/O errors (an entry deleted during the walk) skip the
  entry silently, as before.

## Test harness

- AGENTS.md and `tests/common`: the setup action `chmod <path> <mode>` (octal, Unix), run after the times.
  `setup_fixture` returns a `FixtureDir` which restores the permissions when dropped, so the temp dir can be removed,
  and tells whether the permissions could be removed; `tests/examples.rs` skips the examples of a fixture where they
  can't (Windows, root) and prints them.

## Deviations from the plan

None.

## Tests

- examples E33.1–E33.4 (fixture `perms`): content needed, matched by name, `type = dir`, a file which doesn't match
- unit: `search::tests::walker_error_paths`, `warning::tests::permission_denied_comes_first`

## Limitations and known gaps

- The examples can't run as root or on Windows; they are skipped there.
- Symlinks pointing outside of the allowed roots come with `follow_symlinks` and `allowed_roots` (later tasks).

## Try it

`{ "query": "text has 'TODO'" }` in a tree with a directory without read permission.
