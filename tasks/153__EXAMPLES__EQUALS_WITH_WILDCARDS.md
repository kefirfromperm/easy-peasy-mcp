# Examples for task 153

A JSON response is compared as a JSON value. `name =` and `path =` compare the whole value; when such a condition
with a wildcard character (`*`, `?`, `[`, `{`) leaves the result empty, the hints try the same condition with `~`
(OUTPUT.md#hints).

## Fixtures

### equals-wildcards

```text tree
README.md                  2026-10-01 10:00
docs/guide.md              2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

In `tests/fixtures/equals-wildcards/`. Times are UTC.

### equals-wildcards-literal

```text tree
src/*.rs                   2026-10-01 10:00
src/main.rs                2026-10-01 10:00
```

```text setup
file src/*.rs 3
file src/main.rs 3
```

A file is really named `*.rs`. It is created by the setup: such a name can't be checked out on Windows.

## E153.1 `name =` with a wildcard pattern

No file is named `*.rs`; `name ~ '*.rs'` finds two. The variations `/i` and `has` match nothing, and the similar
names are left out for a value with a wildcard character.

Fixture: `equals-wildcards`

```json request
{"query": "name = '*.rs'"}
```

```text response
files: 0  lines: 0  result: r1
funnel:
  name = '*.rs'                          0
hints:
  name ~ '*.rs'                          2 files: lib.rs, main.rs
```

## E153.2 `path =` with a wildcard pattern

Fixture: `equals-wildcards`

```json request
{"query": "path = 'src/**'"}
```

```text response
files: 0  lines: 0  result: r1
funnel:
  path = 'src/**'                        0
hints:
  path ~ 'src/**'                        2 files: src/lib.rs, src/main.rs
```

## E153.3 No hint when `~` matches nothing

There is no Python file either: the funnel comes without hints.

Fixture: `equals-wildcards`

```json request
{"query": "name = '*.py'"}
```

```text response
files: 0  lines: 0  result: r1
funnel:
  name = '*.py'                          0
```

## E153.4 A file really named with the wildcard characters

`=` compares the whole name, so it finds the file `*.rs` and nothing else; the result isn't empty, there are no hints.

Fixture: `equals-wildcards-literal`

Platform: `linux`

```json request
{"query": "name = '*.rs'", "output": {"detail": "files"}}
```

```text response
files: 1  lines: 0  result: r1

src/*.rs
```

## E153.5 Inside a larger query, with a modifier

The variation is searched with the steps before it and keeps the modifiers of the condition: only `docs/guide.md`
passes `path ~ 'docs/**'` and `name ~ '*.MD' /i`.

Fixture: `equals-wildcards`

```json request
{"query": "path ~ 'docs/**' and name = '*.MD' /i"}
```

```text response
files: 0  lines: 0  result: r1
funnel:
  path ~ 'docs/**'                       1
  and name = '*.MD' /i                   0
hints:
  name ~ '*.MD' /i                       1 file: guide.md
```

## E153.6 Without the hints

Fixture: `equals-wildcards`

```json request
{"query": "name = '*.rs'", "output": {"omit": ["hints"]}}
```

```text response
files: 0  lines: 0  result: r1
funnel:
  name = '*.rs'                          0
```

## E153.7 In JSON

Fixture: `equals-wildcards`

```json request
{"query": "name = '*.rs'", "output": {"format": "json"}}
```

```json response
{
  "version": 2,
  "exists": false,
  "result": "r1",
  "funnel": [
    { "step": "name = '*.rs'", "files": 0 }
  ],
  "hints": [
    "name ~ '*.rs'  2 files: lib.rs, main.rs"
  ],
  "counts": { "files": 0, "lines": 0, "matches": 0 }
}
```

## E153.8 Spec version 1

A new hint doesn't break spec version 1 (TOOLS.md#versioning): a request pinned to it gets the hint too.

Fixture: `equals-wildcards`

```json request
{"query": "name = '*.rs'", "version": 1}
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name = '*.rs'                          0
hints:
  name ~ '*.rs'                          2 files: lib.rs, main.rs
```
