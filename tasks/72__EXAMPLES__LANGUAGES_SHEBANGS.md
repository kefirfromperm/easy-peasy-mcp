# Examples for task 72

Spec: `1`

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### shebangs

```text tree
bin/next                   2026-10-01 10:00
bin/plain                  2026-10-01 10:00
bin/serve                  2026-10-01 10:00
bin/tool                   2026-10-01 10:00
```

File contents are in `tests/fixtures/shebangs/`: the first line of `bin/serve` is `#!/usr/bin/env deno run`, of
`bin/next` `#!/usr/bin/env deno2.1`, of `bin/tool` `#!/usr/bin/python3.11`; `bin/plain` has no shebang. Times are
UTC.

## E72.1 The built-in shebangs

`python3.11` is `python` without its version.

Fixture: `shebangs`

1. `find`

```json request
{"query": "lang = python"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = python

bin/
tool  [#1]
```

2. `find`

```json request
{"query": "lang = unknown"}
```

```text response
files: 3  lines: 0  matches: 0  result: r2
#1 lang = unknown

bin/
next  [#1]
plain  [#1]
serve  [#1]
```

## E72.2 A shebang in a request

The interpreter of `env` is its first argument which is not an option, with or without a version.

Fixture: `shebangs`

```json request
{"query": "lang = deno", "languages": {"deno": {"shebangs": ["deno"]}}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = deno

bin/
next  [#1]
serve  [#1]
```

## E72.3 A shebang for the session

A new language shows all its keys.

Fixture: `shebangs`

1. `update_config`

```json request update_config
{"set": "languages.deno.shebangs = [\"deno\"]"}
```

```text response
[languages.deno]
extensions = []    # was (not set)
names = []    # was (not set)
shebangs = ['deno']    # was (not set)
line_comments = []    # was (not set)
block_comments = []    # was (not set)
nested_block_comments = false    # was (not set)
strings = []    # was (not set)
source = true    # was (not set)
```

2. `find`

```json request
{"query": "lang = deno"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = deno

bin/
next  [#1]
serve  [#1]
```

## E72.4 An invalid value in a request

Fixture: `shebangs`

```json request
{"languages": {"deno": {"shebangs": "deno"}}}
```

```text error
error[invalid_option]: 'languages.deno.shebangs' must be a list of strings
```

## E72.5 An invalid value for `update_config`

Fixture: `shebangs`

```json request update_config
{"set": "languages.deno.shebangs = true"}
```

```text error
error[invalid_value]: invalid value for 'languages.deno.shebangs'
  hint: a list of strings is expected
```

## E72.6 From the config file

Fixture: `shebangs`

```toml conf/l.toml
[languages.deno]
shebangs = ["deno"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "lang = deno"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = deno

bin/
next  [#1]
serve  [#1]
```

## E72.7 An invalid value in the config file

Fixture: `shebangs`

```toml conf/l.toml
[languages.deno]
shebangs = "deno"
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'languages.deno.shebangs': a list of strings is expected
```
