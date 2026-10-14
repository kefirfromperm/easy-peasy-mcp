# Examples for task 2

Spec: `1`

## Fixtures

### flat

```text tree
.cache/x.md                2026-10-01 10:00
.gitignore                 2026-10-01 10:00
.hidden.md                 2026-10-01 10:00
build/out.md               2026-10-01 10:00
ignored.md                 2026-10-01 10:00
notes/.draft.md            2026-10-01 10:00
notes/a.md                 2026-10-01 10:00
notes/b.md                 2026-10-01 10:00
notes/c.md                 2026-10-01 10:00
notes/index.md             2026-10-01 10:00
notes/package-lock.json    2026-10-01 10:00
```

File contents are in `tests/fixtures/flat/`. Times are UTC. What the defaults skip:

- hidden: `.cache/`, `.gitignore`, `.hidden.md`, `notes/.draft.md`
- `.gitignore` (`ignored.md`, `build/`): `ignored.md`, `build/out.md`
- generated: `notes/package-lock.json` (`**/package-lock.json`), `notes/index.md` (the marker `DO NOT EDIT` in the
  first line)

The files left are all documentation files at the same depth with the same modification time.

## E2.1 No `query` field

Fixture: `flat`

```json request
{}
```

```text response
files: 3  lines: 0  matches: 0  result: r1

notes/
a.md
b.md
c.md
```

## E2.2 Empty string

Fixture: `flat`

```json request
{ "query": "" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1

notes/
a.md
b.md
c.md
```

## E2.3 Only whitespace and line breaks

Fixture: `flat`

```json request
{ "query": "  \n\t  " }
```

```text response
files: 3  lines: 0  matches: 0  result: r1

notes/
a.md
b.md
c.md
```
