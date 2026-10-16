# Summary of task 156: Smaller tool descriptions and schemas

## What was built

The input schemas describe what an agent sets in most calls and only name the rest
([TOOLS.md](../TOOLS.md#what-the-schemas-describe)):

- `find`: `traversal`, `escape`, `query_options`, `kinds`, `git` and `languages` are `{ "type": "object" }` with a
  description ("Keys of [traversal] of get_config, e.g. ..."); `output` describes `detail`, `count_by`, `budget`,
  `context`, `after`, `groups`, `only_match`, `highlight`, `omit`, `show`, `cursor`, `file` and lists its other keys in
  its description; the parts of `omit` are listed once, `show` refers to them
- `delete`, `rename`, `copy`, `replace`: `traversal`, `kinds` and `languages` are
  `{ "type": "object", "description": "Like in find." }`
- `get_config`, `update_config` and every tool description are unchanged

No request field or key is removed and the server checks the keys and values as before (`invalid_option` with the
expected keys, or the error of the key), so every request valid before is valid now. The schemas are the same for both
spec versions; spec 1 and spec 2 are unchanged.

## Saving

`tools/list` with `--allow-write`, characters (description + input schema as compact JSON, the whole tool entry):

| Tool            | Before | After  | Saved | ≈ tokens saved |
|-----------------|--------|--------|-------|----------------|
| `find`          | 10076  | 7187   | 2889  | 1.4K           |
| `get_config`    | 557    | 557    | 0     | 0              |
| `update_config` | 920    | 920    | 0     | 0              |
| `delete`        | 3936   | 2313   | 1623  | 0.8K           |
| `rename`        | 4975   | 3352   | 1623  | 0.8K           |
| `copy`          | 5085   | 3462   | 1623  | 0.8K           |
| `replace`       | 5187   | 3564   | 1623  | 0.8K           |
| total           | 30736  | 21355  | 9381  | 4.7K           |

The schemas went from 19728 to 10347 characters (−48%), the whole list by 31%. Tokens are estimated with the ~2
characters per token of JSON schemas measured in run 3 (task 151): the six tools loaded there (all but
`update_config`) shrink from 29816 to 20435 characters, ~4.7K of the 15.2K tokens of that load. The load itself
(the context of an agent before and after ToolSearch) can only be measured in a client session, at the next run of
the experiment.

## Architecture

Only `src/description.rs` changed: `FIND_SCHEMA`, `DELETE_SCHEMA`, `RENAME_SCHEMA`, `REPLACE_SCHEMA` (and `copy_schema`,
derived from rename's) are the JSON blocks of TOOLS.md. The request parsing (`src/request.rs`, `src/write.rs`) already
validated every key with `invalid_option`, so nothing else changed.

## Deviations from the plan

None. The descriptions stayed: the write tools don't repeat the query syntax of `find` (one line names the selection
fields), and the cheat sheet of `find` is what an agent needs before its first query.

## Tests

- E156.1 (startup): `tools/list` names the configuration objects
- E156.2–E156.6: a write tool takes `traversal.hidden`; an unknown and a wrong key of `traversal`; `find` takes the
  listed-only `output.group_dirs` and `sort`; a valid and a wrong `kinds.test`
- E117.2 shows the new schema of `find`
- `description::tests`: the schemas stay under 3600 (`find`) and 1900 (a write tool) characters, the configuration
  objects are named only
- `tests/mcp_stdio.rs::tools_are_the_spec_text` compares `tools/list` with TOOLS.md

## Limitations

- a client shows no per-key types for the named-only objects; the descriptions name them, `get_config` lists them
  with their values, the errors list the expected keys
- the descriptions (9818 characters) are now about half of the list; shortening them is left for a later task

## Try it

```sh
easy-peasy-mcp --allow-write
```

`tools/list` shows the new schemas; `replace` with `"traversal": {"hidden": true}` selects hidden files as before.

## Version

0.150.0
