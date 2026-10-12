# Task 133: Output to a file

Status: implemented, see [133__SUMMARY__OUTPUT_FILE.md](133__SUMMARY__OUTPUT_FILE.md)

Depends on: task 101, task 108, task 113

## Goal

An agent can send a large response to a file instead of its context: the tool result is a short summary with the
path, and the agent reads only the parts it needs (by line ranges, with its own file tools or another `find`). A big
result (all `TODO`s of a monorepo, a full file list for a script, a JSON result for another tool) no longer has to be
cut by `max_output` or paged through with cursors in the context.

## Scope

A spec change first (its own commit; OUTPUT.md, CONFIG.md, TOOLS.md, README.md), then the code.

### The option

- a new request option `output.file`: a file name, a relative path or an absolute path; the response is written there
  in the requested `output.format` (text or JSON), exactly as it would be returned, without the default limits of the
  context (see below)
- `output.file = true`: the server picks the name, `<result id>.txt` or `<result id>.json` (`r12.txt`); a response
  without a result id (`detail = exists`, `result_cache = 0`) gets `out-<n>.txt`
- request-only: it is not a config key, `update_config` rejects `output.file` with `invalid_value` and the hint
  `output.file is set per request`; in a batch (`queries`) each query's `output.file` is its own file, and one
  `output.file` on the request writes all the queries into one file, as the batch response would be

### Where files can be written

- a new read-only key `server.output_dir`, default `<system temp dir>/easy-peasy-mcp`, set only by the config file
  and the command line (`--output-dir`); `get_config` shows it in the `# read-only:` comment line
- a relative `output.file` is relative to `server.output_dir`; an absolute one must be inside it after
  canonicalization, `..` included; outside: `output_file_denied`, hint `files are written to <output_dir>`
- the server never writes elsewhere: the files of the allowed roots are still only read. `server.output_dir = ""`
  turns the option off: `output_file_disabled`
- the directory is created on the first write; an existing file is overwritten; the file is written to a temporary
  name next to it and renamed, so a reader never sees half a response
- the [Security](../CONFIG.md#security) section is updated: "the server only reads the allowed roots; it writes only
  responses, and only into `server.output_dir`"

### Limits in a file

- the defaults of `max_output`, `max_output_per_file` and `budget` don't apply to a file; values given in the request
  do. `max_files`, `max_lines`, `max_matches` and the other limits apply as usual, and a cut response still has its
  `next_cursor`
- a hard cap, read-only `server.max_output_file_size`, default `100M`: a response cut by it ends where it was cut and
  gets the `output_file_truncated` warning (in the file and in the summary), with a cursor to continue

### The tool result

- text: one header line like the `find` header with the counts and the result id, then
  `written: <path>  lines: N  bytes: N`, then the warnings and `next_cursor` if any; JSON (`output.format = "json"`):
  the same as fields (`file`, `lines`, `bytes`) instead of `files`
- the path is absolute, so the agent can read it with its own tools whatever its working directory is
- errors writing the file (`output_file_failed`: permission, disk full) fail the request with the OS error in the
  hint; nothing is returned in the context instead

### Tool description

- one line in TOOLS.md#description: `output.file = 'x.txt' writes the response to a file under the output dir and
  returns its path; for big results.`

## Out of scope

- appending to a file, other formats (CSV, JSONL)
- deleting old files of `server.output_dir`; it is in the temp dir, the system cleans it
- writing into the allowed roots (that is what the write tools of tasks 125–129 are for)
- reading the file back through the server (`find` with `dirs` inside `output_dir` works only if it is an allowed root)

## Spec

- [OUTPUT.md#options](../OUTPUT.md#options), [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation),
  [OUTPUT.md#batch](../OUTPUT.md#batch), [OUTPUT.md#warnings](../OUTPUT.md#warnings),
  [OUTPUT.md#errors](../OUTPUT.md#errors)
- [CONFIG.md#server](../CONFIG.md#server) (`output_dir`, `max_output_file_size`, `--output-dir`),
  [CONFIG.md#security](../CONFIG.md#security)
- [TOOLS.md#tools](../TOOLS.md#tools) (the input schema), [TOOLS.md#description](../TOOLS.md#description)

## Examples

`133__EXAMPLES__OUTPUT_FILE.md` covers:

- a text response written to a named file: the summary and the exact file content
- `output.file = true` with and without a result id; the JSON format
- a response which would be cut by the default `max_output` in the context, written whole to the file
- `max_output_file_size` reached: the warning and the cursor; the next page to another file
- a batch with one file per query and with one file for all
- a path outside `output_dir`, `..` escaping it, `output_dir = ""`, an unwritable directory
- `update_config` setting `output.file`
- the examples use a temporary `output_dir` set by the test, never the real temp dir of the machine

## Acceptance criteria

- the spec describes the option, the keys and the new codes before the code is written
- every example in `133__EXAMPLES__OUTPUT_FILE.md` passes as a test; the examples of all earlier tasks still pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
