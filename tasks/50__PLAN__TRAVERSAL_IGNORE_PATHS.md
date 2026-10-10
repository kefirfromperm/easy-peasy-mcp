# Plan for task 50: `traversal.ignore_paths`

## Design

About 45 keys of the spec are still to come, each with the same wiring: a default, a value from the config file, a
source, `get_config`, `update_config` (set, reset), the request field and its errors. Until task 49 every key had
its own field and source in `Settings`, an arm in five `match`es and its own request parser. This task first moves
the changeable keys into a table, then adds `ignore_paths` as one row of it.

- `src/config/keys.rs` (new): `Key`, the changeable keys in the order of the spec, each with its section, name,
  `Type` and default `Value`. `Type` checks a TOML item (the hint of `invalid value ...` on error) and a JSON value of
  a request (the `must be ...` text on error) and writes a value as TOML. `Value`: `Bool`, `Whole`, `NotSet`, `Text`,
  `List`.
- `src/config/mod.rs` (was `src/config.rs`): `Settings` keeps the read-only keys as fields and the changeable ones in
  a map `Key → (Value, Source)`, with typed getters (`bool`, `whole`, `opt_whole`, `text`, `list`) and `with` — the
  settings of one request, the request values replacing the session ones. The config file check, `get_config` and
  `update_config` go through `Key` instead of per-key code.
- `src/request.rs`: the objects `traversal` (and later `output`, `escape`, `query_options`, `kinds`, `git`) are
  parsed through the same table into `Request.config: Vec<(Key, Value)>`; a key of the spec which is not in the table
  is `not_implemented`, an unknown one `invalid_option` as before.
- `src/engine.rs`: `settings.with(&request.config)` once, the search options from it.
- `ignore_paths`: `Type::Paths`, default `[]`, the hint `a list of paths is expected`; `EngineOptions.home` (set by
  `main` from `HOME` / `USERPROFILE`) expands `~`, relative paths are resolved against the start directory.
- `src/search/mod.rs`: `SearchOptions.ignore_paths` (resolved paths): `WalkBuilder::current_dir(working dir)` and
  `add_ignore(path)` for every existing file — the walker gives these rules the lowest priority and matches them
  relative to the working directory; a later file wins; a missing file is skipped.

## Decisions

- A table with typed getters instead of a macro generating fields: the keys stay data, a new key is one row and one
  getter call; the type of a value is an invariant of its key (an `unreachable!` names it).
- Lists are written as literal strings (`['...']`) like before, scalar strings as basic strings (`"30s"`).

## Tests

- examples E50.1–E50.10 (fixture `ignore-paths`), and the `get_config` examples of tasks 35–49 which now show the key
- unit: the request field and its error, the file value, `Settings::with`, the TOML of every type

## Steps

1. failing tests
2. refactor the configuration into the key table, all earlier tests green
3. `ignore_paths`: key, request field, walker

## Risks

- the refactor touches every configuration path; the examples of tasks 34–49 cover them byte for byte
