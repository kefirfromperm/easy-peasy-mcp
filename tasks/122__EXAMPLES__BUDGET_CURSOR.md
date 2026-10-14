# Examples for task 122

Spec: `1`

A JSON response is compared as a JSON value. The examples use the fixture `spec-output-many` of task 121: 37 Rust
files under `src/`; with `sort = "path asc"` they are listed in path order. The budget lists the files which fit at
level 4 (`detail = files`) and the next cursor starts at the first file not listed, so the pages list every file once.
The cursor is opaque: the examples show the cursors this version makes.

## E122.1 A budget list cut before `max_files` reaches the end

14 of the 37 files fit in 110 tokens; `max_files` (default 100) reaches past the end of the list, the budget cut it, so
the response has a next cursor. The pages list 14, 14 and 9 files.

Fixture: `spec-output-many`

1. `find`

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110}}
```

```text response
files: 37  lines: 0  matches: 0  result: r1  detail: files (budget 110)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiYmE4MGZlZWQiLCJmIjoxNCwidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/api/
admin.rs  [#1]
health.rs  [#1]
items.rs  [#1]
orders.rs  [#1]
search.rs  [#1]
users.rs  [#1]
src/
config.rs  [#1]
src/db/
items.rs  [#1]
migrate.rs  [#1]
orders.rs  [#1]
pool.rs  [#1]
schema.rs  [#1]
users.rs  [#1]
src/eval/
errors.rs  [#1]
```

2. `find` — the same request with the cursor

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110, "cursor": "eyJ2IjoxLCJoIjoiYmE4MGZlZWQiLCJmIjoxNCwidCI6WzM3LDAsMF19"}}
```

```text response
files: 37  lines: 0  matches: 0  result: r2  detail: files (budget 110)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiYmE4MGZlZWQiLCJmIjoyOCwidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/eval/
mod.rs  [#1]
ops.rs  [#1]
scope.rs  [#1]
types.rs  [#1]
value.rs  [#1]
src/jobs/
cleanup.rs  [#1]
export.rs  [#1]
import.rs  [#1]
mail.rs  [#1]
report.rs  [#1]
sync.rs  [#1]
src/
main.rs  [#1]
src/net/
client.rs  [#1]
dns.rs  [#1]
```

3. `find` — the last page fits, no cursor

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110, "cursor": "eyJ2IjoxLCJoIjoiYmE4MGZlZWQiLCJmIjoyOCwidCI6WzM3LDAsMF19"}}
```

```text response
files: 37  lines: 0  matches: 0  result: r3
#1 name ~ '*.rs'

src/net/
proxy.rs  [#1]
retry.rs  [#1]
server.rs  [#1]
tls.rs  [#1]
src/store/
blobs.rs  [#1]
cache.rs  [#1]
files.rs  [#1]
index.rs  [#1]
queue.rs  [#1]
```

## E122.2 A budget list cut inside `max_files`

13 files fit, fewer than `max_files = 30`: the next page starts at the 14th file, `src/eval/errors.rs`, right after the
last file listed, not at the 31st. The second page has 24 files left, within `max_files`, and is cut by the budget only.

Fixture: `spec-output-many`

1. `find`

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110, "max_files": 30}}
```

```text response
files: 37  lines: 0  matches: 0  result: r1  detail: files (budget 110)  truncated: yes (max_files, budget)  next cursor: eyJ2IjoxLCJoIjoiZmY1YzRjNjgiLCJmIjoxMywidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/api/
admin.rs  [#1]
health.rs  [#1]
items.rs  [#1]
orders.rs  [#1]
search.rs  [#1]
users.rs  [#1]
src/
config.rs  [#1]
src/db/
items.rs  [#1]
migrate.rs  [#1]
orders.rs  [#1]
pool.rs  [#1]
schema.rs  [#1]
users.rs  [#1]
```

2. `find` — the same request with the cursor

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110, "max_files": 30, "cursor": "eyJ2IjoxLCJoIjoiZmY1YzRjNjgiLCJmIjoxMywidCI6WzM3LDAsMF19"}}
```

```text response
files: 37  lines: 0  matches: 0  result: r2  detail: files (budget 110)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiZmY1YzRjNjgiLCJmIjoyNywidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/eval/
errors.rs  [#1]
mod.rs  [#1]
ops.rs  [#1]
scope.rs  [#1]
types.rs  [#1]
value.rs  [#1]
src/jobs/
cleanup.rs  [#1]
export.rs  [#1]
import.rs  [#1]
mail.rs  [#1]
report.rs  [#1]
sync.rs  [#1]
src/
main.rs  [#1]
src/net/
client.rs  [#1]
```

## E122.3 In JSON

19 files fit in 300 tokens; the rest, 18 files, fit on the second page.

Fixture: `spec-output-many`

1. `find`

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 300, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "detail_used": "files",
  "counts": { "files": 37, "lines": 0, "matches": 0 },
  "truncated": true,
  "truncated_by": ["budget"],
  "next_cursor": "eyJ2IjoxLCJoIjoiNWE3NGFkMDkiLCJmIjoxOSwidCI6WzM3LDAsMF19",
  "legend": { "#1": "name ~ '*.rs'" },
  "files": [
    { "path": "src/api/admin.rs", "conditions": ["#1"] },
    { "path": "src/api/health.rs", "conditions": ["#1"] },
    { "path": "src/api/items.rs", "conditions": ["#1"] },
    { "path": "src/api/orders.rs", "conditions": ["#1"] },
    { "path": "src/api/search.rs", "conditions": ["#1"] },
    { "path": "src/api/users.rs", "conditions": ["#1"] },
    { "path": "src/config.rs", "conditions": ["#1"] },
    { "path": "src/db/items.rs", "conditions": ["#1"] },
    { "path": "src/db/migrate.rs", "conditions": ["#1"] },
    { "path": "src/db/orders.rs", "conditions": ["#1"] },
    { "path": "src/db/pool.rs", "conditions": ["#1"] },
    { "path": "src/db/schema.rs", "conditions": ["#1"] },
    { "path": "src/db/users.rs", "conditions": ["#1"] },
    { "path": "src/eval/errors.rs", "conditions": ["#1"] },
    { "path": "src/eval/mod.rs", "conditions": ["#1"] },
    { "path": "src/eval/ops.rs", "conditions": ["#1"] },
    { "path": "src/eval/scope.rs", "conditions": ["#1"] },
    { "path": "src/eval/types.rs", "conditions": ["#1"] },
    { "path": "src/eval/value.rs", "conditions": ["#1"] }
  ]
}
```

2. `find` — the same request with the cursor; the last page fits, no cursor

```json request
{"query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 300, "format": "json", "cursor": "eyJ2IjoxLCJoIjoiNWE3NGFkMDkiLCJmIjoxOSwidCI6WzM3LDAsMF19"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r2",
  "counts": { "files": 37, "lines": 0, "matches": 0 },
  "legend": { "#1": "name ~ '*.rs'" },
  "files": [
    { "path": "src/jobs/cleanup.rs", "conditions": ["#1"] },
    { "path": "src/jobs/export.rs", "conditions": ["#1"] },
    { "path": "src/jobs/import.rs", "conditions": ["#1"] },
    { "path": "src/jobs/mail.rs", "conditions": ["#1"] },
    { "path": "src/jobs/report.rs", "conditions": ["#1"] },
    { "path": "src/jobs/sync.rs", "conditions": ["#1"] },
    { "path": "src/main.rs", "conditions": ["#1"] },
    { "path": "src/net/client.rs", "conditions": ["#1"] },
    { "path": "src/net/dns.rs", "conditions": ["#1"] },
    { "path": "src/net/proxy.rs", "conditions": ["#1"] },
    { "path": "src/net/retry.rs", "conditions": ["#1"] },
    { "path": "src/net/server.rs", "conditions": ["#1"] },
    { "path": "src/net/tls.rs", "conditions": ["#1"] },
    { "path": "src/store/blobs.rs", "conditions": ["#1"] },
    { "path": "src/store/cache.rs", "conditions": ["#1"] },
    { "path": "src/store/files.rs", "conditions": ["#1"] },
    { "path": "src/store/index.rs", "conditions": ["#1"] },
    { "path": "src/store/queue.rs", "conditions": ["#1"] }
  ]
}
```

## E122.4 In a section of `queries`

The cursor of a section is passed back in the `output` of that section.

Fixture: `spec-output-many`

1. `find`

```json request
{"queries": [{"label": "rs", "query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110}}]}
```

```text response
== rs ==  files: 37  lines: 0  matches: 0  result: r1  detail: files (budget 110)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiYjFiOTUzOWYiLCJmIjoxNCwidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/api/
admin.rs  [#1]
health.rs  [#1]
items.rs  [#1]
orders.rs  [#1]
search.rs  [#1]
users.rs  [#1]
src/
config.rs  [#1]
src/db/
items.rs  [#1]
migrate.rs  [#1]
orders.rs  [#1]
pool.rs  [#1]
schema.rs  [#1]
users.rs  [#1]
src/eval/
errors.rs  [#1]
```

2. `find` — the same request with the cursor in the section

```json request
{"queries": [{"label": "rs", "query": "name ~ '*.rs'", "output": {"sort": "path asc", "budget": 110, "cursor": "eyJ2IjoxLCJoIjoiYjFiOTUzOWYiLCJmIjoxNCwidCI6WzM3LDAsMF19"}}]}
```

```text response
== rs ==  files: 37  lines: 0  matches: 0  result: r2  detail: files (budget 110)  truncated: yes (budget)  next cursor: eyJ2IjoxLCJoIjoiYjFiOTUzOWYiLCJmIjoyOCwidCI6WzM3LDAsMF19
#1 name ~ '*.rs'

src/eval/
mod.rs  [#1]
ops.rs  [#1]
scope.rs  [#1]
types.rs  [#1]
value.rs  [#1]
src/jobs/
cleanup.rs  [#1]
export.rs  [#1]
import.rs  [#1]
mail.rs  [#1]
report.rs  [#1]
sync.rs  [#1]
src/
main.rs  [#1]
src/net/
client.rs  [#1]
dns.rs  [#1]
```
