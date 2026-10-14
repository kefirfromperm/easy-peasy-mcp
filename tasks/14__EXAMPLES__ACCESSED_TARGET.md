# Examples for task 14

Spec: `1`

## Fixtures

### atimes

```text tree
logs/a.log                 2026-10-01 10:00
logs/b.log                 2026-10-01 10:00
logs/c.log                 2026-10-01 10:00
```

```text setup
file logs/a.log 10
file logs/b.log 10
file logs/c.log 10
atime logs/a.log 2026-10-07 09:00
atime logs/b.log 2026-09-01 00:00
atime logs/c.log 2026-08-01 00:00
```

Now: `2026-10-08 12:00`

The files are created by the test setup with the same modification time and different access times (UTC). The test
server runs with UTC as its local time zone.

## E14.1 Not accessed for a month

Fixture: `atimes`

```json request
{ "query": "accessed < -30d" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 accessed < -30d

logs/
b.log  [#1]
c.log  [#1]
```

## E14.2 Accessed recently

Fixture: `atimes`

```json request
{ "query": "accessed > -2d" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 accessed > -2d

logs/
a.log  [#1]
```

## E14.3 A whole day

Fixture: `atimes`

```json request
{ "query": "accessed = 2026-09-01" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 accessed = 2026-09-01

logs/
b.log  [#1]
```

## E14.4 A range

Fixture: `atimes`

```json request
{ "query": "accessed in 2026-08-01..2026-09-01" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 accessed in 2026-08-01..2026-09-01

logs/
b.log  [#1]
c.log  [#1]
```

## E14.5 `not`

Fixture: `atimes`

```json request
{ "query": "not accessed > -7d" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 accessed > -7d

logs/
b.log
c.log
```

## E14.6 Invalid date

Fixture: `atimes`

```json request
{ "query": "accessed < -1month" }
```

```text error
error[invalid_date]: invalid date '-1month'
  accessed < -1month
             ^^^^^^^
  hint: units: s, m, h, d, w
```
