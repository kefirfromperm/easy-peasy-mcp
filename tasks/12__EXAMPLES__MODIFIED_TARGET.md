# Examples for task 12

Spec: `1`

## Fixtures

### dates

```text tree
logs/f1.log                2026-10-05 08:00
logs/f2.log                2026-10-02 00:00
logs/f3.log                2026-10-01 23:59:59
logs/f4.log                2026-10-01 12:00
logs/f5.log                2026-10-01 00:00
logs/f6.log                2026-09-30 23:59:59
```

```text setup
file logs/f1.log 10
file logs/f2.log 10
file logs/f3.log 10
file logs/f4.log 10
file logs/f5.log 10
file logs/f6.log 10
```

Now: `2026-10-08 12:00`

The files are created by the test setup. Times are UTC and the test server runs with UTC as its local time zone, so
dates without an offset are UTC too. The newer a file, the smaller its number: with relevance sorting newer files
come first, and so they do in path order.

## E12.1 `=` with a date: the whole day

Fixture: `dates`

```json request
{ "query": "modified = 2026-10-01" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 modified = 2026-10-01

logs/
f3.log  [#1]
f4.log  [#1]
f5.log  [#1]
```

## E12.2 `>` with a date: from the next day

Fixture: `dates`

```json request
{ "query": "modified > 2026-10-01" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 modified > 2026-10-01

logs/
f1.log  [#1]
f2.log  [#1]
```

## E12.3 `>=` with a date: from the start of the day

Fixture: `dates`

```json request
{ "query": "modified >= 2026-10-01" }
```

```text response
files: 5  lines: 0  matches: 0  result: r1
#1 modified >= 2026-10-01

logs/
f1.log  [#1]
f2.log  [#1]
f3.log  [#1]
f4.log  [#1]
f5.log  [#1]
```

## E12.4 `<` with a date: before the day

Fixture: `dates`

```json request
{ "query": "modified < 2026-10-01" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 modified < 2026-10-01

logs/
f6.log  [#1]
```

## E12.5 `<=` with a date: until the end of the day

Fixture: `dates`

```json request
{ "query": "modified <= 2026-10-01" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 modified <= 2026-10-01

logs/
f3.log  [#1]
f4.log  [#1]
f5.log  [#1]
f6.log  [#1]
```

## E12.6 `!=` with a date

Fixture: `dates`

```json request
{ "query": "modified != 2026-10-01" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 modified != 2026-10-01

logs/
f1.log  [#1]
f2.log  [#1]
f6.log  [#1]
```

## E12.7 A relative date

Seven days before now is 2026-10-01 12:00; `f4.log` was modified exactly then and is not newer.

Fixture: `dates`

```json request
{ "query": "modified > -7d" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 modified > -7d

logs/
f1.log  [#1]
f2.log  [#1]
f3.log  [#1]
```

## E12.8 A range of relative dates

From 2026-10-01 12:00 to 2026-10-05 12:00, both included.

Fixture: `dates`

```json request
{ "query": "modified in -7d..-3d" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 modified in -7d..-3d

logs/
f1.log  [#1]
f2.log  [#1]
f3.log  [#1]
f4.log  [#1]
```

## E12.9 A range of dates, both days included

Fixture: `dates`

```json request
{ "query": "modified in 2026-09-30..2026-10-01" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 modified in 2026-09-30..2026-10-01

logs/
f3.log  [#1]
f4.log  [#1]
f5.log  [#1]
f6.log  [#1]
```

## E12.10 A date-time with an offset: the whole minute

Fixture: `dates`

```json request
{ "query": "modified = 2026-10-01T14:00+02:00" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 modified = 2026-10-01T14:00+02:00

logs/
f4.log  [#1]
```

## E12.11 A date-time with seconds and `Z`

Fixture: `dates`

```json request
{ "query": "modified >= 2026-10-01T23:59:59Z" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 modified >= 2026-10-01T23:59:59Z

logs/
f1.log  [#1]
f2.log  [#1]
f3.log  [#1]
```

## E12.12 Weeks, hours, minutes

From 2026-10-01 12:00 to 2026-10-04 18:00.

Fixture: `dates`

```json request
{ "query": "modified > -1w and modified < -90h" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 modified > -1w
#2 modified < -90h

logs/
f2.log  [#1 #2]
f3.log  [#1 #2]
```

## E12.13 Seconds

600000 seconds before now is 2026-10-01 13:20.

Fixture: `dates`

```json request
{ "query": "modified < -600000s" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 modified < -600000s

logs/
f4.log  [#1]
f5.log  [#1]
f6.log  [#1]
```

## E12.14 Unknown unit of a relative date

Fixture: `dates`

```json request
{ "query": "modified > -1y" }
```

```text error
error[invalid_date]: invalid date '-1y'
  modified > -1y
             ^^^
  hint: units: s, m, h, d, w
```

## E12.15 Invalid absolute date

Fixture: `dates`

```json request
{ "query": "modified > 2026-13-01" }
```

```text error
error[invalid_date]: invalid date '2026-13-01'
  modified > 2026-13-01
             ^^^^^^^^^^
  hint: ISO 8601: 2026-10-01, 2026-10-01T14:30, 2026-10-01T14:30:00Z, 2026-10-01T14:30:00+02:00
```

## E12.16 A range which ends before it starts

Fixture: `dates`

```json request
{ "query": "modified in 2026-10-02..2026-10-01" }
```

```text error
error[invalid_range]: invalid range '2026-10-02..2026-10-01'
  modified in 2026-10-02..2026-10-01
              ^^^^^^^^^^^^^^^^^^^^^^
  hint: the start must not be greater than the end
```
