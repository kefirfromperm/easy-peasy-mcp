# Examples for task 13

Creation dates can't be set portably, so the examples use conditions which are true for any creation date of the
fixture files: they are created when the test runs. The fixture `flat` is described in
[2__EXAMPLES__EMPTY_QUERY.md](2__EXAMPLES__EMPTY_QUERY.md). A file system without creation dates is covered by
unit tests.

## E13.1 `created` after a date

Fixture: `flat`

```json request
{ "query": "created > 2000-01-01" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 created > 2000-01-01

notes/
a.md  [#1]
b.md  [#1]
c.md  [#1]
```

## E13.2 With other conditions

Fixture: `flat`

```json request
{ "query": "created >= 2000-01-01T00:00Z and name = 'b.md'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 created >= 2000-01-01T00:00Z
#2 name = 'b.md'

notes/
b.md  [#1 #2]
```

## E13.3 A range

Fixture: `flat`

```json request
{ "query": "created in 2000-01-01..2999-12-31" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 created in 2000-01-01..2999-12-31

notes/
a.md  [#1]
b.md  [#1]
c.md  [#1]
```

## E13.4 `not` over a false condition

Fixture: `flat`

```json request
{ "query": "not created < 2000-01-01" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 created < 2000-01-01

notes/
a.md
b.md
c.md
```

## E13.5 Invalid date

Fixture: `flat`

```json request
{ "query": "created > yesterday" }
```

```text error
error[unexpected_token]: unexpected 'yesterday'
  created > yesterday
            ^^^^^^^^^
  hint: a value is expected after '>'
```

## E13.6 Invalid relative date

Fixture: `flat`

```json request
{ "query": "created > -2y" }
```

```text error
error[invalid_date]: invalid date '-2y'
  created > -2y
            ^^^
  hint: units: s, m, h, d, w
```
