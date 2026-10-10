# Summary of task 39: `server.default_dirs` and several working directories

Server version: `0.39.0`.

## What was built

- `server.default_dirs` (CONFIG.md#server), default `["."]`: from the config file (a non-empty list of strings,
  otherwise `invalid value for 'server.default_dirs': a non-empty list of paths is expected` at start), shown by
  `get_config` with its source, changed and reset by `update_config`.
- `update_config` changes values for the first time (CONFIG.md#update_config, spec clarified first): `reset` goes
  back to the value of the file / command line / default with its source, `set` checks every value
  (`invalid_value`, hint `a non-empty list of paths is expected`) and applies all of them or nothing; the result
  lists the keys whose value changed, `<key> = <value>    # was <old>`, `, reset` for a reset; a changed value has
  the source `update`; nothing changed: `# nothing changed`.
- The request field `dirs` (TOOLS.md#input-schema) replaces `default_dirs` for one request; `'dirs' must be a
  non-empty list of strings` otherwise (`invalid_option`).
- Several working directories (OUTPUT.md#paths): resolved against the start directory and checked in order —
  `dir_not_found` (`the working directory '<dir>' doesn't exist` / `is not a directory`), `dir_not_allowed`; `..` is
  fine inside a root. The paths get the directory as given as a prefix (trailing separators dropped, `.` omitted),
  `path` conditions still see the path relative to the working directory; files are ordered by their full path; an
  entry reached again (device and inode) is returned once, under the first working directory.
- Config comments follow their line after four spaces (spec changed: aligned comments depend on the length of the
  paths, so results would differ between machines).
- The `find` description and schema have `dirs`.

## Architecture

- `src/config.rs`: `Mutable` (the keys `update_config` changes) with `parse`, `MutableValue`; `Settings` with
  `default_dirs` and its source, `value`, `reset`, `set`; `update_config(base, session, arguments)`, `reset_keys`,
  `changes`; `Source::Update`; `toml_list`, `toml_string`.
- `src/engine.rs`: the session settings behind a `RwLock` (requests work on a snapshot); `working_dir` checks a
  directory and gives the search path and the prefix; `find` merges the results of all directories.
- `src/search/mod.rs`: `Found.id` (device and inode, for every entry); `src/warning.rs`: `Warnings::extend` with a
  prefix; `src/output/mod.rs`: paths keep `..` and the root of absolute paths.
- `src/request.rs`: `Request.dirs`; `src/error.rs`: `dir_not_found`, `invalid_value`.

## Deviations from the plan

- The comment alignment of the spec was replaced (see above); the `get_config` examples of tasks 35, 36 and 38 show
  the new key (separate commits).

## Tests

- examples E39.1–E39.15 (fixture `dirs`)
- unit: `config::tests::update_config_changes_the_session` (set, atomic, invalid value, reset, set over reset),
  `request::tests::dirs`, the file values and the rendering

## Limitations and known gaps

- On Windows entries have no identity yet (volume and file index), so overlapping directories may return a file twice
  there — task 120.

## Try it

`{ "dirs": ["../api", "../web"], "query": "name ~ '*.rs'" }`; `update_config { "set": "server.default_dirs = ['src']" }`
