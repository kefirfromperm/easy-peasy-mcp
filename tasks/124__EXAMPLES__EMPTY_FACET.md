# Examples for task 124

Spec: `1`

A JSON response is compared as a JSON value. The examples use the fixture `huge` of task 104: eleven files — six in
`vendor/lib/`, two in `src/`, two tests in `tests/` (`.rs`), `docs/d1.md`. The fixture is not a git repository.

## E124.1 The empty query has no `condition` line

The empty query has no conditions, so the `condition` facet of the default facets has no values and no line. The other
facets keep their lines and their order.

Fixture: `huge`

```json request
{"query": "", "output": {"detail": "facets"}}
```

```text response
files: 11  lines: 0  matches: 0  result: r1
dir:        vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
lang:       rust 10 · markdown 1
kind:       source 8 · test 2 · doc 1
```

## E124.2 `git` outside of a repository

No file is in a repository, so `git` has no values and no line; `ext` after it is written as usual. A `git` facet is no
`git` condition: there is no `not_a_git_repo` warning.

Fixture: `huge`

```json request
{"query": "name ~ '*.md'", "output": {"detail": "facets", "facets": ["git", "ext"]}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.md'
ext:        md 1
```

## E124.3 The empty query in JSON

The facet without values stays as an empty object, so the keys of `facets` are those of `output.facets`.

Fixture: `huge`

```json request
{"query": "", "output": {"detail": "facets", "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "facets": {
    "dir": { "vendor/lib/": 6, "src/": 2, "tests/": 2, "docs/": 1 },
    "lang": { "rust": 10, "markdown": 1 },
    "kind": { "source": 8, "test": 2, "doc": 1 },
    "condition": {}
  },
  "counts": { "files": 11, "lines": 0, "matches": 0 }
}
```

## E124.4 `git` outside of a repository in JSON

Fixture: `huge`

```json request
{"query": "name ~ '*.md'", "output": {"detail": "facets", "facets": ["git", "ext"], "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "legend": { "#1": "name ~ '*.md'" },
  "facets": {
    "git": {},
    "ext": { "md": 1 }
  },
  "counts": { "files": 1, "lines": 0, "matches": 0 }
}
```
