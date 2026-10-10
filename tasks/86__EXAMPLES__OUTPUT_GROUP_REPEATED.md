# Examples for task 86

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value.

## Fixtures

### repeated

```text tree
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
src/c.rs                   2026-10-01 10:00
```

File contents are in `tests/fixtures/repeated/`: every file starts with `use anyhow::Result;` (indented by two spaces
in `src/b.rs`); `src/a.rs` also has `fn a() -> Result<()> { todo!() }`. Times are UTC.

## E86.1 A line in 3 files is printed once by default

Its whitespace doesn't count. `src/b.rs` and `src/c.rs` have no other matched line, so they are not printed; the
counts don't change.

Fixture: `repeated`

1. `find`

```json request
{"query": "text has 'Result'"}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

repeated:
  ×3 use anyhow::⟦#1:Result⟧;    src/a.rs:1, src/b.rs:1, src/c.rs:1
src/
a.rs  [#1]  lines: 2  matches: 2
2:fn a() -> ⟦#1:Result⟧<()> { todo!() }
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
detail = "lines"
funnel = "auto"
facets = ['dir', 'lang', 'kind', 'condition']
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 100
max_lines = 500
max_matches = 1000
max_matches_per_file = 50
max_line_length = 300
max_output_per_file = 4000
max_output = 40000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E86.2 Off in a request

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"group_repeated": 0}}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

src/
a.rs  [#1]  lines: 2  matches: 2
1:use anyhow::⟦#1:Result⟧;
2:fn a() -> ⟦#1:Result⟧<()> { todo!() }
b.rs  [#1]  lines: 1  matches: 1
⇥2
1:use anyhow::⟦#1:Result⟧;
c.rs  [#1]  lines: 1  matches: 1
1:use anyhow::⟦#1:Result⟧;
```

## E86.3 A higher minimum in a request

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"group_repeated": 4}}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

src/
a.rs  [#1]  lines: 2  matches: 2
1:use anyhow::⟦#1:Result⟧;
2:fn a() -> ⟦#1:Result⟧<()> { todo!() }
b.rs  [#1]  lines: 1  matches: 1
⇥2
1:use anyhow::⟦#1:Result⟧;
c.rs  [#1]  lines: 1  matches: 1
1:use anyhow::⟦#1:Result⟧;
```

## E86.4 `detail = files` lists every file

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"detail": "files"}}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

src/
a.rs  [#1]  lines: 2  matches: 2
b.rs  [#1]  lines: 1  matches: 1
c.rs  [#1]  lines: 1  matches: 1
```

## E86.5 In JSON

Fixture: `repeated`

```json request
{"query": "text has 'Result'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "repeated": [
    { "count": 3, "text": "use anyhow::Result;",
      "fragments": [ { "col": [13, 19], "text": "Result", "ids": ["#1"] } ],
      "locations": ["src/a.rs:1", "src/b.rs:1", "src/c.rs:1"] }
  ],
  "counts": { "files": 3, "lines": 4, "matches": 4 },
  "legend": { "#1": "text has 'Result'" },
  "files": [
    {
      "path": "src/a.rs",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [ { "n": 2, "text": "fn a() -> Result<()> { todo!() }",
            "fragments": [ { "col": [11, 17], "text": "Result", "ids": ["#1"] } ] } ] }
      ]
    }
  ]
}
```

## E86.6 Off for the session

Fixture: `repeated`

1. `update_config`

```json request update_config
{"set": "output.group_repeated = 0"}
```

```text response
[output]
group_repeated = 0    # was 3
```

2. `find`

```json request
{"query": "text has 'Result'"}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

src/
a.rs  [#1]  lines: 2  matches: 2
1:use anyhow::⟦#1:Result⟧;
2:fn a() -> ⟦#1:Result⟧<()> { todo!() }
b.rs  [#1]  lines: 1  matches: 1
⇥2
1:use anyhow::⟦#1:Result⟧;
c.rs  [#1]  lines: 1  matches: 1
1:use anyhow::⟦#1:Result⟧;
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
meta = false    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 0    # update
sort = "relevance"    # default
path_style = "relative"    # default
```

## E86.7 An invalid value in a request

Fixture: `repeated`

```json request
{"output": {"group_repeated": -3}}
```

```text error
error[invalid_option]: 'output.group_repeated' must be a whole number
```

## E86.8 An invalid value for `update_config`

Fixture: `repeated`

```json request update_config
{"set": "output.group_repeated = true"}
```

```text error
error[invalid_value]: invalid value for 'output.group_repeated'
  hint: a whole number is expected (0: off)
```

## E86.9 From the config file

Fixture: `repeated`

```toml conf/o.toml
[output]
group_repeated = 0
```

```text command
easy-peasy-mcp --config conf/o.toml
```

1. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
meta = false    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 0    # file: conf/o.toml
sort = "relevance"    # default
path_style = "relative"    # default
```

2. `find`

```json request
{"query": "text has 'Result'"}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'Result'

src/
a.rs  [#1]  lines: 2  matches: 2
1:use anyhow::⟦#1:Result⟧;
2:fn a() -> ⟦#1:Result⟧<()> { todo!() }
b.rs  [#1]  lines: 1  matches: 1
⇥2
1:use anyhow::⟦#1:Result⟧;
c.rs  [#1]  lines: 1  matches: 1
1:use anyhow::⟦#1:Result⟧;
```

## E86.10 An invalid value in the config file

Fixture: `repeated`

```toml conf/o.toml
[output]
group_repeated = "3"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.group_repeated': a whole number is expected (0: off)
```
