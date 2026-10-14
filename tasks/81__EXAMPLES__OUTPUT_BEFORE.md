# Examples for task 81

Spec: `1`

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `context` of task 80:
`unwrap()` on the lines 3 and 9 of `src/config.rs`.

## E81.1 Equal to `context` by default

Fixture: `context`

1. `find`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 1}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
--
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
10-}
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"
detail = "lines"
count_by = "total"
# budget =  (not set)
chars_per_token = 4
funnel = "auto"
facets = ['dir', 'lang', 'kind', 'condition']
facet_depth = 2
facet_limit = 8
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
repeated_locations = 3
sort = "relevance"
path_style = "relative"
omit = []
show = []
```

## E81.2 Lines before in a request

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"before": 2}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
1-fn load() {
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
--
7-fn save() {
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
```

## E81.3 `before` replaces `context` for the lines before

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 1, "before": 0}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
--
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E81.4 Lines before for the session

Fixture: `context`

1. `update_config`

```json request update_config
{"set": "output.before = 2"}
```

```text response
[output]
before = 2    # was (not set)
```

2. `find`

```json request
{"query": "text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
1-fn load() {
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
--
7-fn save() {
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
count_by = "total"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
before = 2    # update
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
group_repeated = 3    # default
repeated_locations = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
show = []    # default
```

## E81.5 An invalid value in a request

Fixture: `context`

```json request
{"output": {"before": "2"}}
```

```text error
error[invalid_option]: 'output.before' must be a whole number
```

## E81.6 An invalid value for `update_config`

Fixture: `context`

```json request update_config
{"set": "output.before = -2"}
```

```text error
error[invalid_value]: invalid value for 'output.before'
  hint: a whole number is expected
```

## E81.7 From the config file

Fixture: `context`

```toml conf/o.toml
[output]
before = 2
```

```text command
easy-peasy-mcp --config conf/o.toml
```

1. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
count_by = "total"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['dir', 'lang', 'kind', 'condition']    # default
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
before = 2    # file: conf/o.toml
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
group_repeated = 3    # default
repeated_locations = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
omit = []    # default
show = []    # default
```

2. `find`

```json request
{"query": "text has 'unwrap()'"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
1-fn load() {
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
--
7-fn save() {
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
```

## E81.8 An invalid value in the config file

Fixture: `context`

```toml conf/o.toml
[output]
before = "all"
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.before': a whole number is expected
```
