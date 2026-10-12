# Examples for task 27

## Fixtures

### lens

```text tree
config/app.conf            2026-10-01 10:00
config/long.txt            2026-10-01 10:00
config/tabs.txt            2026-10-01 10:00
```

File contents are in `tests/fixtures/lens/`: `config/app.conf` is the configuration of the spec example,
`config/long.txt` has a line of 150 `x`, `config/tabs.txt` has a line `<tab>x` and a line `äöü`. Times are UTC.

## E27.1 Lines which are not comments and not empty, like `grep -v`

The group has no positive text condition: every matched line is one fragment with the id of the group.

Fixture: `lens`

```json request
{ "query": "name = 'app.conf' and line(not text starts '#' and len > 0)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'app.conf'
#2 line(not text starts '#' and len > 0)
#3 text starts '#'
#4 len > 0

config/
app.conf  [#1 #2]  lines: 2  matches: 2
4:⟦#2:listen = 8080⟧
--
9:⟦#2:log_level = debug⟧
```

## E27.2 Long lines

Fixture: `lens`

```json request
{ "query": "line(len > 120)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 line(len > 120)
#2 len > 120

config/
long.txt  [#1]  lines: 1  matches: 1
1:⟦#1:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx⟧
```

## E27.3 Empty lines

Fixture: `lens`

```json request
{ "query": "name = 'app.conf' and line(len = 0)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'app.conf'
#2 line(len = 0)
#3 len = 0

config/
app.conf  [#1 #2]  lines: 2  matches: 2
5:
--
8:
```

## E27.4 A tab and a non-ASCII letter are one character each

Fixture: `lens`

```json request
{ "query": "name = 'tabs.txt' and line(len in 2..3)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'tabs.txt'
#2 line(len in 2..3)
#3 len in 2..3

config/
tabs.txt  [#1 #2]  lines: 2  matches: 2
1:	x
2:äöü
```

## E27.5 `len` with text conditions on the same line

`len` never produces fragments. `longer line here` in `config/tabs.txt` has 16 characters.

Fixture: `lens`

```json request
{ "query": "line(text has 'line' and len < 15)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 line(text has 'line' and len < 15)
#2 text has 'line'
#3 len < 15

config/
long.txt  [#1]  lines: 1  matches: 1
2:short ⟦#1,#2:line⟧
```

## E27.6 `len` outside of a group

Fixture: `lens`

```json request
{ "query": "len > 100" }
```

```text error
error[line_target_outside]: 'len' outside of line( ... )
  len > 100
  ^^^
  hint: 'len' works only inside line( ... ) or near( ... )
```
