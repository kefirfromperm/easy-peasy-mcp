# Examples for task 62

Examples with a command line are startup examples (AGENTS.md). The examples use the fixture `fuzzy` of task 32; the
scores of `cfgldr`: `config-loader.rs` and `config_loader.rs` 0.70, `ConfigLoader.java` 0.64,
`scaffold_bigolder.ts` 0.40.

## E62.1 Scores from 0.5 by default

Fixture: `fuzzy`

1. `find`

```json request
{"query": "name ~~ 'cfgldr' and name ~ '*.{rs,ts}'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'
#2 name ~ '*.{rs,ts}'

src/
config-loader.rs  [#1 #2]
config_loader.rs  [#1 #2]
```

2. `get_config`

```json request get_config
{ "section": "query" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[query]
fuzzy_min_score = 0.5
```

## E62.2 A lower score in a request

Fixture: `fuzzy`

```json request
{"query": "name ~~ 'cfgldr' and name ~ '*.{rs,ts}'", "query_options": {"fuzzy_min_score": 0.3}}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'
#2 name ~ '*.{rs,ts}'

src/
config-loader.rs  [#1 #2]
config_loader.rs  [#1 #2]
scaffold_bigolder.ts  [#1 #2]
```

## E62.3 A higher score in a request

Fixture: `fuzzy`

```json request
{"query": "name ~~ 'cfgldr'", "query_options": {"fuzzy_min_score": 0.65}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'

src/
config-loader.rs  [#1]
config_loader.rs  [#1]
```

## E62.4 A score for the session

Fixture: `fuzzy`

1. `update_config`

```json request update_config
{"set": "query.fuzzy_min_score = 0.65"}
```

```text response
[query]
fuzzy_min_score = 0.65    # was 0.5
```

2. `find`

```json request
{"query": "name ~~ 'cfgldr'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'

src/
config-loader.rs  [#1]
config_loader.rs  [#1]
```

3. `get_config`

```json request get_config
{ "section": "query", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[query]
fuzzy_min_score = 0.65    # update
```

## E62.5 An invalid value in a request

Fixture: `fuzzy`

```json request
{"query_options": {"fuzzy_min_score": 1.5}}
```

```text error
error[invalid_option]: 'query_options.fuzzy_min_score' must be a number from 0 to 1
```

## E62.6 An invalid value for `update_config`

Fixture: `fuzzy`

```json request update_config
{"set": "query.fuzzy_min_score = \"high\""}
```

```text error
error[invalid_value]: invalid value for 'query.fuzzy_min_score'
  hint: a number from 0 to 1 is expected
```

## E62.7 From the config file

A whole number is a number too.

Fixture: `fuzzy`

```toml conf/q.toml
[query]
fuzzy_min_score = 1
```

```text command
easy-peasy-mcp --config conf/q.toml
```

1. `get_config`

```json request get_config
{ "section": "query", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[query]
fuzzy_min_score = 1    # file: conf/q.toml
```

2. `find`

Even the whole name scores below 1 (0.84 for `handler.go`), so `~~` matches nothing.

```json request
{"query": "name ~~ 'handler.go' or name = 'handlers.rs'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~~ 'handler.go'
#2 name = 'handlers.rs'

src/server/
handlers.rs  [#2]
```

## E62.8 An invalid value in the config file

Fixture: `fuzzy`

```toml conf/q.toml
[query]
fuzzy_min_score = -0.1
```

```text command
easy-peasy-mcp --config conf/q.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/q.toml:2: invalid value for 'query.fuzzy_min_score': a number from 0 to 1 is expected
```
