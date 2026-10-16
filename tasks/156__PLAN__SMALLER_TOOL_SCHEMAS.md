# Plan for task 156: Smaller tool descriptions and schemas

## Design

Only `src/description.rs` changes: the input schemas become the JSON blocks of
[TOOLS.md](../TOOLS.md#what-the-schemas-describe). The request parsing (`src/request.rs`, `src/write.rs`,
`src/replace.rs`, `src/rename.rs`) already checks every key of the configuration objects and its value with
`invalid_option`, so no request handling changes.

- `FIND_SCHEMA`: `traversal`, `escape`, `query_options`, `kinds`, `git`, `languages` become `{ "type": "object" }`
  with a description; `output` keeps `detail`, `count_by`, `budget`, `context`, `after`, `groups`, `only_match`,
  `highlight`, `omit`, `show`, `cursor`, `file` and lists the other keys in its description; `omit` and `show` take
  `["array", "string"]` with the parts listed once
- `DELETE_SCHEMA`, `RENAME_SCHEMA`, `REPLACE_SCHEMA` (and `copy_schema`, derived from rename): `traversal`, `kinds`,
  `languages` become `{ "type": "object", "description": "Like in find." }`
- the descriptions stay

Rejected:

- removing the configuration objects from the write tool schemas: a client which checks the arguments against
  `additionalProperties: false` would reject a request the server accepts
- one shared `$defs` for the write tools: a `$ref` across tools doesn't exist, every tool is loaded alone
- shortening the tool descriptions: they are the query cheat sheet and the rules of a destructive call; the schemas
  are two thirds of the size and repeat themselves

## Tests

- E156.1 (startup): `tools/list` has the named-only objects
- E156.2–E156.6: requests with the named-only keys work or fail with the error of the key (they pass already: the
  behaviour doesn't change)
- E117.2: its `json receive` block shows the old schema of `find`, updated with the code
- `tests/mcp_stdio.rs` compares `tools/list` with the blocks of TOOLS.md (already there)
- a unit test in `src/description.rs`: the compact JSON of each schema stays under a size (`find` 3600, a write tool
  1900), so a schema doesn't grow back unnoticed

## Steps

1. red: the size test, run E156.1 and the TOOLS.md comparison
2. green: the new schemas in `src/description.rs`, E117.2 updated
3. measure `tools/list` of the release binary, sizes and the estimated tokens in the summary

## Risks

- a client may show the agent fewer hints for the named-only keys; the descriptions name them and `get_config` lists
  them with their values
- tokens can't be measured as an agent loads the tools from here: the summary gives characters and the estimate of
  ~2 characters per token of the task
