# Examples for task 74

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### pascal

```text tree
notes.pas                  2026-10-01 10:00
```

File contents are in `tests/fixtures/pascal/`, `notes.pas`:

```text
TODO before (* TODO in a block *) TODO after
(* a block
TODO on its second line *)
```

Times are UTC.

## E74.1 A file of an unknown language is searched as it is

Fixture: `pascal`

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 2  matches: 4  result: r1
#1 text has 'TODO' /c

./
notes.pas  [#1]  lines: 2  matches: 4
1:⟦#1:TODO⟧ before (* ⟦#1:TODO⟧ in a block *) ⟦#1:TODO⟧ after
--
3:⟦#1:TODO⟧ on its second line *)
```

## E74.2 Block comments in a request

A block comment ends at its end marker, also on another line.

Fixture: `pascal`

```json request
{"query": "text has 'TODO' /c", "languages": {"pascal": {"extensions": ["pas"], "block_comments": [["(*", "*)"]]}}}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'TODO' /c

./
notes.pas  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ before (* TODO in a block *) ⟦#1:TODO⟧ after
```

## E74.3 Block comments for the session

Fixture: `pascal`

1. `update_config`

```json request update_config
{"set": "[languages.pascal]\nextensions = [\"pas\"]\nblock_comments = [[\"(*\", \"*)\"]]"}
```

```text response
[languages.pascal]
extensions = ['pas']    # was (not set)
names = []    # was (not set)
shebangs = []    # was (not set)
line_comments = []    # was (not set)
block_comments = [['(*', '*)']]    # was (not set)
```

2. `find`

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'TODO' /c

./
notes.pas  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ before (* TODO in a block *) ⟦#1:TODO⟧ after
```

## E74.4 An invalid value in a request

Fixture: `pascal`

```json request
{"languages": {"pascal": {"block_comments": ["(*", "*)"]}}}
```

```text error
error[invalid_option]: 'languages.pascal.block_comments' must be a list of [start, end] pairs of strings
```

## E74.5 An invalid value for `update_config`

Fixture: `pascal`

```json request update_config
{"set": "languages.pascal.block_comments = [[\"(*\"]]"}
```

```text error
error[invalid_value]: invalid value for 'languages.pascal.block_comments'
  hint: a list of [start, end] pairs of strings is expected
```

## E74.6 From the config file

Fixture: `pascal`

```toml conf/l.toml
[languages.pascal]
extensions = ["pas"]
block_comments = [["(*", "*)"]]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "text has 'TODO' /c"}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'TODO' /c

./
notes.pas  [#1]  lines: 1  matches: 2
1:⟦#1:TODO⟧ before (* TODO in a block *) ⟦#1:TODO⟧ after
```

## E74.7 An invalid value in the config file

Fixture: `pascal`

```toml conf/l.toml
[languages.pascal]
extensions = ["pas"]
block_comments = [["(*", "*)", "x"]]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:3: invalid value for 'languages.pascal.block_comments': a list of [start, end] pairs of strings is expected
```
