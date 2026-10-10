# Examples for task 73

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### terraform

```text tree
main.tf                    2026-10-01 10:00
```

File contents are in `tests/fixtures/terraform/`, `main.tf`:

```text
# TODO in a hash comment
resource "a" "b" {} // TODO in a slash comment
locals { todo = "TODO in a string" }
```

Times are UTC.

## E73.1 A file of an unknown language is searched as it is

Fixture: `terraform`

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 text has 'TODO' /c

./
main.tf  [#1]  lines: 3  matches: 3
1:# ⟦#1:TODO⟧ in a hash comment
2:resource "a" "b" {} // ⟦#1:TODO⟧ in a slash comment
3:locals { todo = "⟦#1:TODO⟧ in a string" }
```

## E73.2 Line comments in a request

Fixture: `terraform`

```json request
{"query": "text has 'TODO' /c", "languages": {"terraform": {"extensions": ["tf"], "line_comments": ["#", "//"]}}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO' /c

./
main.tf  [#1]  lines: 1  matches: 1
3:locals { todo = "⟦#1:TODO⟧ in a string" }
```

## E73.3 One kind of line comment

Fixture: `terraform`

```json request
{"query": "text has 'TODO' /c", "languages": {"terraform": {"extensions": ["tf"], "line_comments": ["#"]}}}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'TODO' /c

./
main.tf  [#1]  lines: 2  matches: 2
2:resource "a" "b" {} // ⟦#1:TODO⟧ in a slash comment
3:locals { todo = "⟦#1:TODO⟧ in a string" }
```

## E73.4 Line comments for the session

Fixture: `terraform`

1. `update_config`

```json request update_config
{"set": "[languages.terraform]\nextensions = [\"tf\"]\nline_comments = [\"#\", \"//\"]"}
```

```text response
[languages.terraform]
extensions = ['tf']    # was (not set)
names = []    # was (not set)
shebangs = []    # was (not set)
line_comments = ['#', '//']    # was (not set)
block_comments = []    # was (not set)
nested_block_comments = false    # was (not set)
strings = []    # was (not set)
source = true    # was (not set)
```

2. `find`

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO' /c

./
main.tf  [#1]  lines: 1  matches: 1
3:locals { todo = "⟦#1:TODO⟧ in a string" }
```

## E73.5 An invalid value in a request

Fixture: `terraform`

```json request
{"languages": {"terraform": {"line_comments": "#"}}}
```

```text error
error[invalid_option]: 'languages.terraform.line_comments' must be a list of strings
```

## E73.6 An invalid value for `update_config`

Fixture: `terraform`

```json request update_config
{"set": "languages.terraform.line_comments = [[\"#\"]]"}
```

```text error
error[invalid_value]: invalid value for 'languages.terraform.line_comments'
  hint: a list of strings is expected
```

## E73.7 From the config file

Fixture: `terraform`

```toml conf/l.toml
[languages.terraform]
extensions = ["tf"]
line_comments = ["#", "//"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO' /c

./
main.tf  [#1]  lines: 1  matches: 1
3:locals { todo = "⟦#1:TODO⟧ in a string" }
```

## E73.8 An invalid value in the config file

Fixture: `terraform`

```toml conf/l.toml
[languages.terraform]
extensions = ["tf"]
line_comments = "#"
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:3: invalid value for 'languages.terraform.line_comments': a list of strings is expected
```
