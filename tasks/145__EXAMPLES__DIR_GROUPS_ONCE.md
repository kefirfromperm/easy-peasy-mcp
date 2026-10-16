# Examples for task 145

With `sort = relevance` and `group_dirs = true` (both the defaults) the files of a directory are put together, so
every directory gets one header: the directories in the order of their most relevant file, the files of a directory
in their relevance order (OUTPUT.md#directory-groups). The order is the order of the result: in JSON, for the cursor,
`max_files` and the budget.

## Fixtures

### dir-groups

```text tree
app/d.rs           2026-10-01 10:00
app/f.rs           2026-10-01 10:00
app/h.rs           2026-10-01 10:00
app/k.rs           2026-10-01 10:00
app/n.rs           2026-10-01 10:00
lib/b.rs           2026-10-01 10:00
lib/e.rs           2026-10-01 10:00
lib/i.rs           2026-10-01 10:00
lib/l.rs           2026-10-01 10:00
lib/o.rs           2026-10-01 10:00
src/a.rs           2026-10-01 10:00
src/c.rs           2026-10-01 10:00
src/g.rs           2026-10-01 10:00
src/j.rs           2026-10-01 10:00
src/m.rs           2026-10-01 10:00
```

File contents are in `tests/fixtures/dir-groups/`. Every file but `app/f.rs` (`// done`) starts with the line
`// todo <name>` (`// todo a` in `a.rs`) followed by lines `let xN = N;`: none in `a.rs`, 1 in `b.rs`, 4 in `c.rs`,
then the squares of 3 to 13 in `d.rs`, `e.rs`, `g.rs`, `h.rs`, `i.rs`, `j.rs`, `k.rs`, `l.rs`, `m.rs`, `n.rs`, `o.rs`.
The density of the matches falls in that order, so with `group_dirs = false` the files of `text has 'todo'` are in the
order `src/a.rs`, `lib/b.rs`, `src/c.rs`, `app/d.rs`, `lib/e.rs`, `src/g.rs`, `app/h.rs`, `lib/i.rs`, `src/j.rs`,
`app/k.rs`, `lib/l.rs`, `src/m.rs`, `app/n.rs`, `lib/o.rs`: the three directories take turns.

## E145.1 One header per directory

Fixture: `dir-groups`

`src/` has the most relevant file, then `lib/` (`b.rs`), then `app/` (`d.rs`); `src/c.rs` comes up to `src/a.rs`,
`lib/e.rs` to `lib/b.rs`.

```json request
{"query": "line(text =~ `todo [a-e]`)"}
```
```text response
files: 5  lines: 5  result: r1

src/
a.rs  lines: 1
1:// ⟦todo a⟧
c.rs  lines: 1
1:// ⟦todo c⟧
lib/
b.rs  lines: 1
1:// ⟦todo b⟧
e.rs  lines: 1
1:// ⟦todo e⟧
app/
d.rs  lines: 1
1:// ⟦todo d⟧
```

## E145.2 The files and the counts per file

Fixture: `dir-groups`

1. `detail = files`

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files"}}
```
```text response
files: 5  lines: 5  result: r1

src/
a.rs  lines: 1
c.rs  lines: 1
lib/
b.rs  lines: 1
e.rs  lines: 1
app/
d.rs  lines: 1
```

2. `detail = count` with `count_by = file`

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "count", "count_by": "file"}}
```
```text response
files: 5  lines: 5  result: r2

src/
a.rs  lines: 1
c.rs  lines: 1
lib/
b.rs  lines: 1
e.rs  lines: 1
app/
d.rs  lines: 1
```

## E145.3 JSON in the same order

Fixture: `dir-groups`

JSON has no headers, but its files are in the order of the text.

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files", "format": "json"}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "counts": {"files": 5, "lines": 5, "matches": 5},
  "files": [
    {"path": "src/a.rs", "lines": 1, "matches": 1},
    {"path": "src/c.rs", "lines": 1, "matches": 1},
    {"path": "lib/b.rs", "lines": 1, "matches": 1},
    {"path": "lib/e.rs", "lines": 1, "matches": 1},
    {"path": "app/d.rs", "lines": 1, "matches": 1}
  ]
}
```

## E145.4 `max_files` and the next page

Fixture: `dir-groups`

`max_files` takes the first files of the grouped order; the next page continues in it.

1. the first page

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files", "max_files": 3}}
```
```text response
files: 5  lines: 5  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNjNiY2M4OWQiLCJmIjozLCJ0IjpbNSw1LDVdfQ

src/
a.rs  lines: 1
c.rs  lines: 1
lib/
b.rs  lines: 1
```

2. the next page

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files", "max_files": 3, "cursor": "eyJ2IjoxLCJoIjoiNjNiY2M4OWQiLCJmIjozLCJ0IjpbNSw1LDVdfQ"}}
```
```text response
files: 5  lines: 5  result: r2

lib/
e.rs  lines: 1
app/
d.rs  lines: 1
```

## E145.5 A file list cut by the budget

Fixture: `dir-groups`

The budget lowers the detail to the files and cuts the list in the grouped order: the ten files of `src/` and `lib/`
fit, the files of `app/` are on the next page, which fits as requested.

1. the first page

```json request
{"query": "text has 'todo'", "output": {"budget": 115, "show": ["match_counts"]}}
```
```text response
files: 14  lines: 14  matches: 14  result: r1  detail: files (budget 115)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiMWYzZmJjOGMiLCJmIjoxMCwidCI6WzE0LDE0LDE0XX0

src/
a.rs  lines: 1  matches: 1
c.rs  lines: 1  matches: 1
g.rs  lines: 1  matches: 1
j.rs  lines: 1  matches: 1
m.rs  lines: 1  matches: 1
lib/
b.rs  lines: 1  matches: 1
e.rs  lines: 1  matches: 1
i.rs  lines: 1  matches: 1
l.rs  lines: 1  matches: 1
o.rs  lines: 1  matches: 1
```

2. the next page

```json request
{"query": "text has 'todo'", "output": {"budget": 115, "show": ["match_counts"], "cursor": "eyJ2IjoxLCJoIjoiMWYzZmJjOGMiLCJmIjoxMCwidCI6WzE0LDE0LDE0XX0"}}
```
```text response
files: 14  lines: 14  matches: 14  result: r2

app/
d.rs  lines: 1  matches: 1
1:// ⟦todo⟧ d
h.rs  lines: 1  matches: 1
1:// ⟦todo⟧ h
k.rs  lines: 1  matches: 1
1:// ⟦todo⟧ k
n.rs  lines: 1  matches: 1
1:// ⟦todo⟧ n
```

## E145.6 Without directory groups and with another sort

Fixture: `dir-groups`

1. `group_dirs = false` keeps the order of the scores

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files", "group_dirs": false}}
```
```text response
files: 5  lines: 5  result: r1

src/a.rs  lines: 1
lib/b.rs  lines: 1
src/c.rs  lines: 1
app/d.rs  lines: 1
lib/e.rs  lines: 1
```

2. `sort = path asc` is not regrouped: the files of a directory are together anyway

```json request
{"query": "line(text =~ `todo [a-e]`)", "output": {"detail": "files", "sort": "path asc"}}
```
```text response
files: 5  lines: 5  result: r2

app/
d.rs  lines: 1
lib/
b.rs  lines: 1
e.rs  lines: 1
src/
a.rs  lines: 1
c.rs  lines: 1
```
