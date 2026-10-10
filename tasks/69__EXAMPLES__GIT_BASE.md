# Examples for task 69

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### gitbase

```text tree
repo/a.txt                 2026-10-01 10:00
repo/b.txt                 2026-10-01 10:00
repo/c.txt                 2026-10-01 10:00
```

```text setup
file repo/a.txt 1
file repo/b.txt 1
git repo init -q -b main
git repo add a.txt b.txt
git repo commit -q -m init
git repo checkout -q -b develop
file repo/a.txt 2
git repo commit -q -a -m change-a
git repo checkout -q -b feature
file repo/b.txt 2
git repo commit -q -a -m change-b
file repo/c.txt 1
```

The repository is on the branch `feature`, made from `develop`, made from `main`: `a.txt` was changed by a commit of
`develop`, `b.txt` by a commit of `feature`, `c.txt` is untracked. Times are UTC.

## E69.1 `auto` by default: `main` here

Fixture: `gitbase`

1. `find`

```json request
{"query": "git = branch"}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
a.txt  [#1]
b.txt  [#1]
c.txt  [#1]
```

2. `get_config`

```json request get_config
{ "section": "git" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[git]
base = "auto"
```

## E69.2 Another base in a request

Fixture: `gitbase`

```json request
{"query": "git = branch", "git": {"base": "develop"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
b.txt  [#1]
c.txt  [#1]
```

## E69.3 A base which doesn't exist

`git = branch` is `changed` in the repository, and its files have a warning.

Fixture: `gitbase`

```json request
{"query": "git = branch", "git": {"base": "release"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
c.txt  [#1]

warning: git_base_not_found (3): repo/a.txt, repo/b.txt, repo/c.txt
```

## E69.4 A base for the session

Fixture: `gitbase`

1. `update_config`

```json request update_config
{"set": "git.base = \"develop\""}
```

```text response
[git]
base = "develop"    # was "auto"
```

2. `find`

```json request
{"query": "git = branch"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = branch

repo/
b.txt  [#1]
c.txt  [#1]
```

3. `get_config`

```json request get_config
{ "section": "git", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[git]
base = "develop"    # update
```

## E69.5 An invalid value in a request

A name which starts with `-` is not a base.

Fixture: `gitbase`

```json request
{"git": {"base": "--all"}}
```

```text error
error[invalid_option]: 'git.base' must be "auto" or a branch, tag or commit
```

## E69.6 An invalid value for `update_config`

Fixture: `gitbase`

```json request update_config
{"set": "git.base = \"\""}
```

```text error
error[invalid_value]: invalid value for 'git.base'
  hint: "auto" or a branch, tag or commit is expected
```

## E69.7 From the config file

Fixture: `gitbase`

```toml conf/g.toml
[git]
base = "develop"
```

```text command
easy-peasy-mcp --config conf/g.toml
```

1. `get_config`

```json request get_config
{ "section": "git", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[git]
base = "develop"    # file: conf/g.toml
```

2. `find`

```json request
{"query": "git = branch and name ~ '*.txt'"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 git = branch
#2 name ~ '*.txt'

repo/
b.txt  [#1 #2]
c.txt  [#1 #2]

warning: not_a_git_repo (1): conf/g.toml
```

## E69.8 An invalid value in the config file

Fixture: `gitbase`

```toml conf/g.toml
[git]
base = ["main"]
```

```text command
easy-peasy-mcp --config conf/g.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/g.toml:2: invalid value for 'git.base': "auto" or a branch, tag or commit is expected
```
