# Examples for task 156

Smaller input schemas ([TOOLS.md](../TOOLS.md#what-the-schemas-describe)): the objects of configuration keys are named
in the schemas, not described key by key; `find` describes the common `output` keys and lists the others in the
description of `output`. The examples are written for the current spec version 2.

`tools/list` with `--allow-write`, in characters (the description, the input schema as compact JSON):

| Tool            | Description | Schema before | Schema after | Saved  |
|-----------------|-------------|---------------|--------------|--------|
| `find`          | 3534        | 6377          | 3488         | 2889   |
| `get_config`    | 212         | 233           | 233          | 0      |
| `update_config` | 461         | 289           | 289          | 0      |
| `delete`        | 864         | 2906          | 1283         | 1623   |
| `rename`        | 1544        | 3237          | 1614         | 1623   |
| `copy`          | 1649        | 3244          | 1621         | 1623   |
| `replace`       | 1554        | 3442          | 1819         | 1623   |
| total           | 9818        | 19728         | 10347        | 9381   |

The descriptions are unchanged. A request still takes every key the schemas only name: the server checks the keys and
their values (E156.2 to E156.6).

## Fixtures

### smaller-schemas

```text tree
.cache/settings.toml   2026-10-01 10:00
spec/colour_spec.rs    2026-10-01 10:00
src/main.rs            2026-10-01 10:00
src/settings.toml      2026-10-01 10:00
```

File contents are in `tests/fixtures/smaller-schemas/`: both `settings.toml` are `colour = "red"`,
`spec/colour_spec.rs` is `test colour`, `src/main.rs` is `fn main() {}`. `.cache/` is hidden, so it is searched only
with `traversal.hidden = true`.

## E156.1 The schemas name the configuration keys

The objects of configuration keys are `{ "type": "object" }` with a description; `output` of `find` describes its
common keys and lists the others.

Fixture: `smaller-schemas`

```text command
easy-peasy-mcp --allow-write
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "find",
        "inputSchema": {
          "properties": {
            "traversal": { "type": "object", "description": "Keys of [traversal] of get_config, e.g. {\"hidden\": true, \"exclude\": [\"*.lock\"]}." },
            "output": {
              "type": "object",
              "description": "Keys of [output] of get_config. Also: format before sort path_style max_files max_lines max_matches max_matches_per_file max_line_length max_output max_output_per_file facets facet_depth facet_limit funnel meta markers dedent blank_context group_dirs group_repeated repeated_locations path_fragments chars_per_token.",
              "properties": {
                "detail": { "enum": ["exists", "count", "facets", "files", "lines"] },
                "omit": {
                  "type": ["array", "string"], "items": { "type": "string" },
                  "description": "Parts to leave out, or \"all\": counts result dir legend funnel hints conditions file_counts match_ids groups line_numbers line_counts gaps indent more_lines locations warnings left_out."
                },
                "show": { "type": ["array", "string"], "items": { "type": "string" }, "description": "Parts to show whatever omit says, or \"all\"." }
              }
            },
            "escape": { "type": "object", "description": "Keys of [escape] of get_config: max_level." },
            "query_options": { "type": "object", "description": "Keys of [query] of get_config: fuzzy_min_score." },
            "kinds": { "type": "object", "description": "Keys of [kinds] of get_config, e.g. {\"test\": [\"spec/**\"]}." },
            "git": { "type": "object", "description": "Keys of [git] of get_config: base." },
            "languages": { "type": "object", "description": "Languages like [languages] of get_config; replace the configured ones with the same name." }
          }
        }
      },
      {"name": "get_config"},
      {"name": "update_config"},
      {
        "name": "delete",
        "inputSchema": {
          "properties": {
            "traversal": { "type": "object", "description": "Like in find." },
            "kinds": { "type": "object", "description": "Like in find." },
            "languages": { "type": "object", "description": "Like in find." }
          }
        }
      },
      {
        "name": "rename",
        "inputSchema": {
          "properties": {
            "traversal": { "type": "object", "description": "Like in find." },
            "kinds": { "type": "object", "description": "Like in find." },
            "languages": { "type": "object", "description": "Like in find." }
          }
        }
      },
      {
        "name": "copy",
        "inputSchema": {
          "properties": {
            "traversal": { "type": "object", "description": "Like in find." },
            "kinds": { "type": "object", "description": "Like in find." },
            "languages": { "type": "object", "description": "Like in find." }
          }
        }
      },
      {
        "name": "replace",
        "inputSchema": {
          "properties": {
            "traversal": { "type": "object", "description": "Like in find." },
            "kinds": { "type": "object", "description": "Like in find." },
            "languages": { "type": "object", "description": "Like in find." }
          }
        }
      }
    ]
  }
}
```

## E156.2 A write tool takes a key of `traversal`

`traversal.hidden` is only named in the schema of `replace`; the hidden `.cache/settings.toml` is selected.

Fixture: `smaller-schemas`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'red' @c", "traversal": {"hidden": true}, "replace": {"@c": "blue"}, "dry_run": true}
```
```text response
dry run  would replace: 2  matches: 2

.cache/
settings.toml  matches: 1
-1:colour = "red"
+1:colour = "blue"
src/
settings.toml  matches: 1
-1:colour = "red"
+1:colour = "blue"
```

## E156.3 An unknown key of `traversal` in a write tool

The server names the keys the schema leaves out.

Fixture: `smaller-schemas`

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "name = 'settings.toml'", "traversal": {"colour": 1}, "to": "{dir}/old.toml"}
```
```text error
error[invalid_option]: unknown field 'traversal.colour'
  hint: expected one of: min_depth, max_depth, follow_symlinks, hidden, ignore_files, git_excludes, ignore_paths, exclude, path_case, binary, max_file_size, encoding, fallback_encoding, archives, max_archive_size, skip_generated
```

## E156.4 A wrong value of a key of `traversal` in a write tool

Fixture: `smaller-schemas`

```text command
easy-peasy-mcp --allow-write
```

```json request rename
{"query": "name = 'settings.toml'", "traversal": {"hidden": "yes"}, "to": "{dir}/old.toml"}
```
```text error
error[invalid_option]: 'traversal.hidden' must be true or false
```

## E156.5 `find` takes an `output` key the schema only lists

`group_dirs` and `sort` are listed in the description of `output`, not described.

Fixture: `smaller-schemas`

```json request
{"query": "text has 'colour'", "output": {"group_dirs": false, "sort": "path desc"}}
```
```text response
files: 2  lines: 2  matches: 2  result: r1

src/settings.toml  lines: 1  matches: 1
1:⟦colour⟧ = "red"
spec/colour_spec.rs  lines: 1  matches: 1
1:test ⟦colour⟧
```

## E156.6 A key of `kinds` in `find`

A valid value works, a wrong one is the error of that key.

Fixture: `smaller-schemas`

1. a list of wildcards

```json request
{"query": "kind = test", "kinds": {"test": ["spec/**"]}, "output": {"detail": "files"}}
```
```text response
files: 1  lines: 0  matches: 0  result: r1

spec/
colour_spec.rs
```

2. a single string

```json request
{"query": "kind = test", "kinds": {"test": "spec/**"}}
```
```text error
error[invalid_option]: 'kinds.test' must be a list of strings
```
