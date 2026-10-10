# Plan for task 33: Unreadable entries

## Design

- `warning.rs`: `WarningCode::PermissionDenied`, first in the order of the spec table.
- `search/mod.rs`:
  - the walk callback gets the errors of the walker too: an I/O error with the kind `PermissionDenied` and a path
    (`ignore::Error` nested in `WithPath` / `WithDepth`) is a `permission_denied` warning for that path, a directory
    when the path is one (the walker reports a directory it can't list)
  - a matched directory which can't be listed (`read_dir` fails) is not returned
  - reading the content of a file: a `PermissionDenied` error is a warning, the file is skipped
  - a matched file is opened before it is returned; a `PermissionDenied` error is a warning, the file is skipped
  - other I/O errors (a file deleted during the walk) skip the entry silently, as before
- the server never changes permissions: nothing to do, the code has no call which could.
- `tests/common`: the setup action `chmod <path> <mode>` (Unix), run after the times; `setup_fixture` returns a
  `FixtureDir` which restores the permissions when dropped, so the temp dir can be removed, and tells whether the
  permissions could be removed; `tests/examples.rs` skips the examples of such a fixture otherwise.
- the `find` description mentions the warning.

No new dependencies.

## Tests

- examples E33.1–E33.4 (fixture `perms`; skipped on Windows and as root)
- unit: the order of `permission_denied` among the warnings; the path of a nested walker error

## Steps

1. the harness action, failing examples and unit tests
2. warning code, walker errors, unreadable files and directories
3. description

## Risks

- the examples can't run where permissions can't be removed; CI on Linux as a normal user runs them
