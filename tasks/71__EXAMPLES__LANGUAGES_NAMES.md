# Examples for task 71

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### filenames

```text tree
Gemfile                    2026-10-01 10:00
Jenkinsfile                2026-10-01 10:00
build/Dockerfile           2026-10-01 10:00
```

File contents are in `tests/fixtures/filenames/`: one line each. Times are UTC.

## E71.1 The built-in names

`Gemfile` is Ruby by its name, `Jenkinsfile` has no language.

Fixture: `filenames`

```json request
{"query": "lang in (ruby, unknown)"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang in (ruby, unknown)

./
Gemfile  [#1]
Jenkinsfile  [#1]
```

## E71.2 A name in a request

Fixture: `filenames`

```json request
{"query": "lang = groovy", "languages": {"groovy": {"extensions": ["groovy", "gradle"], "names": ["Jenkinsfile"]}}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = groovy

./
Jenkinsfile  [#1]
```

## E71.3 Names in any case

Fixture: `filenames`

```json request
{"query": "lang = jenkins", "languages": {"jenkins": {"names": ["JENKINSFILE"]}}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = jenkins

./
Jenkinsfile  [#1]
```

## E71.4 A name of another language

Fixture: `filenames`

```json request
{"languages": {"groovy": {"names": ["Jenkinsfile", "Gemfile"]}}}
```

```text error
error[invalid_option]: the name 'Gemfile' of 'languages.groovy.names' is also in 'languages.ruby'
```

## E71.5 A name for the session

Fixture: `filenames`

1. `update_config`

```json request update_config
{"set": "[languages.groovy]\nextensions = [\"groovy\", \"gradle\"]\nnames = [\"Jenkinsfile\"]"}
```

```text response
[languages.groovy]
names = ['Jenkinsfile']    # was []
line_comments = []    # was ['//']
```

2. `find`

```json request
{"query": "lang = groovy"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = groovy

./
Jenkinsfile  [#1]
```

## E71.6 An invalid value in a request

Fixture: `filenames`

```json request
{"languages": {"groovy": {"names": "Jenkinsfile"}}}
```

```text error
error[invalid_option]: 'languages.groovy.names' must be a list of strings
```

## E71.7 An invalid value for `update_config`

Fixture: `filenames`

```json request update_config
{"set": "languages.groovy.names = [1]"}
```

```text error
error[invalid_value]: invalid value for 'languages.groovy.names'
  hint: a list of strings is expected
```

## E71.8 From the config file

Fixture: `filenames`

```toml conf/l.toml
[languages.groovy]
extensions = ["groovy", "gradle"]
names = ["Jenkinsfile"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "lang = groovy"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = groovy

./
Jenkinsfile  [#1]
```

## E71.9 A name of another language in the config file

Fixture: `filenames`

```toml conf/l.toml
[languages.groovy]
extensions = ["groovy", "gradle"]
names = ["Jenkinsfile", "Dockerfile"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:3: invalid value for 'languages.groovy.names': the name 'Dockerfile' is also in 'languages.dockerfile'
```
