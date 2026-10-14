# Examples for task 11

Spec: `1`

## Fixtures

### sizes

```text tree
data/b512.dat              2026-10-01 10:00
data/k1.dat                2026-10-01 10:00
data/k10.dat               2026-10-01 10:00
data/k1_5.dat              2026-10-01 10:00
data/m1.dat                2026-10-01 10:00
data/m1_5.dat              2026-10-01 10:00
data/zero.dat              2026-10-01 10:00
```

```text setup
file data/b512.dat 512
file data/k1.dat 1024
file data/k10.dat 10240
file data/k1_5.dat 1536
file data/m1.dat 1048576
file data/m1_5.dat 1572864
file data/zero.dat 0
```

All files are created by the test setup with the given number of bytes. Times are UTC.

## E11.1 `size = 0`

Fixture: `sizes`

```json request
{ "query": "size = 0" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 size = 0

data/
zero.dat  [#1]
```

## E11.2 `>`

`data/m1.dat` has exactly 1M and is not greater.

Fixture: `sizes`

```json request
{ "query": "size > 1M" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 size > 1M

data/
m1_5.dat  [#1]
```

## E11.3 `>=`

Fixture: `sizes`

```json request
{ "query": "size >= 1M" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 size >= 1M

data/
m1.dat  [#1]
m1_5.dat  [#1]
```

## E11.4 `<`

Fixture: `sizes`

```json request
{ "query": "size < 1K" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 size < 1K

data/
b512.dat  [#1]
zero.dat  [#1]
```

## E11.5 `<=`

Units may be written in lower case.

Fixture: `sizes`

```json request
{ "query": "size <= 1k" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 size <= 1k

data/
b512.dat  [#1]
k1.dat  [#1]
zero.dat  [#1]
```

## E11.6 `!=` and bytes with `B`

Fixture: `sizes`

```json request
{ "query": "size != 0 and size < 1536B" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 size != 0
#2 size < 1536B

data/
b512.dat  [#1 #2]
k1.dat  [#1 #2]
```

## E11.7 A fraction

Fixture: `sizes`

```json request
{ "query": "size = 1.5K" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 size = 1.5K

data/
k1_5.dat  [#1]
```

## E11.8 A range, both ends included

Fixture: `sizes`

```json request
{ "query": "size in 1K..10K" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 size in 1K..10K

data/
k1.dat  [#1]
k10.dat  [#1]
k1_5.dat  [#1]
```

## E11.9 `G`

Fixture: `sizes`

```json request
{ "query": "size < 1G and size > 1M" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 size < 1G
#2 size > 1M

data/
m1_5.dat  [#1 #2]
```

## E11.10 A number without a unit

Fixture: `sizes`

```json request
{ "query": "size = 512" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 size = 512

data/
b512.dat  [#1]
```

## E11.11 Invalid size

Fixture: `sizes`

```json request
{ "query": "size > 10MB" }
```

```text error
error[invalid_size]: invalid size '10MB'
  size > 10MB
         ^^^^
  hint: units: B, K, M, G
```

## E11.12 Invalid range

Fixture: `sizes`

```json request
{ "query": "size in 10M..1M" }
```

```text error
error[invalid_range]: invalid range '10M..1M'
  size in 10M..1M
          ^^^^^^^
  hint: the start must not be greater than the end
```

## E11.13 A text operator on `size`

Fixture: `sizes`

```json request
{ "query": "size has '1'" }
```

```text error
error[operator_not_allowed]: operator 'has' is not allowed for 'size'
  size has '1'
       ^^^
  hint: expected one of: =, !=, <, <=, >, >=, in
```
