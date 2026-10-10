# Examples for task 76

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### strings

```text tree
app.tf                     2026-10-01 10:00
```

File contents are in `tests/fixtures/strings/`, `app.tf`:

```text
url = "https://example.com/#TODO" # TODO in a comment
TODO = 1
```

Times are UTC.

## E76.1 Without strings a comment marker in a string starts a comment

Fixture: `strings`

```json request
{"query": "text has 'TODO' /c", "languages": {"terraform": {"extensions": ["tf"], "line_comments": ["#"]}}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO' /c

./
app.tf  [#1]  lines: 1  matches: 1
2:⟦#1:TODO⟧ = 1
```

## E76.2 Strings in a request

Fixture: `strings`

```json request
{"query": "text has 'TODO' /c", "languages": {"terraform": {"extensions": ["tf"], "line_comments": ["#"], "strings": ["\""]}}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO' /c

./
app.tf  [#1]  lines: 2  matches: 2
1:url = "https://example.com/#⟦#1:TODO⟧" # TODO in a comment
2:⟦#1:TODO⟧ = 1
```

## E76.3 Strings for the session

Fixture: `strings`

1. `update_config`

```json request update_config
{"set": "[languages.terraform]\nextensions = [\"tf\"]\nline_comments = [\"#\"]\nstrings = ['\"']"}
```

```text response
[languages.terraform]
extensions = ['tf']    # was (not set)
names = []    # was (not set)
shebangs = []    # was (not set)
line_comments = ['#']    # was (not set)
block_comments = []    # was (not set)
nested_block_comments = false    # was (not set)
strings = ['"']    # was (not set)
source = true    # was (not set)
```

2. `find`

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO' /c

./
app.tf  [#1]  lines: 2  matches: 2
1:url = "https://example.com/#⟦#1:TODO⟧" # TODO in a comment
2:⟦#1:TODO⟧ = 1
```

## E76.4 An invalid value in a request

Fixture: `strings`

```json request
{"languages": {"terraform": {"strings": "\""}}}
```

```text error
error[invalid_option]: 'languages.terraform.strings' must be a list of strings
```

## E76.5 An invalid value for `update_config`

Fixture: `strings`

```json request update_config
{"set": "languages.terraform.strings = false"}
```

```text error
error[invalid_value]: invalid value for 'languages.terraform.strings'
  hint: a list of strings is expected
```

## E76.6 From the config file

Fixture: `strings`

```toml conf/l.toml
[languages.terraform]
extensions = ["tf"]
line_comments = ["#"]
strings = ['"']
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO' /c

./
app.tf  [#1]  lines: 2  matches: 2
1:url = "https://example.com/#⟦#1:TODO⟧" # TODO in a comment
2:⟦#1:TODO⟧ = 1
```

## E76.7 An invalid value in the config file

Fixture: `strings`

```toml conf/l.toml
[languages.terraform]
strings = '"'
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'languages.terraform.strings': a list of strings is expected
```
