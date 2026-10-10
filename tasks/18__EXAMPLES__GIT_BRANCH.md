# Examples for task 18

## Fixtures

### gitfeature

```text tree
repo/a.txt                 2026-10-01 10:00
repo/b.txt                 2026-10-01 10:00
repo/c.txt                 2026-10-01 10:00
repo/e.txt                 2026-10-01 10:00
```

```text setup
file repo/a.txt 1
file repo/b.txt 1
file repo/c.txt 1
git repo init -q -b main
git repo add .
git repo commit -q -m init
git repo checkout -q -b feature
file repo/a.txt 2
git repo commit -q -a -m change-a
file repo/b.txt 2
file repo/e.txt 1
```

The repository is on the branch `feature`, made from `main`: `a.txt` was changed by a commit of the branch,
`b.txt` has uncommitted changes, `e.txt` is untracked, `c.txt` is unchanged. The base `auto` is `main` (there is no
`origin`).

### gitmain

```text tree
repo/a.txt                 2026-10-01 10:00
repo/b.txt                 2026-10-01 10:00
repo/c.txt                 2026-10-01 10:00
```

```text setup
file repo/a.txt 1
file repo/b.txt 1
file repo/c.txt 1
git repo init -q -b main
git repo add .
git repo commit -q -m init
file repo/a.txt 2
git repo commit -q -a -m change-a
file repo/b.txt 2
```

The same changes, but on `main` itself: the commit which changed `a.txt` is on the base branch.

## E18.1 `branch`: changed by the commits of the branch or not committed

Fixture: `gitfeature`

```json request
{ "query": "git = branch" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
a.txt  [#1]
b.txt  [#1]
e.txt  [#1]
```

## E18.2 `changed` doesn't see the commits

Fixture: `gitfeature`

```json request
{ "query": "git = changed" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = changed

repo/
b.txt  [#1]
e.txt  [#1]
```

## E18.3 A file changed by a commit of the branch is clean

Fixture: `gitfeature`

```json request
{ "query": "git = clean" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = clean

repo/
a.txt  [#1]
c.txt  [#1]
```

## E18.4 On the base branch `branch` is `changed`

Fixture: `gitmain`

```json request
{ "query": "git = branch" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
b.txt  [#1]
```
