# Examples for task 17

## Fixtures

### gitrepo

```text tree
outside/loose.txt          2026-10-01 10:00
repo/both.txt              2026-10-01 10:00
repo/clean.txt             2026-10-01 10:00
repo/conflict.txt          2026-10-01 10:00
repo/modified.txt          2026-10-01 10:00
repo/staged.txt            2026-10-01 10:00
repo/untracked.txt         2026-10-01 10:00
```

```text setup
file repo/clean.txt 1
file repo/modified.txt 1
file repo/staged.txt 1
file repo/both.txt 1
file repo/conflict.txt 1
git repo init -q -b main
git repo add .
git repo commit -q -m init
git repo checkout -q -b other
file repo/conflict.txt 2
git repo commit -q -a -m other
git repo checkout -q main
file repo/conflict.txt 3
git repo commit -q -a -m main
git! repo merge -q other
file repo/modified.txt 2
file repo/staged.txt 2
git repo add staged.txt
file repo/both.txt 2
git repo add both.txt
file repo/both.txt 3
file repo/untracked.txt 1
file outside/loose.txt 1
```

Everything is built by the test setup: the repository `repo/` is in the middle of a merge with a conflict in
`conflict.txt`; `modified.txt` has unstaged changes, `staged.txt` staged ones, `both.txt` both, `untracked.txt` is
not tracked, `clean.txt` has no changes. `outside/` is not in a repository. Times are UTC.

## E17.1 `modified`: unstaged changes

Every example has the warning: `outside/loose.txt` is evaluated, but it is not in a repository, so every `git`
condition is false for it.

Fixture: `gitrepo`

```json request
{ "query": "git = modified" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = modified

repo/
both.txt  [#1]
modified.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.2 `staged`

Fixture: `gitrepo`

```json request
{ "query": "git = staged" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = staged

repo/
both.txt  [#1]
staged.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.3 `untracked`

Fixture: `gitrepo`

```json request
{ "query": "git = untracked" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 git = untracked

repo/
untracked.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.4 `conflicted`

Fixture: `gitrepo`

```json request
{ "query": "git = conflicted" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 git = conflicted

repo/
conflict.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.5 `changed`: any of the above

Fixture: `gitrepo`

```json request
{ "query": "git = changed" }
```

```text response
files: 5  lines: 0  matches: 0  result: r1
#1 git = changed

repo/
both.txt  [#1]
conflict.txt  [#1]
modified.txt  [#1]
staged.txt  [#1]
untracked.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.6 `clean`

Fixture: `gitrepo`

```json request
{ "query": "git = clean" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 git = clean

repo/
clean.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.7 `in ( ... )`

Fixture: `gitrepo`

```json request
{ "query": "GIT in (Modified, untracked)" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 GIT in (Modified, untracked)

repo/
both.txt  [#1]
modified.txt  [#1]
untracked.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.8 `!=` is false outside of a repository

Fixture: `gitrepo`

```json request
{ "query": "git != clean" }
```

```text response
files: 5  lines: 0  matches: 0  result: r1
#1 git != clean

repo/
both.txt  [#1]
conflict.txt  [#1]
modified.txt  [#1]
staged.txt  [#1]
untracked.txt  [#1]

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.9 `not` of a false condition

The files match only through `not`, so no condition is listed.

Fixture: `gitrepo`

```json request
{ "query": "not git = changed" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = changed

outside/
loose.txt
repo/
clean.txt

warning: not_a_git_repo (1): outside/loose.txt
```

## E17.10 Unknown value

Fixture: `gitrepo`

```json request
{ "query": "git = dirty" }
```

```text error
error[unknown_value]: unknown value 'dirty'
  git = dirty
        ^^^^^
  hint: expected one of: modified, staged, untracked, conflicted, changed, branch, clean
```
