# AGENTS.md

Instructions for AI agents and humans working on **easy-peasy-mcp**, a local stdio MCP server in Rust that finds files
and places in files with one query (`find` + `grep` and more).

## Source of truth

The specification is the contract. Code, tests and examples follow it, never the other way round.

| File                     | Covers                                                                    |
|--------------------------|---------------------------------------------------------------------------|
| [README.md](README.md)   | overview, request fields, a first example                                 |
| [QUERY.md](QUERY.md)     | query language: targets, operators, scopes, modifiers, grammar, errors   |
| [OUTPUT.md](OUTPUT.md)   | response format, detail levels, fragments, limits, funnel, result ids    |
| [CONFIG.md](CONFIG.md)   | config file, command line, defaults, traversal, encodings, languages     |
| [TOOLS.md](TOOLS.md)     | MCP transport, tool schemas, results, errors, timeout, progress, cancel  |
| `tasks/`                 | the development plan, one task at a time, see [Workflow](#workflow)      |

- If the spec is ambiguous or contradicts itself, fix the spec first, in its own commit, then continue.
- Never implement behavior the spec doesn't describe. If a task needs something new, add it to the spec first.
- Follow the spec's own versioning rules (TOOLS.md#versioning): spec version `1` is not broken by any task.
- All output is optional. Every part of a `find` response can be left out with `output.omit` and brought back with
  `output.show` (OUTPUT.md#omit), and whether it is shown by default comes from the configuration (`output.omit` in
  CONFIG.md), not from the code. A task or spec change that adds anything to the response, in text or JSON, names its
  `omit` part, adds it to the table in OUTPUT.md#omit, sets its default and covers it in the examples.

## Agent tools

When the **easy-peasy** MCP server (this project's own server) is registered in the agent's client, use its tools
first, wherever they can do the job, instead of shell pipelines:

| Job                                              | easy-peasy tool | Instead of                                   |
|--------------------------------------------------|-----------------|----------------------------------------------|
| find files, find text in files, count matches    | `find`          | `grep`, `rg`, `find`, `git grep`, `ls -R`    |
| replace text in one or many files                | `replace`       | `sed -i`, `grep -l … \| xargs sed`, `perl -pi` |
| rename or move files by a pattern                | `rename`        | `mv` loops, `git mv` scripts                 |
| copy files                                       | `copy`          | `cp` loops                                   |
| delete files (to the trash)                      | `delete`        | `rm`, `find … -delete`                       |

- call `get_config` once per session (with a `section`, the whole config is large); its header names the server
  version, which can lag the version in `Cargo.toml`
- `replace`, `rename`, `copy` and `delete` on a selection you haven't seen: first with `dry_run = true`, then for real
  with `expect` (files) or `expect_matches`, so a selection that changed in between fails instead of doing the wrong
  thing; a `find` result id (`result = "r12"`) reuses exactly the files you looked at
- a single known edit in one file is still a plain editor edit; easy-peasy is for edits selected by a query
- check the effect of a write tool with `git status` / `git diff`
- fall back to the shell tools when easy-peasy can't express the request or isn't available; a case it should handle
  but doesn't is a bug or a missing feature, worth a task in `tasks/`
- create such a task right away, without waiting to be asked, see [New tasks](#new-tasks): a bug, a missing feature,
  and also a response that could carry the same information in fewer tokens (a repeated number, a redundant line or
  path, a part the agent never uses) or a request that takes more calls or characters than it should

## Workflow

Development goes task by task. Every task lives in `tasks/` as a set of numbered files. `N` is the task number
(`1`, `2`, ...), `NAME` is a short UPPER_SNAKE_CASE name (`MVP`, `LINE_SCOPE`, `FUNNEL`).

| Step | File                              | Written when                       | Contains                                         |
|------|-----------------------------------|------------------------------------|--------------------------------------------------|
| 1    | `N__TASK__NAME.md`                | the task is defined                | goal, scope, out of scope, acceptance criteria, status |
| 2    | `N__EXAMPLES__NAME.md`            | before any code                    | requests and exact responses; the test data      |
| 3    | `N__PLAN__NAME.md`                | coding starts                      | design, modules, steps, test list, risks         |
| 4    | tests, then code                  | —                                  | red → green → refactor                           |
| 5    | `N__SUMMARY__NAME.md`             | the task is finished               | detailed description of what was built           |

Example for the first task:

```
tasks/
  1__TASK__MVP.md
  1__EXAMPLES__MVP.md
  1__PLAN__MVP.md
  1__SUMMARY__MVP.md
```

Do the steps in order. Don't write the plan before the examples are agreed, don't write code before the plan, don't
write production code before a failing test for it. Every step is committed separately on the task branch, see
[Git](#git).

### 1. Task — `N__TASK__NAME.md`

```markdown
# Task N: <title>

Status: not implemented

## Goal
What the user (an agent calling the server) can do after this task that they couldn't before.

## Scope
- spec features included, with links to the spec sections (QUERY.md#line-scope, ...)

## Out of scope
- features explicitly left for later tasks

## Acceptance criteria
- every example in N__EXAMPLES__NAME.md passes as a test
- all tests of earlier tasks still pass
- the release binary works when registered in an MCP client
```

`Status` is the first thing a reader sees and it always tells the truth; the Status column of
[tasks/README.md](tasks/README.md) is changed in the same commit:

| Status            | Set when                                              |
|-------------------|-------------------------------------------------------|
| `not implemented` | the task file is created                              |
| `in progress`     | the plan is written and coding started                |
| `implemented`     | all acceptance criteria are met; links the SUMMARY file |

### 2. Examples — `N__EXAMPLES__NAME.md`

Examples are the base for the test data. Tests are generated from them, so they must be exact.

**Every example must be correct for the whole application**, not only for this task. An example shows what the
finished server, with the full spec implemented, answers to that request. Therefore:

- use only the request fields, query syntax and output options the task implements, so the current server can
  answer it; never show a response that a later task would change
- write the full response exactly as the spec defines it: headers, legend, condition ids, markers, counts, result id,
  warnings — no "simplified for now" output
- the defaults are the spec defaults; if a later task adds a feature that changes the default output (e.g. a
  new header field), the example is wrong and the spec or the example must be fixed, not ignored
- examples from earlier tasks stay valid forever; they run as regression tests in every later task
- an example that turns out to be wrong is fixed in the examples file, with the reason in the commit message

Format, so tests can read the file directly:

````markdown
# Examples for task N

## Fixtures

### small-rust
```text tree
src/main.rs        2026-10-01 10:00
src/config.rs      2026-10-02 09:10
README.md          2026-10-01 10:00
```
File contents are in `tests/fixtures/small-rust/`.

## E1.1 Find files by name

Fixture: `small-rust`

```json request
{ "query": "name ~ '*.rs'", "output": { "detail": "files" } }
```

```text response
<the exact text of the response, byte for byte as the spec defines it>
```

## E1.2 Unknown operator

Fixture: `small-rust`

```json request
{ "query": "text contains 'x'" }
```

```text error
<the exact text of the error, byte for byte as the spec defines it>
```
````

- ids are `E<task>.<n>`, unique and never reused; a test refers to an example by its id
- `json request` — the `arguments` of a `tools/call` of `find` (or another tool, written as
  `json request get_config`)
- `text response` — the exact text content item, `isError: false`; `text error` — the same with `isError: true`;
  `json response` and `json error` — when `output.format = "json"`: the text content item is a JSON document, compared
  with the block as a JSON value, so the block may be indented for reading
- a fixture is a directory under `tests/fixtures/<name>/`; anything that can't live in git (mtimes, symlinks,
  permissions, empty dirs, encodings with odd bytes) is listed in the fixture section and created by the test setup: a `text setup`
  block in the fixture section, one action per line — `mkdir <dir>`, `file <path> [<bytes>]` (a file of `x` bytes, empty without a size),
  `symlink <path> -> <target>` (the target relative to the link's directory), `atime <path> <date> <time>` (UTC, set
  after the times of the tree), `bytes <path> <hex>` (a file of these bytes), `git <dir> <args>` (runs `git -C <dir> <args>` with a fixed identity and no user
  configuration; `git!` ignores a failure, e.g. a merge with conflicts), `chmod <path> <mode>` (octal Unix
  permissions, set after the times; an example of a fixture with `chmod` is skipped where permissions can't be
  removed: on Windows, or when the entry stays readable, e.g. running as root), `hide <path>` (the `hidden` flag on
  macOS, the hidden attribute on Windows; nothing on Linux, which has neither), `junction <path> -> <target>` (a
  junction on Windows, a symlink to the directory elsewhere)
- the server runs with the fixture directory as its only allowed root and default working directory and with the
  built-in configuration; an example that needs other settings states them (`config` TOML block or CLI arguments)
- a **startup example** tests the binary instead of the library: a `text command` block (`easy-peasy-mcp` and its
  arguments, separated by spaces), optionally a `text env` block (`NAME=value` lines) and `toml <path>` blocks (files
  written into the fixture before the start). The binary runs in the fixture directory with `HOME`,
  `XDG_CONFIG_HOME` and `APPDATA` pointing to an empty directory outside of it and without `EASY_PEASY_CONFIG`,
  unless the env block sets them; `<fixture>` works in the env and stderr blocks too. A `text stderr exit <code>` block is what the binary prints to stderr before it exits with that
  code; without it the server must start, and the `json request` blocks that follow (`json request get_config` for
  another tool) are sent to it over MCP. A line
  `` Platform: `linux` `` (or `macos`, `windows`) runs the example only there
- a startup example can also be a JSON-RPC exchange after `initialize`: a `json send` block is a message sent as it
  is; a `json receive` block waits (at most 10 seconds) for a message which matches it and no earlier `json receive`
  took — the block is a subset of the message, a string `*` matches any value and a `*` inside a string any
  characters — so concurrent responses can be received in any order; a `json never` block fails when a message which matches it has arrived in the exchange or
  arrives within one second; the response to `initialize` (id 1) is the first message of the exchange, so a
  `json receive` block can check it too
- `<version>` in a response stands for the server version (`CARGO_PKG_VERSION`), e.g. in the first line of
  `get_config`; `<fixture>` in any example stands for the canonical absolute path of the fixture directory, in the
  requests too
- `<output_dir>` stands for the canonical path of `server.output_dir`, in the requests too: the examples run with
  `server.output_dir` = the directory `out` next to the fixture directory, not created beforehand; a startup example
  runs with the temp directory of the system (`TMPDIR`, `TMP`, `TEMP`) pointing to an empty directory outside of the
  fixture, so its default `server.output_dir` is `<output_dir>`
- a `text file <path>` (or `json file <path>`) block after a response is the exact content of the file `<path>` of
  `<output_dir>` after that request, compared like a response
- a `text setup` block in an example (not in its fixture section) changes the fixture between the requests before
  and after it, with the actions of the fixture setup and `remove <path>` (removes a file, a symlink or a directory
  tree), e.g. to change a file after a `find`
- the examples never touch the trash of the machine: they run with the trash test hook of the library, which moves
  what a write tool puts into the trash into the directory `trash` next to the fixture directory (created when
  needed), at its path relative to its allowed root; a startup example passes the directory in
  `EASY_PEASY_TEST_TRASH`, the value `none` is a file system without a trash. A `text trash` block after a response is
  the content of that directory after that request: every entry on its own line, sorted by path, a directory ending
  with `/`, a symlink as `<path> -> <target>`; an empty block is an empty trash
- a line `` Slow reads: `2s` `` in an example makes every read of a file's content take that long (a test hook of the
  library, cut short at the deadline of the request), so a timeout can be shown without depending on the machine; a
  startup example passes it to the binary in `EASY_PEASY_TEST_SLOW_READS` (milliseconds)
- an examples file is written for a spec version (TOOLS.md#versioning): a line `` Spec: `1` `` before its fixtures
  runs every example of the file against a server whose current spec version is that one (a test hook of the library;
  a startup example passes it to the binary in `EASY_PEASY_TEST_SPEC`), so a request without `version` gets that
  spec; without the line the examples are for the current spec version. The examples of the tasks before 141 are
  spec 1 examples and keep their responses forever, the new examples are written for the current spec
- every example runs against a fresh server, so result ids start at `r1`; a multi-step example (`within`, `since`,
  pagination) is written as a numbered sequence of requests and responses in one example
- nothing in an example may depend on the machine: no absolute paths outside the fixture, no current time — dates
  are absolute or relative to a fixed `now` given in the fixture section: a line `` Now: `2026-10-08 12:00` `` (UTC); the
  test server runs with this `now` and with UTC as its local time zone
- examples are written with the conventions of Linux: on every platform the test server compares paths
  case-sensitively for `path_case = native`, and on Windows the responses and the stderr are compared with every
  `\` read as `/`; unless the example is marked with a platform — then it runs with the conventions of that platform
  and is compared exactly. `<fixture>` is written in the native form, on Windows without the `\\?\` prefix

### 3. Plan — `N__PLAN__NAME.md`

Written when the task is taken, on the task branch; set the task status to `in progress` in the same commit.

- design: modules touched or added, new types, data flow
- decisions and the alternatives rejected, briefly
- the test list: unit tests per module, example tests (by id), integration tests
- implementation steps; when the task is finished the build is green and the binary usable
- risks and open questions

### 4. Tests, then code

Strict TDD:

1. **Red** — write the example tests (they read `N__EXAMPLES__NAME.md`) and the unit tests from the plan; run them
   and see them fail for the right reason.
2. **Green** — write the simplest code that makes them pass.
3. **Refactor** — clean up with all tests green. No behavior change without a test.

Commit the failing tests first (a red commit is fine on the task branch), then the code that makes them pass, in one
or more commits. Refactor in small steps.

### 5. Summary — `N__SUMMARY__NAME.md`

Written when the task is done. In the same commit set the task status to `implemented`, link the summary from the
task file and bump the server version, see [Server version](#server-version).

- what was built, feature by feature, with links to the spec sections
- architecture: modules, key types, how a request flows through them
- deviations from the plan and why
- tests: what is covered and where (example ids, test files)
- limitations and known gaps, and which future task closes them
- how to try it: command line, an MCP client registration, a sample request
- the new server version

### Server version

The server version (TOOLS.md#versioning) is `version` in `Cargo.toml`; `serverInfo` and `--version` take it from
there (`env!("CARGO_PKG_VERSION")`), never a second copy. It is bumped after every implemented task, in the summary
commit:

- a task adds or changes features: bump the minor version, `0.N.0` → `0.N+1.0`
- a task only fixes bugs, refactors or changes tooling: bump the patch version, `0.N.M` → `0.N.M+1`
- `1.0.0` is set by hand when the spec is fully implemented, not by a task

The first task starts at `0.1.0`. The spec version is unrelated and changes only by the spec's own rules. Examples
and tests never hard-code the server version: a test that checks `serverInfo` compares with `CARGO_PKG_VERSION`.

### New tasks

An agent that finds a bug, a missing feature or a way to save tokens (see [Agent tools](#agent-tools)) writes the
task itself, without waiting to be asked:

- `N__TASK__NAME.md` with the next free number, status `not implemented`, `Depends on`, and a goal that says how it was
  found: the request, the response (or the part of it that matters) and, for a token saving, an estimate of the
  characters or tokens saved per file and per response
- a row in [tasks/README.md](tasks/README.md)
- one commit on `master`, `Add task N: <title>`, with the finding in the body; when the agent is on a task branch, the
  commit goes to `master` after that task is merged, so the task branch stays about its own task
- the agent tells the user the number and the title; it doesn't implement the new task in passing

## Always a usable application

After every task — at the summary commit and on `master` after the merge — the program builds, passes all tests and
works as an MCP server. Intermediate commits on a task branch may be red:

- `cargo build --release` produces `easy-peasy-mcp`, which starts over stdio, answers `initialize`, lists its tools and
  answers `find` for the implemented subset
- a feature that is not implemented yet fails cleanly as a tool error (`isError: true`) with the code
  `not_implemented` and a message naming the feature; never a panic, a hang, a silent wrong answer or a protocol error.
  Adding an error code is not a breaking change (TOOLS.md#versioning); add `not_implemented` to the request errors
  table in OUTPUT.md#errors with the first task that uses it. Examples never show it, see [Examples](#2-examples--n__examples__namemd)
- no dead stubs, no `todo!()` / `unimplemented!()` reachable from a request
- the spec features of the task are complete for what they cover: a task that adds `text has` adds it with its errors,
  modifiers listed in the task scope, and output exactly as specified

## Rust conventions

### Toolchain

- stable Rust, edition 2024; pin the toolchain in `rust-toolchain.toml` and `rust-version` in `Cargo.toml`
- `cargo fmt` with the default style
- `cargo clippy --all-targets --all-features -- -D warnings`, with `clippy::pedantic` enabled as warnings in
  `[lints]` of `Cargo.toml`; `#[allow(...)]` only locally and with a comment saying why
- `unsafe_code = "forbid"`
- dependencies: well-known crates (see the candidates in README.md); every new dependency is named in the plan with
  the reason; `cargo deny` / `cargo audit` when they are set up

### Code

- one crate, a library plus a thin binary: `src/lib.rs` holds everything, `src/main.rs` only parses the command line,
  sets up logging and runs the server
- small modules with one responsibility; `pub` only what other modules need, `pub(crate)` by default
- make invalid states unrepresentable: enums over flags, newtypes over raw strings and numbers (`ConditionId`,
  `ResultId`, `SpecVersion`, `ByteSize`), parse at the boundary — validate input once, pass typed values inward
- no `unwrap()` / `expect()` / `panic!` in non-test code, except for proven invariants with an `expect` message
  stating the invariant
- errors: `thiserror` enums per module; every user-facing error maps to exactly one spec error code
  (`unknown_operator`, `dir_not_allowed`, ...) with message, position and hint as the spec defines; `anyhow` only in
  `main.rs`
- no global mutable state; the configuration is an immutable value, a request works on a snapshot (`Arc<Config>`)
- I/O at the edges: parsing, evaluation and formatting are pure functions over data and are unit tested without the
  file system; the walker and file reader sit behind small traits only where a test needs a fake
- iterators and borrowing over cloning; no premature optimization, but no accidental quadratic work over files or
  lines either; measure before optimizing (benches in `benches/` with `criterion`)
- concurrency: the walk and search are parallel (`ignore`, a thread pool of `server.threads`); the MCP layer is async
  (`tokio`, `rmcp`); blocking search work runs off the async runtime; cancellation and timeout are checked
  cooperatively
- cross-platform from day one: `Path`/`PathBuf`, never string paths; output paths use `/` as the spec says; no
  Unix-only APIs without a `cfg` alternative
- logging with `tracing` to stderr only; stdout carries only protocol messages
- doc comments (`///`) on every public item, saying what it does, not how; comments in the code only where the reason
  isn't obvious
- names follow the spec vocabulary: target, condition, scope, modifier, fragment, funnel, facet

### Architecture

The request flows one way through layers; inner layers don't know about outer ones.

```
mcp (rmcp tools, JSON schema, stdio)        — adapter: arguments → Request, Response → text content
  └─ request (Request, merge with Config)   — validated, typed request
       ├─ query (lexer → parser → AST → semantic check → plan)
       ├─ search (walk, read, decode, evaluate conditions, collect matches)
       └─ output (relevance, limits, budget, funnel, facets → text | json)
config (defaults ← file ← CLI ← update_config ← request)
```

Suggested layout, grown task by task, not created empty in advance:

```
src/
  main.rs            command line, logging, start the server
  lib.rs             module tree, the public entry point `find(&Config, Request) -> Response`
  mcp/               tool definitions, schemas, results, errors, progress, cancellation
  config/            sections, sources, merge, get_config / update_config TOML
  request/           request types and validation
  query/             lexer, parser, ast, semantic checks, errors with positions and hints
  search/            walker, file reading and decoding, condition evaluation
  output/            response model, text format, json format
tests/
  common/            helpers: example file reader, fixture setup, MCP client over stdio
  fixtures/          file trees used by the examples
  examples.rs        runs every example of every task
  mcp_stdio.rs       end-to-end: spawn the binary, talk JSON-RPC
```

- the core entry point is a plain function, callable from tests without MCP
- the MCP layer is thin and tested end to end by spawning the binary

## Tests

- **unit tests** in `#[cfg(test)] mod tests` next to the code: lexer, parser, each operator, formatter, merge rules
- **example tests** in `tests/examples.rs`: read every `tasks/*__EXAMPLES__*.md`, build the fixture in a temp dir
  (`tempfile`), run the request through the library entry point, compare the whole response text exactly
  (`pretty_assertions` for readable diffs); a test failure names the example id
- **end-to-end tests** in `tests/mcp_stdio.rs`: spawn the built binary, run `initialize`, `tools/list`, a `find`;
  also cancellation and timeout once implemented
- property tests (`proptest`) where they pay off: the parser never panics, escaping round-trips
- tests are deterministic: fixed mtimes and `now`, sorted output, no network, no dependence on the machine's files
- a bug fix starts with a test that reproduces it

## Definition of done

Before marking a step or a task as done, all of these pass:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build --release
```

And for a task:

- every example in `N__EXAMPLES__NAME.md` and of every earlier task passes
- the binary was started and a `find` request answered over stdio (an end-to-end test does this)
- the spec is consistent with the code; any spec change is committed
- `N__SUMMARY__NAME.md` is written and the task status is `implemented`
- the server version in `Cargo.toml` is bumped, see [Server version](#server-version)

## Git

- every task is done on its own feature branch from `master`: `task/N-name`, e.g. `task/1-mvp`
- the task file `N__TASK__NAME.md` is written beforehand and is already on `master` (status `not implemented`)
- an agent taking a task commits on the branch in this order, at least five commits per task:

  | # | Commit                                                          | Build      |
  |---|-----------------------------------------------------------------|------------|
  | 1 | `N__EXAMPLES__NAME.md` and the fixtures                         | unchanged  |
  | 2 | `N__PLAN__NAME.md`, task status `in progress`                   | unchanged  |
  | 3 | tests: example tests and unit tests, failing                    | red is OK  |
  | 4 | code that makes the tests pass; may be several commits          | green at the end |
  | 5 | `N__SUMMARY__NAME.md`, task status `implemented`, version bump  | green      |

- red tests may be committed on a task branch; `master` is always green
- a spec fix found during the task is its own commit on the branch
- the branch is merged into `master` only when the [definition of done](#definition-of-done) is met
- as soon as the task is ready (the summary commit is done) and all the checks of the definition of done are green, the
  agent merges the branch into `master` with `git merge --no-ff` (`Merge task N: <title>`), without waiting to be asked
- right after the merge, the agent pushes `master` (`git push origin master`): agent worktrees start from
  `origin/HEAD`, so an unpushed `master` sends them to an old commit
- then the agent builds and installs the new version from `master`, so the developer's machine always
  runs the latest server: `./install.sh --build` (`install.bat --build` on Windows), then checks that
  `easy-peasy-mcp --version` names the version of `Cargo.toml`; the install keeps a running server on its old binary,
  so the agent tells the user to restart the MCP server (`/mcp` in Claude Code) to use the new one
- a commit message says what and why in the imperative mood
