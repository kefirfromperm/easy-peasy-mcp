# Examples for task 8

The fixture `names` is described in [3__EXAMPLES__NAME_OPERATORS.md](3__EXAMPLES__NAME_OPERATORS.md), the fixture
`flat` in [2__EXAMPLES__EMPTY_QUERY.md](2__EXAMPLES__EMPTY_QUERY.md).

## E8.1 A query documented with comments

The legend writes the conditions without the comments.

Fixture: `names`

```json request
{ "query": "name has 'config'          # the concept\nand name ends '.rs'        # Rust only\nand not name starts 'app'  # not the app one\n" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'config'
#2 name ends '.rs'
#3 name starts 'app'

lib/
config.rs  [#1 #2]
config_loader.rs  [#1 #2]
```

## E8.2 `#` inside a string is text

Fixture: `names`

```json request
{ "query": "name = 'a#b' or name = 'config.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'a#b'
#2 name = 'config.rs'

lib/
config.rs  [#2]
```

## E8.3 A comment right after a value

Fixture: `names`

```json request
{ "query": "name = 'config.rs'# the config" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'config.rs'

lib/
config.rs  [#1]
```

## E8.4 A query of comments only is the empty query

Fixture: `flat`

```json request
{ "query": "# nothing to filter\n# yet" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1

notes/
a.md
b.md
c.md
```
