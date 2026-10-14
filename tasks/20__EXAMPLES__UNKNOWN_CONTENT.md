# Examples for task 20

Spec: `1`

## Fixtures

### binary

```text tree
assets/notes.txt           2026-10-01 10:00
assets/other.txt           2026-10-01 10:00
assets/sprite.bin          2026-10-01 10:00
```

File contents are in `tests/fixtures/binary/`; `assets/sprite.bin` has zero bytes and the text `logo`. Times are UTC.

## E20.1 A binary file is not returned because of its content

`text has 'logo'` is unknown for the binary file, so it is not returned, and the response has a warning.

Fixture: `binary`

```json request
{ "query": "text has 'logo'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'logo'

assets/
notes.txt  [#1]  lines: 1  matches: 1
1:the ⟦#1:logo⟧ is here

warning: binary_skipped (1): assets/sprite.bin
```

## E20.2 Neither by `not`

`not unknown` is unknown.

Fixture: `binary`

```json request
{ "query": "not text has 'logo'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 text has 'logo'

assets/
other.txt

warning: binary_skipped (1): assets/sprite.bin
```

## E20.3 But by other conditions

Fixture: `binary`

```json request
{ "query": "name ~ '*.bin' or text has 'logo'" }
```

```text response
files: 2  lines: 1  matches: 1  result: r1
#1 name ~ '*.bin'
#2 text has 'logo'

assets/
notes.txt  [#2]  lines: 1  matches: 1
1:the ⟦#2:logo⟧ is here
sprite.bin  [#1]

warning: binary_skipped (1): assets/sprite.bin
```

## E20.4 A binary file excluded by other conditions is not read

The name decides for `assets/sprite.bin` before its content is needed, so it has no warning.

Fixture: `binary`

```json request
{ "query": "name ~ '*.txt' and text has 'see'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ '*.txt'
#2 text has 'see'

assets/
other.txt  [#1 #2]  lines: 1  matches: 1
1:nothing to ⟦#2:see⟧
```
