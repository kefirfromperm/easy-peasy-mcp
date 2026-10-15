# Examples for task 155

The smaller default limits of spec 2 (OUTPUT.md#defaults-of-spec-2, OUTPUT.md#limits-and-truncation): `max_files = 20`,
`max_lines = 100`, `max_matches = 200`, `max_matches_per_file = 10`, `max_output_per_file = 800`, `max_output = 8000`;
`"version": 1` and a response written to a file keep the limits of spec 1. The examples are written for the current
spec version 2.

## Fixtures

### smaller-limits

```text tree
one.txt            2026-10-01 10:00
many/f01.txt       2026-10-01 10:00
many/f02.txt       2026-10-01 10:00
many/f03.txt       2026-10-01 10:00
many/f04.txt       2026-10-01 10:00
many/f05.txt       2026-10-01 10:00
many/f06.txt       2026-10-01 10:00
many/f07.txt       2026-10-01 10:00
many/f08.txt       2026-10-01 10:00
many/f09.txt       2026-10-01 10:00
many/f10.txt       2026-10-01 10:00
many/f11.txt       2026-10-01 10:00
many/f12.txt       2026-10-01 10:00
many/f13.txt       2026-10-01 10:00
many/f14.txt       2026-10-01 10:00
many/f15.txt       2026-10-01 10:00
many/f16.txt       2026-10-01 10:00
many/f17.txt       2026-10-01 10:00
many/f18.txt       2026-10-01 10:00
many/f19.txt       2026-10-01 10:00
many/f20.txt       2026-10-01 10:00
many/f21.txt       2026-10-01 10:00
many/f22.txt       2026-10-01 10:00
many/f23.txt       2026-10-01 10:00
many/f24.txt       2026-10-01 10:00
many/f25.txt       2026-10-01 10:00
```

File contents are in `tests/fixtures/smaller-limits/`: `one.txt` has 15 lines `TODO one step 1` to `TODO one step 15`,
every `many/fNN.txt` 5 lines `TODO fNN step 1` to `TODO fNN step 5`: 26 files and 140 lines with `TODO`. With
`sort = relevance` `one.txt` comes first (shallower), the files of `many/` follow in the order of their paths.

## E155.1 A broad query with the default limits

Fixture: `smaller-limits`

1. 26 files, 140 lines: `one.txt` is cut after 10 matches, the lines end after 100, the files after 20

```json request
{"query": "text has 'TODO'"}
```
```text response
files: 26  lines: 140  matches: 140  result: r1  truncated: yes (max_files, max_lines, max_matches_per_file)  next cursor: eyJ2IjoxLCJoIjoiMWFkODBmOTAiLCJmIjoxOSwidCI6WzI2LDE0MCwxNDBdfQ

./
one.txt  lines: 15  matches: 15  (cut)
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
many/
f01.txt  lines: 5  matches: 5
1:⟦TODO⟧ f01 step 1
2:⟦TODO⟧ f01 step 2
3:⟦TODO⟧ f01 step 3
4:⟦TODO⟧ f01 step 4
5:⟦TODO⟧ f01 step 5
f02.txt  lines: 5  matches: 5
1:⟦TODO⟧ f02 step 1
2:⟦TODO⟧ f02 step 2
3:⟦TODO⟧ f02 step 3
4:⟦TODO⟧ f02 step 4
5:⟦TODO⟧ f02 step 5
f03.txt  lines: 5  matches: 5
1:⟦TODO⟧ f03 step 1
2:⟦TODO⟧ f03 step 2
3:⟦TODO⟧ f03 step 3
4:⟦TODO⟧ f03 step 4
5:⟦TODO⟧ f03 step 5
f04.txt  lines: 5  matches: 5
1:⟦TODO⟧ f04 step 1
2:⟦TODO⟧ f04 step 2
3:⟦TODO⟧ f04 step 3
4:⟦TODO⟧ f04 step 4
5:⟦TODO⟧ f04 step 5
f05.txt  lines: 5  matches: 5
1:⟦TODO⟧ f05 step 1
2:⟦TODO⟧ f05 step 2
3:⟦TODO⟧ f05 step 3
4:⟦TODO⟧ f05 step 4
5:⟦TODO⟧ f05 step 5
f06.txt  lines: 5  matches: 5
1:⟦TODO⟧ f06 step 1
2:⟦TODO⟧ f06 step 2
3:⟦TODO⟧ f06 step 3
4:⟦TODO⟧ f06 step 4
5:⟦TODO⟧ f06 step 5
f07.txt  lines: 5  matches: 5
1:⟦TODO⟧ f07 step 1
2:⟦TODO⟧ f07 step 2
3:⟦TODO⟧ f07 step 3
4:⟦TODO⟧ f07 step 4
5:⟦TODO⟧ f07 step 5
f08.txt  lines: 5  matches: 5
1:⟦TODO⟧ f08 step 1
2:⟦TODO⟧ f08 step 2
3:⟦TODO⟧ f08 step 3
4:⟦TODO⟧ f08 step 4
5:⟦TODO⟧ f08 step 5
f09.txt  lines: 5  matches: 5
1:⟦TODO⟧ f09 step 1
2:⟦TODO⟧ f09 step 2
3:⟦TODO⟧ f09 step 3
4:⟦TODO⟧ f09 step 4
5:⟦TODO⟧ f09 step 5
f10.txt  lines: 5  matches: 5
1:⟦TODO⟧ f10 step 1
2:⟦TODO⟧ f10 step 2
3:⟦TODO⟧ f10 step 3
4:⟦TODO⟧ f10 step 4
5:⟦TODO⟧ f10 step 5
f11.txt  lines: 5  matches: 5
1:⟦TODO⟧ f11 step 1
2:⟦TODO⟧ f11 step 2
3:⟦TODO⟧ f11 step 3
4:⟦TODO⟧ f11 step 4
5:⟦TODO⟧ f11 step 5
f12.txt  lines: 5  matches: 5
1:⟦TODO⟧ f12 step 1
2:⟦TODO⟧ f12 step 2
3:⟦TODO⟧ f12 step 3
4:⟦TODO⟧ f12 step 4
5:⟦TODO⟧ f12 step 5
f13.txt  lines: 5  matches: 5
1:⟦TODO⟧ f13 step 1
2:⟦TODO⟧ f13 step 2
3:⟦TODO⟧ f13 step 3
4:⟦TODO⟧ f13 step 4
5:⟦TODO⟧ f13 step 5
f14.txt  lines: 5  matches: 5
1:⟦TODO⟧ f14 step 1
2:⟦TODO⟧ f14 step 2
3:⟦TODO⟧ f14 step 3
4:⟦TODO⟧ f14 step 4
5:⟦TODO⟧ f14 step 5
f15.txt  lines: 5  matches: 5
1:⟦TODO⟧ f15 step 1
2:⟦TODO⟧ f15 step 2
3:⟦TODO⟧ f15 step 3
4:⟦TODO⟧ f15 step 4
5:⟦TODO⟧ f15 step 5
f16.txt  lines: 5  matches: 5
1:⟦TODO⟧ f16 step 1
2:⟦TODO⟧ f16 step 2
3:⟦TODO⟧ f16 step 3
4:⟦TODO⟧ f16 step 4
5:⟦TODO⟧ f16 step 5
f17.txt  lines: 5  matches: 5
1:⟦TODO⟧ f17 step 1
2:⟦TODO⟧ f17 step 2
3:⟦TODO⟧ f17 step 3
4:⟦TODO⟧ f17 step 4
5:⟦TODO⟧ f17 step 5
f18.txt  lines: 5  matches: 5
1:⟦TODO⟧ f18 step 1
2:⟦TODO⟧ f18 step 2
3:⟦TODO⟧ f18 step 3
4:⟦TODO⟧ f18 step 4
5:⟦TODO⟧ f18 step 5
f19.txt  lines: 5  matches: 5
```

2. the next page

```json request
{"query": "text has 'TODO'", "output": {"cursor": "eyJ2IjoxLCJoIjoiMWFkODBmOTAiLCJmIjoxOSwidCI6WzI2LDE0MCwxNDBdfQ"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r2

many/
f19.txt  lines: 5  matches: 5
1:⟦TODO⟧ f19 step 1
2:⟦TODO⟧ f19 step 2
3:⟦TODO⟧ f19 step 3
4:⟦TODO⟧ f19 step 4
5:⟦TODO⟧ f19 step 5
f20.txt  lines: 5  matches: 5
1:⟦TODO⟧ f20 step 1
2:⟦TODO⟧ f20 step 2
3:⟦TODO⟧ f20 step 3
4:⟦TODO⟧ f20 step 4
5:⟦TODO⟧ f20 step 5
f21.txt  lines: 5  matches: 5
1:⟦TODO⟧ f21 step 1
2:⟦TODO⟧ f21 step 2
3:⟦TODO⟧ f21 step 3
4:⟦TODO⟧ f21 step 4
5:⟦TODO⟧ f21 step 5
f22.txt  lines: 5  matches: 5
1:⟦TODO⟧ f22 step 1
2:⟦TODO⟧ f22 step 2
3:⟦TODO⟧ f22 step 3
4:⟦TODO⟧ f22 step 4
5:⟦TODO⟧ f22 step 5
f23.txt  lines: 5  matches: 5
1:⟦TODO⟧ f23 step 1
2:⟦TODO⟧ f23 step 2
3:⟦TODO⟧ f23 step 3
4:⟦TODO⟧ f23 step 4
5:⟦TODO⟧ f23 step 5
f24.txt  lines: 5  matches: 5
1:⟦TODO⟧ f24 step 1
2:⟦TODO⟧ f24 step 2
3:⟦TODO⟧ f24 step 3
4:⟦TODO⟧ f24 step 4
5:⟦TODO⟧ f24 step 5
f25.txt  lines: 5  matches: 5
1:⟦TODO⟧ f25 step 1
2:⟦TODO⟧ f25 step 2
3:⟦TODO⟧ f25 step 3
4:⟦TODO⟧ f25 step 4
5:⟦TODO⟧ f25 step 5
```

## E155.2 A file cut by `max_matches_per_file`

Fixture: `smaller-limits`

1. the default: the first 10 matches, no cursor

```json request
{"query": "name = 'one.txt' and text has 'TODO'"}
```
```text response
files: 1  lines: 15  matches: 15  result: r1  truncated: yes (max_matches_per_file)

./
one.txt  lines: 15  matches: 15  (cut)
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
```

2. the limit raised in the request

```json request
{"query": "name = 'one.txt' and text has 'TODO'", "output": {"max_matches_per_file": 20}}
```
```text response
files: 1  lines: 15  matches: 15  result: r2

./
one.txt  lines: 15  matches: 15
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
11:⟦TODO⟧ one step 11
12:⟦TODO⟧ one step 12
13:⟦TODO⟧ one step 13
14:⟦TODO⟧ one step 14
15:⟦TODO⟧ one step 15
```

## E155.3 More than 20 files

Fixture: `smaller-limits`

1. the default

```json request
{"query": "text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNDM0MTUyMjEiLCJmIjoyMCwidCI6WzI2LDE0MCwxNDBdfQ

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
f05.txt  lines: 5  matches: 5
f06.txt  lines: 5  matches: 5
f07.txt  lines: 5  matches: 5
f08.txt  lines: 5  matches: 5
f09.txt  lines: 5  matches: 5
f10.txt  lines: 5  matches: 5
f11.txt  lines: 5  matches: 5
f12.txt  lines: 5  matches: 5
f13.txt  lines: 5  matches: 5
f14.txt  lines: 5  matches: 5
f15.txt  lines: 5  matches: 5
f16.txt  lines: 5  matches: 5
f17.txt  lines: 5  matches: 5
f18.txt  lines: 5  matches: 5
f19.txt  lines: 5  matches: 5
```

2. `max_files` raised in the request

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "max_files": 30}}
```
```text response
files: 26  lines: 140  matches: 140  result: r2

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
f05.txt  lines: 5  matches: 5
f06.txt  lines: 5  matches: 5
f07.txt  lines: 5  matches: 5
f08.txt  lines: 5  matches: 5
f09.txt  lines: 5  matches: 5
f10.txt  lines: 5  matches: 5
f11.txt  lines: 5  matches: 5
f12.txt  lines: 5  matches: 5
f13.txt  lines: 5  matches: 5
f14.txt  lines: 5  matches: 5
f15.txt  lines: 5  matches: 5
f16.txt  lines: 5  matches: 5
f17.txt  lines: 5  matches: 5
f18.txt  lines: 5  matches: 5
f19.txt  lines: 5  matches: 5
f20.txt  lines: 5  matches: 5
f21.txt  lines: 5  matches: 5
f22.txt  lines: 5  matches: 5
f23.txt  lines: 5  matches: 5
f24.txt  lines: 5  matches: 5
f25.txt  lines: 5  matches: 5
```

## E155.4 The limits raised in the configuration

Fixture: `smaller-limits`

1. `update_config`

```json request update_config
{"set": "[output]\nmax_files = 30\nmax_matches_per_file = 20"}
```
```text response
[output]
max_files = 30    # was 20
max_matches_per_file = 20    # was 10
```

2. all the files

```json request
{"query": "text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r1

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
f05.txt  lines: 5  matches: 5
f06.txt  lines: 5  matches: 5
f07.txt  lines: 5  matches: 5
f08.txt  lines: 5  matches: 5
f09.txt  lines: 5  matches: 5
f10.txt  lines: 5  matches: 5
f11.txt  lines: 5  matches: 5
f12.txt  lines: 5  matches: 5
f13.txt  lines: 5  matches: 5
f14.txt  lines: 5  matches: 5
f15.txt  lines: 5  matches: 5
f16.txt  lines: 5  matches: 5
f17.txt  lines: 5  matches: 5
f18.txt  lines: 5  matches: 5
f19.txt  lines: 5  matches: 5
f20.txt  lines: 5  matches: 5
f21.txt  lines: 5  matches: 5
f22.txt  lines: 5  matches: 5
f23.txt  lines: 5  matches: 5
f24.txt  lines: 5  matches: 5
f25.txt  lines: 5  matches: 5
```

3. all the matches of a file

```json request
{"query": "name = 'one.txt' and text has 'TODO'"}
```
```text response
files: 1  lines: 15  matches: 15  result: r2

./
one.txt  lines: 15  matches: 15
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
11:⟦TODO⟧ one step 11
12:⟦TODO⟧ one step 12
13:⟦TODO⟧ one step 13
14:⟦TODO⟧ one step 14
15:⟦TODO⟧ one step 15
```

## E155.5 `"version": 1` keeps the limits of spec 1

Fixture: `smaller-limits`

1. all the files

```json request
{"version": 1, "query": "text has 'TODO'", "output": {"detail": "files"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r1
#1 text has 'TODO'

./
one.txt  [#1]  lines: 15  matches: 15
many/
f01.txt  [#1]  lines: 5  matches: 5
f02.txt  [#1]  lines: 5  matches: 5
f03.txt  [#1]  lines: 5  matches: 5
f04.txt  [#1]  lines: 5  matches: 5
f05.txt  [#1]  lines: 5  matches: 5
f06.txt  [#1]  lines: 5  matches: 5
f07.txt  [#1]  lines: 5  matches: 5
f08.txt  [#1]  lines: 5  matches: 5
f09.txt  [#1]  lines: 5  matches: 5
f10.txt  [#1]  lines: 5  matches: 5
f11.txt  [#1]  lines: 5  matches: 5
f12.txt  [#1]  lines: 5  matches: 5
f13.txt  [#1]  lines: 5  matches: 5
f14.txt  [#1]  lines: 5  matches: 5
f15.txt  [#1]  lines: 5  matches: 5
f16.txt  [#1]  lines: 5  matches: 5
f17.txt  [#1]  lines: 5  matches: 5
f18.txt  [#1]  lines: 5  matches: 5
f19.txt  [#1]  lines: 5  matches: 5
f20.txt  [#1]  lines: 5  matches: 5
f21.txt  [#1]  lines: 5  matches: 5
f22.txt  [#1]  lines: 5  matches: 5
f23.txt  [#1]  lines: 5  matches: 5
f24.txt  [#1]  lines: 5  matches: 5
f25.txt  [#1]  lines: 5  matches: 5
```

2. all the matches of a file

```json request
{"version": 1, "query": "name = 'one.txt' and text has 'TODO'"}
```
```text response
files: 1  lines: 15  matches: 15  result: r2
#1 name = 'one.txt'
#2 text has 'TODO'

./
one.txt  [#1 #2]  lines: 15  matches: 15
1:⟦#2:TODO⟧ one step 1
2:⟦#2:TODO⟧ one step 2
3:⟦#2:TODO⟧ one step 3
4:⟦#2:TODO⟧ one step 4
5:⟦#2:TODO⟧ one step 5
6:⟦#2:TODO⟧ one step 6
7:⟦#2:TODO⟧ one step 7
8:⟦#2:TODO⟧ one step 8
9:⟦#2:TODO⟧ one step 9
10:⟦#2:TODO⟧ one step 10
11:⟦#2:TODO⟧ one step 11
12:⟦#2:TODO⟧ one step 12
13:⟦#2:TODO⟧ one step 13
14:⟦#2:TODO⟧ one step 14
15:⟦#2:TODO⟧ one step 15
```

## E155.6 A batch

Fixture: `smaller-limits`

Every query has the default limits of spec 2: `one.txt` is cut after 10 matches, the list of `all` after 20 files.

```json request
{"queries": [{"label": "one", "query": "name = 'one.txt' and text has 'TODO'"}, {"label": "all", "query": "text has 'TODO'", "output": {"detail": "files"}}]}
```
```text response
== one ==  files: 1  lines: 15  matches: 15  result: r1  truncated: yes (max_matches_per_file)

./
one.txt  lines: 15  matches: 15  (cut)
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
== all ==  files: 26  lines: 140  matches: 140  result: r2  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiYzVkZTA1NmMiLCJmIjoyMCwidCI6WzI2LDE0MCwxNDBdfQ

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
f05.txt  lines: 5  matches: 5
f06.txt  lines: 5  matches: 5
f07.txt  lines: 5  matches: 5
f08.txt  lines: 5  matches: 5
f09.txt  lines: 5  matches: 5
f10.txt  lines: 5  matches: 5
f11.txt  lines: 5  matches: 5
f12.txt  lines: 5  matches: 5
f13.txt  lines: 5  matches: 5
f14.txt  lines: 5  matches: 5
f15.txt  lines: 5  matches: 5
f16.txt  lines: 5  matches: 5
f17.txt  lines: 5  matches: 5
f18.txt  lines: 5  matches: 5
f19.txt  lines: 5  matches: 5
```

## E155.7 A response written to a file keeps the limits of spec 1

Fixture: `smaller-limits`

The count limits which neither the request nor the configuration sets are the ones of spec 1 in a file.

1. all the files

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "file": "files.txt"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r1
written: <output_dir>/files.txt  lines: 30  bytes: 840
```
```text file files.txt
files: 26  lines: 140  matches: 140  result: r1

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
f05.txt  lines: 5  matches: 5
f06.txt  lines: 5  matches: 5
f07.txt  lines: 5  matches: 5
f08.txt  lines: 5  matches: 5
f09.txt  lines: 5  matches: 5
f10.txt  lines: 5  matches: 5
f11.txt  lines: 5  matches: 5
f12.txt  lines: 5  matches: 5
f13.txt  lines: 5  matches: 5
f14.txt  lines: 5  matches: 5
f15.txt  lines: 5  matches: 5
f16.txt  lines: 5  matches: 5
f17.txt  lines: 5  matches: 5
f18.txt  lines: 5  matches: 5
f19.txt  lines: 5  matches: 5
f20.txt  lines: 5  matches: 5
f21.txt  lines: 5  matches: 5
f22.txt  lines: 5  matches: 5
f23.txt  lines: 5  matches: 5
f24.txt  lines: 5  matches: 5
f25.txt  lines: 5  matches: 5
```

2. all the matches of a file

```json request
{"query": "name = 'one.txt' and text has 'TODO'", "output": {"file": "one.txt"}}
```
```text response
files: 1  lines: 15  matches: 15  result: r2
written: <output_dir>/one.txt  lines: 19  bytes: 453
```
```text file one.txt
files: 1  lines: 15  matches: 15  result: r2

./
one.txt  lines: 15  matches: 15
1:⟦TODO⟧ one step 1
2:⟦TODO⟧ one step 2
3:⟦TODO⟧ one step 3
4:⟦TODO⟧ one step 4
5:⟦TODO⟧ one step 5
6:⟦TODO⟧ one step 6
7:⟦TODO⟧ one step 7
8:⟦TODO⟧ one step 8
9:⟦TODO⟧ one step 9
10:⟦TODO⟧ one step 10
11:⟦TODO⟧ one step 11
12:⟦TODO⟧ one step 12
13:⟦TODO⟧ one step 13
14:⟦TODO⟧ one step 14
15:⟦TODO⟧ one step 15
```

3. a limit given in the request applies to the file too

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "max_files": 5, "file": "five.txt"}}
```
```text response
files: 26  lines: 140  matches: 140  result: r3  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMjVjZmE0NGEiLCJmIjo1LCJ0IjpbMjYsMTQwLDE0MF19
written: <output_dir>/five.txt  lines: 9  bytes: 313
```
```text file five.txt
files: 26  lines: 140  matches: 140  result: r3  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMjVjZmE0NGEiLCJmIjo1LCJ0IjpbMjYsMTQwLDE0MF19

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
f02.txt  lines: 5  matches: 5
f03.txt  lines: 5  matches: 5
f04.txt  lines: 5  matches: 5
```

## E155.8 The funnel of a huge result

Fixture: `smaller-limits`

A result of more than 10 × `max_files` files gets the funnel and the hints, which say where the matches are; with the
default `max_files` from 201 files on.

```json request
{"query": "text has 'TODO'", "output": {"detail": "files", "max_files": 2}}
```
```text response
files: 26  lines: 140  matches: 140  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiMmRkMTFhZTciLCJmIjoyLCJ0IjpbMjYsMTQwLDE0MF19
funnel:
  text has 'TODO'                       26
hints:
  dir: many/ 25 · ./ 1
  lang: unknown 26
  narrow with: not path ~ 'many/**', kind = source, /w

./
one.txt  lines: 15  matches: 15
many/
f01.txt  lines: 5  matches: 5
```

## E155.9 `get_config` shows the new defaults

Fixture: `smaller-limits`

```json request get_config
{"section": "output"}
```
```text response
# easy-peasy-mcp <version>, spec 2, supported specs: 1, 2
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[output]
format = "text"
detail = "lines"
count_by = "total"
# budget =  (not set)
chars_per_token = 4
funnel = "auto"
facets = ['dir', 'lang', 'kind', 'condition']
facet_depth = 2
facet_limit = 8
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
groups = []
only_match = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 20
max_lines = 100
max_matches = 200
max_matches_per_file = 10
max_line_length = 300
max_output_per_file = 800
max_output = 8000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
repeated_locations = 3
sort = "relevance"
path_style = "relative"
omit = ['warnings']
show = []
```
