# Plan for task 133: output to a file

## Design

The spec change is committed already ([OUTPUT.md#output-to-a-file](../OUTPUT.md#output-to-a-file),
[CONFIG.md#server](../CONFIG.md#server)).

### Configuration

- `Settings` gets two read-only values with their sources, like `allowed_roots`: `output_dir: Option<PathBuf>`
  (`None` for `""`) and `max_output_file_size: String` (as written, parsed with `parse_size` where it is used)
- `output_dir` is resolved by `config::output_dir(text, start_dir, home)`: `~` expanded, joined to the start
  directory, the longest existing part canonicalized (`paths::canonical_prefix`), so the directory needn't exist;
  the default is `std::env::temp_dir()/easy-peasy-mcp`
- the config file (`FileValues`), the command line (`CommandLine::output_dir`, `--output-dir`) and `get_config` know
  both keys; `READ_ONLY` lists them; `server` of `SECTIONS` has them after `result_cache`
- `output.file` is not a key: the config file says `invalid value for 'output.file': output.file is set per request`,
  `update_config` (`set`, `reset`) `invalid_value` with the hint `output.file is set per request`

### Request

- `request::parse` takes `output.file` out of the `output` object like `output.cursor`: `Request::file:
  Option<FileTarget>`, `FileTarget::Path(String)` or `FileTarget::Auto`; `false` is `None`; another value is
  `'output.file' must be a path or true`
- `Request::given(key)`: the value of a key given in the request, for the limits for a file
- `cursor::request_hash` leaves out `output.file` as it does `output.cursor`

### Writing — a new module `src/output_file.rs`

- `resolve(dir, file) -> Result<PathBuf, Report>`: the path joined to `dir` (or absolute), `.` and `..` resolved
  lexically, the existing part canonicalized; inside `dir` and not `dir` itself, otherwise `output_file_denied`
- `write(path, text) -> Result<Written, Report>`: creates the directories, writes a temporary file
  `.<name>.<pid>-<n>.tmp` next to the file and renames it; `output_file_failed` with the error of the OS as the hint;
  `Written { path, lines, bytes }`
- `Written::line()`: `written: <path>  lines: N  bytes: N`

### Response

- `Response::cap: bool`: `limits.max_output` is the cap of `server.max_output_file_size`; `truncated_by` then names
  `max_output_file_size` for `max_output`, and `all_warnings` adds `output_file_truncated` when the listing was cut
  by it
- `WarningCode::OutputFileTruncated` after `GitBaseNotFound`, a warning without entries
- `budget::fit` returns the final response with the text, so the summary is made from the response at the level
  the budget chose
- `Response::warnings_text()`: the warnings block of the end of the text response (empty line and the lines);
  `Response::summary_json()`: the JSON document with only the fields of the header and the warnings

### Engine

- `try_find(arguments, progress, in_file)` returns an `Answer { text, head }`: the text of the response (or the
  summary when the request has its own file) and the `Head` of its summary (the first line and the warnings in text,
  the summary fields in JSON)
- the limits of a response in a file (`in_file` or a request with `output.file`): `max_output`,
  `max_output_per_file` and `budget` only from the request (`Request::given`), `max_output` at most the cap
- the target is checked before the search (`output_file_disabled`, `output_file_denied`); `true` is named after the
  search: `rN.txt` / `rN.json`, or `out-<n>` from a counter of the session (`Engine::unnamed`)
- `batch`: `output.file` of the request is taken out of the shared fields before the items are merged; the items of a
  shared file have `in_file`; an item with its own file gets no part of `budget` and `max_output` and doesn't inherit
  the shared ones; the totals of the split are the values of the request with a shared file and the configured ones
  otherwise; the batch response goes to the shared file unless all the queries fail; the summary is the first line of
  every section, in JSON the heads of the items with their labels

### Tool description and schema

- `description.rs`: the line of `output.file` after `queries = ...`, `file` in the `output` schema

## Rejected

- the cap as a separate limit counted in bytes of the file: the listing counts characters of paths and lines; a
  second counter of the whole text would need a second rendering loop. The spec counts it like `max_output`
- deriving the summary by parsing the response text (the first line, the `warning:` lines at the end): a file named
  `warning: x` would be taken for a warning; the response knows its warnings
- `readOnlyHint: false` for `find`: it writes only its own responses, never the searched files (TOOLS.md#tools)

## Tests

- E133.1–E133.15 (`tests/examples.rs`, startup ones in `tests/startup.rs`); the harnesses get `<output_dir>`, the
  replacement of the placeholders in the requests and the `text file` / `json file` blocks
- unit tests: `output_file::resolve` (relative, absolute, `..`, the directory itself, a symlink out of it),
  `write` (directories, replace, the temporary file is gone, `lines` and `bytes`), `request` (`output.file` values),
  `cursor::request_hash` without `output.file`, `config` (the keys of the file and the command line, `get_config`,
  `update_config`), `main` (`--output-dir`)
- earlier examples which change: the `# read-only:` line and the `[server]` section of `get_config`, the usage line
  of the command line, the `find` description (E117.2, E131.x) — fixed in their own commit
- the examples of every earlier task still pass

## Steps

1. red: the harness changes and the unit tests
2. configuration: the keys, the command line, `get_config`, `update_config`
3. request, cursor hash, `output_file`, the warning and the error codes
4. engine: single requests, then batches
5. description and schema
6. fix the earlier examples
7. summary, status `implemented`, minor version bump `0.124.0` → `0.125.0`

## Risks

- the default `output_dir` depends on the machine: the tests always set it (library) or point the temp directory of
  the system to their own (startup)
- the error text of the OS differs between platforms: E133.7 runs on Linux only
- a response without limits can be big: the cap bounds it, 100M by default
