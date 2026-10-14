# Examples for task 75

Spec: `1`

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### nested

```text tree
deep.pas                   2026-10-01 10:00
```

File contents are in `tests/fixtures/nested/`, `deep.pas`:

```text
(* outer (* inner *) TODO still outer *) TODO after
```

Times are UTC.

## E75.1 Block comments don't nest by default

The comment ends at the first `*)`.

Fixture: `nested`

```json request
{"query": "text has 'TODO' /c", "languages": {"pascal": {"extensions": ["pas"], "block_comments": [["(*", "*)"]]}}}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'TODO' /c

./
deep.pas  [#1]  lines: 1  matches: 2
1:(* outer (* inner *) ⟦#1:TODO⟧ still outer *) ⟦#1:TODO⟧ after
```

## E75.2 Nested block comments in a request

The comment ends after as many end markers as start markers.

Fixture: `nested`

```json request
{"query": "text has 'TODO' /c", "languages": {"pascal": {"extensions": ["pas"], "block_comments": [["(*", "*)"]], "nested_block_comments": true}}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO' /c

./
deep.pas  [#1]  lines: 1  matches: 1
1:(* outer (* inner *) TODO still outer *) ⟦#1:TODO⟧ after
```

## E75.3 Nested block comments for the session

Fixture: `nested`

1. `update_config`

```json request update_config
{"set": "[languages.pascal]\nextensions = [\"pas\"]\nblock_comments = [[\"(*\", \"*)\"]]\nnested_block_comments = true"}
```

```text response
[languages.pascal]
extensions = ['pas']    # was (not set)
names = []    # was (not set)
shebangs = []    # was (not set)
line_comments = []    # was (not set)
block_comments = [['(*', '*)']]    # was (not set)
nested_block_comments = true    # was (not set)
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
deep.pas  [#1]  lines: 1  matches: 1
1:(* outer (* inner *) TODO still outer *) ⟦#1:TODO⟧ after
```

## E75.4 An invalid value in a request

Fixture: `nested`

```json request
{"languages": {"pascal": {"nested_block_comments": "yes"}}}
```

```text error
error[invalid_option]: 'languages.pascal.nested_block_comments' must be true or false
```

## E75.5 An invalid value for `update_config`

Fixture: `nested`

```json request update_config
{"set": "languages.pascal.nested_block_comments = 1"}
```

```text error
error[invalid_value]: invalid value for 'languages.pascal.nested_block_comments'
  hint: true or false is expected
```

## E75.6 From the config file

Fixture: `nested`

```toml conf/l.toml
[languages.pascal]
extensions = ["pas"]
block_comments = [["(*", "*)"]]
nested_block_comments = true
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
deep.pas  [#1]  lines: 1  matches: 1
1:(* outer (* inner *) TODO still outer *) ⟦#1:TODO⟧ after
```

## E75.7 An invalid value in the config file

Fixture: `nested`

```toml conf/l.toml
[languages.pascal]
nested_block_comments = "yes"
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'languages.pascal.nested_block_comments': true or false is expected
```
