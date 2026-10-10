# Examples for task 82

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `context` of task 80:
`unwrap()` on the lines 3 and 9 of `src/config.rs`.

## E82.1 Equal to `context` by default

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
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
detail = "lines"
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 100
max_lines = 500
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E82.2 Lines after in a request

The file ends after line 10.

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"after": 2}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
5-Ok(cfg)
--
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E82.3 `after` replaces `context` for the lines after

Fixture: `context`

```json request
{"query": "text has 'unwrap()'", "output": {"context": 1, "after": 0}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'unwrap()'

src/
config.rs  [#1]  lines: 2  matches: 2
2-let raw = read(path)?;
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
--
8-let out = render();
9:write(out).⟦#1:unwrap()⟧;
```

## E82.4 Lines after for the session

Fixture: `context`

1. `update_config`

```json request update_config
{"set": "output.after = 2"}
```

```text response
[output]
after = 2    # was (not set)
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
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
5-Ok(cfg)
--
9:write(out).⟦#1:unwrap()⟧;
10-}
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
meta = false    # default
context = 0    # default
# before =  (not set)    # default
after = 2    # update
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E82.5 An invalid value in a request

Fixture: `context`

```json request
{"output": {"after": true}}
```

```text error
error[invalid_option]: 'output.after' must be a whole number
```

## E82.6 An invalid value for `update_config`

Fixture: `context`

```json request update_config
{"set": "output.after = 0.5"}
```

```text error
error[invalid_value]: invalid value for 'output.after'
  hint: a whole number is expected
```

## E82.7 From the config file

Fixture: `context`

```toml conf/o.toml
[output]
after = 2
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
meta = false    # default
context = 0    # default
# before =  (not set)    # default
after = 2    # file: conf/o.toml
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
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
3:let cfg = parse(&raw).⟦#1:unwrap()⟧;
4-let port = cfg.port;
5-Ok(cfg)
--
9:write(out).⟦#1:unwrap()⟧;
10-}
```

## E82.8 An invalid value in the config file

Fixture: `context`

```toml conf/o.toml
[output]
after = -1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.after': a whole number is expected
```
