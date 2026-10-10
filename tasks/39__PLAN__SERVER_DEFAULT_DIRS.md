# Plan for task 39: `server.default_dirs` and several working directories

## Design

- `src/config.rs`:
  - `server.default_dirs` in the config file (a non-empty list of strings) and in `Settings` with its source;
    `Source::Update`.
  - `update_config` gets the base settings (defaults ← file ← command line) and the session settings: the `reset`
    keys go back to the base, the `set` values are checked (`invalid_value`, hint `a non-empty list of paths is
    expected`) and applied on a copy, which replaces the session settings only when everything is valid (atomic).
    The result lists the keys whose value changed, `# was <old>`, `, reset` for a reset.
  - `get_config` shows `default_dirs` as written, TOML literal strings.
- `src/engine.rs`: the session settings behind a `RwLock`; every request works on a snapshot. `find` takes `dirs` of
  the request or `default_dirs`; each directory is resolved against the start directory, checked in order
  (`dir_not_found`, `dir_not_allowed`), searched; the paths get the directory as given (without trailing
  separators, `.` omitted) as a prefix; a file reached again (device and inode) is dropped; the files are sorted by
  their full path.
- `src/search/mod.rs`: `search` returns the file identities; identities for directories and links too.
- `src/warning.rs`: warnings of several directories are merged with the prefix.
- `src/request.rs`: the field `dirs` (`'dirs' must be a non-empty list of strings`).
- `src/error.rs`: `dir_not_found`, `invalid_value`.
- `src/description.rs`: the `dirs` field in the schema and the description.

## Tests

- examples E39.1–E39.15 (fixture `dirs`)
- unit: the dirs of a request, the update with reset and set, the result lines, the prefixes

## Steps

1. failing tests
2. config value and update, request field, several directories in the engine

## Risks

- the prefix must not change what `path` conditions see: they stay relative to the working directory
