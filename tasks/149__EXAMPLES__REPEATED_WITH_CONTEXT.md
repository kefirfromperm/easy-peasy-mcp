# Examples for task 149

In spec 2 a matched line with context lines is grouped in the `repeated` section only when its block, the line with
its context, is the same in the files (OUTPUT.md#repeated-lines-with-context). The examples are written for the
current spec version 2, except E149.6.

## Fixtures

### repeated-context

```text tree
a.md          2026-10-01 10:00
b.md          2026-10-01 10:00
c.md          2026-10-01 10:00
d.md          2026-10-01 10:00
```

File contents are in `tests/fixtures/repeated-context/`. Every file has a heading `# <name>`, `## Tree` and the
opening fence of a `text tree` block in line 4 followed by its own line `src/<x>.rs`; after the block `Status: done`
with a closing fence before it and `end` after it: in line 7 of `a.md`, `b.md` and `c.md` (`c.md` has `end.`), in
line 9 of `d.md`, where the three lines are indented by two spaces after a line `- item`.

## E149.1 An anchor line with different context

Fixture: `repeated-context`

The fence is in all the files, its context differs: every file has it with its context.

```json request
{"query": "text has '```text'", "output": {"context": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

./
a.md  lines: 1  matches: 1
3-## Tree
4:⟦```text⟧ tree
5-src/a.rs
b.md  lines: 1  matches: 1
3-## Tree
4:⟦```text⟧ tree
5-src/b.rs
c.md  lines: 1  matches: 1
3-## Tree
4:⟦```text⟧ tree
5-src/c.rs
d.md  lines: 1  matches: 1
3-## Tree
4:⟦```text⟧ tree
5-src/d.rs
```

## E149.2 The same context in three files

Fixture: `repeated-context`

The block of `d.md` is the same without its common indentation; `c.md` has `end.` after the line and keeps it.

```json request
{"query": "text has 'Status:'", "output": {"context": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

repeated:
  ×3 a.md:7, b.md:7, d.md:9
    -```
    :⟦Status:⟧ done
    -end
./
c.md  lines: 1  matches: 1
6-```
7:⟦Status:⟧ done
8-end.
```

## E149.3 Before only

Fixture: `repeated-context`

The line before is the same in all four files.

```json request
{"query": "text has 'Status:'", "output": {"before": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

repeated:
  ×4 a.md:7, b.md:7, c.md:7, … 1 more
    -```
    :⟦Status:⟧ done
```

## E149.4 After only

Fixture: `repeated-context`

1. One line after: `c.md` differs

```json request
{"query": "text has 'Status:'", "output": {"after": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

repeated:
  ×3 a.md:7, b.md:7, d.md:9
    :⟦Status:⟧ done
    -end
./
c.md  lines: 1  matches: 1
7:⟦Status:⟧ done
8-end.
```

2. `after = "block"`: the line opens no block, so its block is the line alone, written as a repeated line without
   context

```json request
{"query": "text has 'Status:'", "output": {"after": "block", "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r2

repeated:
  ×4 ⟦Status:⟧ done    a.md:7, b.md:7, c.md:7, … 1 more
```

## E149.5 Without context, unchanged

Fixture: `repeated-context`

```json request
{"query": "text has 'Status:'", "output": {"context": 0, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

repeated:
  ×4 ⟦Status:⟧ done    a.md:7, b.md:7, c.md:7, … 1 more
```

## E149.6 Spec 1, unchanged

Fixture: `repeated-context`

Spec 1 groups the line and leaves its context out.

```json request
{"version": 1, "query": "text has 'Status:'", "output": {"context": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1
#1 text has 'Status:'

repeated:
  ×4 ⟦#1:Status:⟧ done    a.md:7, b.md:7, c.md:7, … 1 more
```

## E149.7 Another matched line in the block

Fixture: `repeated-context`

`Status` and `end` are next to each other: the block of each has the other one, so neither is grouped.

```json request
{"query": "text =~ `Status|end`", "output": {"context": 1, "sort": "path asc"}}
```

```text response
files: 4  lines: 8  matches: 8  result: r1

./
a.md  lines: 2  matches: 2
6-```
7:⟦Status⟧: done
8:⟦end⟧
b.md  lines: 2  matches: 2
6-```
7:⟦Status⟧: done
8:⟦end⟧
c.md  lines: 2  matches: 2
6-```
7:⟦Status⟧: done
8:⟦end⟧.
d.md  lines: 2  matches: 2
⇥2
8-```
9:⟦Status⟧: done
10:⟦end⟧
```

## E149.8 Columns and the locations left out

Fixture: `repeated-context`

```json request
{"query": "text has 'Status:'", "output": {"before": 1, "highlight": "columns", "omit": ["locations"], "sort": "path asc"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1

repeated:
  ×4
    -```
    :Status: done
      @ 1..8
```

## E149.9 In JSON

Fixture: `repeated-context`

```json request
{"query": "text has 'Status:'", "output": {"context": 1, "format": "json", "sort": "path asc"}}
```

```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "repeated": [
    { "count": 3, "before": ["```"], "text": "Status: done",
      "fragments": [ { "col": [1, 8], "text": "Status:" } ],
      "after": ["end"],
      "locations": ["a.md:7", "b.md:7", "d.md:9"] }
  ],
  "counts": { "files": 4, "lines": 4, "matches": 4 },
  "files": [
    {
      "path": "c.md",
      "lines": 1,
      "matches": 1,
      "blocks": [
        { "lines": [
            { "n": 6, "text": "```" },
            { "n": 7, "text": "Status: done",
              "fragments": [ { "col": [1, 8], "text": "Status:" } ] },
            { "n": 8, "text": "end." } ] }
      ]
    }
  ]
}
```
