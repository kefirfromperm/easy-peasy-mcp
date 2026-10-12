# Summary of task 133: output to a file

Server version: `0.125.0`.

## What was built

[OUTPUT.md#output-to-a-file](../OUTPUT.md#output-to-a-file): `output.file` writes the response to a file under
`server.output_dir`, and the tool returns a short summary with the path. A big result doesn't go into the context of
the agent, and the configured limits that keep the context small don't cut it.

```json
{ "query": "text has 'TODO'", "output": { "file": "todos.txt" } }
```

```
files: 3  lines: 4  matches: 4  result: r1
written: /tmp/easy-peasy-mcp/todos.txt  lines: 16  bytes: 337
```

- `output.file` is a file name, a relative path or an absolute path inside the output directory. `true` names the
  file after the result id (`r12.txt`, `.json`), or `out-<n>` when the response has no result id (and for a batch).
  `false` is the same as not setting it
- the file holds the response exactly as it would be returned. It is written to a temporary file and renamed,
  replacing an existing file. The directories are created when needed
- the path must stay inside `server.output_dir` after `..` is resolved and the existing directories are
  canonicalized; otherwise the request fails with `output_file_denied`. With `server.output_dir = ""` it fails with
  `output_file_disabled`. A write error (`output_file_failed`) has the error of the OS as its hint. The path is
  checked before the search
- limits for a file: `max_output`, `max_output_per_file` and `budget` apply only with values given in the request,
  and the configured values don't apply. The read-only `server.max_output_file_size` (default `100M`) is a hard cap,
  counted like `max_output`. When the cap cuts the response, `truncated_by` names `max_output_file_size`, the
  response gets the `output_file_truncated` warning and the cursor continues on the next page
- the summary: in text, the first line of the response, then `written: <path>  lines: N  bytes: N`, then its
  warnings. In JSON, the header fields and the warnings, then `file`, `lines` and `bytes`
- batches: an item's `output.file` writes its query to its own file, and its section is the summary. The request's
  `output.file` writes the whole batch response to one file, and the summary is the first line of every section. A
  query in its own file gets no part of the shared `budget` and `max_output`. Nothing is written when all the queries
  fail
- `output.file` is not a configuration key, and it isn't part of the request a cursor belongs to
- configuration ([CONFIG.md#server](../CONFIG.md#server)): `server.output_dir` (default `easy-peasy-mcp` in the temp
  directory of the system, needn't exist, shown resolved by `get_config`) and `server.max_output_file_size`, both
  read-only; `--output-dir <dir>`; `update_config` rejects `output.file` with `invalid_value`
- [Security](../CONFIG.md#security): the server only reads the allowed roots and writes only responses, and only into
  `server.output_dir`. `find` keeps `readOnlyHint: true`
- the `find` description has the line of `output.file`, and the schema has its field. The `update_config`
  description names every read-only key

## Architecture

- `src/output_file.rs` (new): `FileTarget` (a path, or `Auto`); `resolve` checks the path in the output directory;
  `write` writes atomically and returns `Written` (path, lines, bytes); `Head` is the part of a response its summary
  keeps (the first line and the warnings, or the JSON summary fields); `Head::summary` and `with_file` build the
  summary
- `src/paths.rs`: `canonical_prefix`, the canonical form of a path which needn't exist
- `src/config/mod.rs`: `Settings::output_dir` and `max_output_file_size` with their sources, from the config file
  (`FileValues`) and `--output-dir` (`CommandLine`), in `get_config` and `READ_ONLY`; `output_dir()` resolves the
  directory
- `src/request.rs`: `Request::file` (from `file_target`), `Request::given` for the values given in the request
- `src/cursor.rs`: the request hash leaves out `output.file`
- `src/output/`: `Limits::file_cap` names the cap in `truncated_by` and adds `output_file_truncated` in `all_warnings`;
  `Response::warnings_text` and `summary_json`; `budget::fit` also returns the response at the level it chose
- `src/engine.rs`: `try_find(arguments, progress, in_file)` returns an `Answer` (the text and the `Head`). The target
  is checked before the search, `file_limits` sets the limits for a file, and the file is written after the
  response; `file_path` names `true`. `batch` takes the shared `output.file` out of the items, splits the totals of
  the request with a shared file, keeps the own-file items out of the split (`set_item_output`) and writes the batch
  response and its `batch::summary`
- `src/main.rs`: `--output-dir`

A request flows: `request::parse` (the file) → `target` (disabled / denied) → `run` → `respond` (the limits for a
file, the head) → `output_file::write` → `Head::summary`.

## Deviations from the plan

- the `update_config` description named only two read-only keys. A spec fix (TOOLS.md) and the code now name all
  four
- clippy's length limit split `batch` and `respond` into `set_item_output`, `file_limits`, `head` and `answers`

## Tests

- E133.1–E133.10 (`tests/examples.rs`, fixture `output-file`): a named file, `true` with JSON, `out-<n>`, the
  configured limits vs. the request's limits with the next page in another file, directories, absolute paths, a
  replaced file, denied paths and wrong values, a write error (Linux), `update_config`, a batch with a file per query
  and one file for all (text and JSON)
- E133.11–E133.15 (`tests/startup.rs`): `--output-dir` with `get_config` sources, `output_dir = ""`, invalid config
  values, and the `max_output_file_size` cap with its warning and the next page
- the harnesses: `<output_dir>` (library: `out` next to the fixture; startup: the system temp directory pointed at
  a directory of the test), placeholders in the requests, `text file` / `json file` blocks (AGENTS.md)
- unit tests: `output_file` (paths inside and outside, a symlink out, an atomic write and replace, a write error,
  lines), `paths::canonical_prefix`, `request` (`output.file`, `given`), `cursor` (the hash without the file),
  `config` (the keys of the file, the command line and the defaults, `update_config` errors)
- earlier examples fixed in their own commit: the `# read-only:` line and the `[server]` section of every
  `get_config` response, the usage line (E34.15, E37.3, E37.4, E38.x, E40.x), the descriptions (E117.2, E131.1),
  E121.8

## Limitations

- the cap counts characters of the listed paths and lines, like `max_output`, not the bytes of the file; the header,
  the legend and the warnings come on top
- `server.output_dir` is never cleaned by the server; it lives in the temp directory by default
- the result id of a request whose file can't be written is used, though the response is not returned
- out of scope as planned: appending, other formats, deleting old files, reading the files back through the server

## How to try it

```sh
easy-peasy-mcp --root ~/projects --output-dir ~/.cache/easy-peasy
```

```json
{ "query": "text has 'TODO'", "output": { "file": true } }
```

returns `written: /home/me/.cache/easy-peasy/r1.txt  lines: ...  bytes: ...`; read the file by line ranges.
