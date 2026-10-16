# Examples for task 152

The rule of spec 2 leaves out the part `match_counts` of the text format — `matches: N` of the header and of the
files, `: 1` of `count_by = line` — when every matched line of the whole result has one match
(OUTPUT.md#one-match-per-line). The examples are written for the current spec version 2.

## Fixtures

### match-counts

```text tree
a.rs               2026-10-01 10:00
b.rs               2026-10-01 10:00
c.txt              2026-10-01 10:00
```

File contents are in `tests/fixtures/match-counts/`: `a.rs` has `foo` in lines 1 and 2 (and `a` in line 2), `b.rs`
has `foo` twice in line 1, `c.txt` is `a a` and `b`.

The other examples use the fixture `omit` of task 134: `src/lib.rs` with `unwrap(` in lines 2 and 4, `src/main.rs`
with `unwrap(` in line 2 and `TODO` in line 3, the binary `data.bin` which gives `binary_skipped`.

## E152.1 One match in every line

Fixture: `omit`

Every matched line has one `unwrap(`: no `matches:` in the header or after a path, and `count_by = line` lists the
line numbers.

1. lines

```json request
{"query": "text has 'unwrap('"}
```
```text response
files: 2  lines: 3  result: r1

src/
lib.rs  lines: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);

warnings left out: binary_skipped (1)
```

2. files

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  result: r2

src/
lib.rs  lines: 2
main.rs  lines: 1

warnings left out: binary_skipped (1)
```

3. count

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count"}}
```
```text response
files: 2  lines: 3  result: r3

warnings left out: binary_skipped (1)
```

4. count by file

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 2  lines: 3  result: r4

src/
lib.rs  lines: 2
main.rs  lines: 1

warnings left out: binary_skipped (1)
```

5. count by line

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 2  lines: 3  result: r5

src/
lib.rs  lines: 2
  2
  4
main.rs  lines: 1
  2

warnings left out: binary_skipped (1)
```

## E152.2 No matched lines

Fixture: `omit`

A search of names has no line with another number of matches than one.

```json request
{"query": "name ~ '*.rs'", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 0  result: r1

src/
lib.rs
main.rs
```

## E152.3 A line with two matches

Fixture: `match-counts`

Line 1 of `b.rs` has two matches: every count is written, also of `a.rs` with one match per line.

1. lines

```json request
{"query": "text has 'foo'"}
```
```text response
files: 2  lines: 3  matches: 4  result: r1

./
a.rs  lines: 2  matches: 2
1:⟦foo⟧(1);
2:bar(⟦foo⟧);
b.rs  lines: 1  matches: 2
1:⟦foo⟧(⟦foo⟧);
```

2. count by line

```json request
{"query": "text has 'foo'", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 2  lines: 3  matches: 4  result: r2

./
a.rs  lines: 2  matches: 2
  1: 1
  2: 1
b.rs  lines: 1  matches: 2
  1: 2
```

## E152.4 The lines and the matches add up the same

Fixture: `match-counts`

`c.txt` has two matches in line 1, the second one continues into line 2, which has `0`: `lines: 2  matches: 2`, but
not one match per line, so every count is written.

```json request
{"query": "text =~ `a(\\nb)?` /m", "output": {"detail": "count", "count_by": "line"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

./
c.txt  lines: 2  matches: 2
  1: 2
  2: 0
a.rs  lines: 1  matches: 1
  2: 1
```

## E152.5 A cursor

Fixture: `match-counts`

The first page shows only `a.rs`, with one match per line, but the result has `b.rs`: the decision is made over the
whole result, so both pages write the counts.

1. the first page

```json request
{"query": "text has 'foo'", "output": {"detail": "files", "sort": "path asc", "max_files": 1}}
```
```text response
files: 2  lines: 3  matches: 4  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiM2UxMThmYTYiLCJmIjoxLCJ0IjpbMiwzLDRdfQ

./
a.rs  lines: 2  matches: 2
```

2. the next page

```json request
{"query": "text has 'foo'", "output": {"detail": "files", "sort": "path asc", "max_files": 1, "cursor": "eyJ2IjoxLCJoIjoiM2UxMThmYTYiLCJmIjoxLCJ0IjpbMiwzLDRdfQ"}}
```
```text response
files: 2  lines: 3  matches: 4  result: r2

./
b.rs  lines: 1  matches: 2
```

## E152.6 Show and omit of the part

Fixture: `omit`

1. `show` brings the numbers back

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files", "show": ["match_counts"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 2  matches: 2
main.rs  lines: 1  matches: 1

warnings left out: binary_skipped (1)
```

2. the counts of the lines too

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "show": ["match_counts"]}}
```
```text response
files: 2  lines: 3  matches: 3  result: r2

src/
lib.rs  lines: 2  matches: 2
  2: 1
  4: 1
main.rs  lines: 1  matches: 1
  2: 1

warnings left out: binary_skipped (1)
```

3. `show = ["line_counts"]` alone doesn't: the counts of the lines belong to `match_counts` too

```json request
{"query": "text has 'unwrap('", "output": {"detail": "count", "count_by": "line", "show": ["line_counts"]}}
```
```text response
files: 2  lines: 3  result: r3

src/
lib.rs  lines: 2
  2
  4
main.rs  lines: 1
  2

warnings left out: binary_skipped (1)
```

4. `show = "all"`, the output of spec 1

```json request
{"query": "text has 'unwrap('", "output": {"detail": "files", "show": "all"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r4
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

5. `"version": 1`

```json request
{"version": 1, "query": "text has 'unwrap('", "output": {"detail": "files"}}
```
```text response
files: 2  lines: 3  matches: 3  result: r5
#1 text has 'unwrap('

src/
lib.rs  [#1]  lines: 2  matches: 2
main.rs  [#1]  lines: 1  matches: 1

warning: binary_skipped (1): data.bin
```

## E152.7 Omit of the part with two matches in a line

Fixture: `match-counts`

The agent asked to leave the numbers out: they are left out whatever the matches, in `count_by = line` too.

```json request
{"query": "text has 'foo'", "output": {"detail": "count", "count_by": "line", "omit": ["match_counts"]}}
```
```text response
files: 2  lines: 3  result: r1

./
a.rs  lines: 2
  1
  2
b.rs  lines: 1
  1
```

## E152.8 JSON keeps the matches

Fixture: `omit`

```json request
{"query": "text has 'unwrap('", "output": {"format": "json", "detail": "count", "count_by": "line"}}
```
```json response
{"version":2,"exists":true,"result":"r1","counts":{"files":2,"lines":3,"matches":3},"files":[{"path":"src/lib.rs","lines":2,"matches":2,"matched_lines":[{"n":2,"matches":1},{"n":4,"matches":1}]},{"path":"src/main.rs","lines":1,"matches":1,"matched_lines":[{"n":2,"matches":1}]}],"left_out":{"warnings":{"binary_skipped":1}}}
```
