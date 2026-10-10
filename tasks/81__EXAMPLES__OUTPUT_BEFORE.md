# Examples for task 81

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
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"
detail = "lines"
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
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
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
context = 0    # default
before = 2    # update
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
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
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[output]
format = "text"    # default
detail = "lines"    # default
context = 0    # default
before = 2    # file: conf/o.toml
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
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
