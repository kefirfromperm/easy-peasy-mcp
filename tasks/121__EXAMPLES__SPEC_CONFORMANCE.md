# Examples for task 121

The request and response examples of the spec, as tests: every example names the file and the section it
reproduces — README.md and TOOLS.md (E121.1–E121.19), QUERY.md (E121.20–E121.49), OUTPUT.md (E121.50–E121.99) and
CONFIG.md (E121.100–E121.119). Startup examples run the binary for the command line, the config files and the
JSON-RPC exchanges. The queries of QUERY.md which have no response are checked by `tests/spec_queries.rs`: each one
parses.

## Fixtures

### spec-readme-query

```text tree
README.md          2026-10-07 10:00
src/config.rs      2026-10-07 10:00
src/legacy.rs      2026-09-20 10:00
src/main.rs        2026-10-07 10:00
tests/load.rs      2026-10-07 10:00
```

Now: `2026-10-08 12:00`

File contents are in `tests/fixtures/spec-readme-query/`. `src/main.rs` calls `unwrap` in two lines and mentions it in
a comment line, `src/config.rs` only in a comment line; `src/legacy.rs` is older than a week, `tests/load.rs` is in
`tests/`, `README.md` is not Rust.

### spec-readme-request

```text tree
README.md                   2026-10-01 10:00
src/config.rs               2026-10-01 10:00
src/error.rs                2026-10-01 10:00
src/main.rs                 2026-10-01 10:00
target/debug/build/out.rs   2026-10-01 10:00
```

```text setup
bytes target/debug/build/out.rs 6c65742078203d206c6f616428292e756e7772617028293b0a
```

File contents are in `tests/fixtures/spec-readme-request/`; `target/debug/build/out.rs` (`let x = load().unwrap();`)
is written by the setup. `src/config.rs` has `unwrap(` and `expect(` in lines 42 and 43 and `unwrap()` in a doc
comment, `src/main.rs` has `unwrap(` in line 18; `README.md` mentions both.

### spec-tools-results

```text tree
src/lib.rs     2026-10-01 10:00
src/main.rs    2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-tools-results/`: line 12 of `src/main.rs` is `// TODO remove`, `src/lib.rs`
has another `TODO`.

### spec-tools-progress

```text tree
logs/api-1.log       2026-10-07 10:00
logs/api-2.log       2026-10-07 10:00
logs/api-3.log       2026-10-07 10:00
logs/api-4.log       2026-10-07 10:00
logs/search-1.log    2026-10-07 10:00
logs/search-2.log    2026-10-07 10:00
logs/worker-1.log    2026-10-07 10:00
logs/worker-2.log    2026-10-07 10:00
```

File contents are in `tests/fixtures/spec-tools-progress/`: eight log files, three of them with the request id
`req-7f3a9c`. With one search thread and slow reads a search of the content takes a few seconds.

### spec-tools-session

```text tree
build.rs                     2026-10-01 10:00
Cargo.toml                   2026-10-01 10:00
docs/errors.md               2026-10-01 10:00
src/cache.rs                 2026-10-01 10:00
src/cli.rs                   2026-10-01 10:00
src/config.rs                2026-10-01 10:00
src/eval/mod.rs              2026-10-01 10:00
src/eval/ops.rs              2026-10-01 10:00
src/lib.rs                   2026-10-01 10:00
src/main.rs                  2026-10-01 10:00
src/output.rs                2026-10-01 10:00
src/parser/ast.rs            2026-10-01 10:00
src/parser/lexer.rs          2026-10-01 10:00
src/parser/mod.rs            2026-10-01 10:00
src/server/handlers.rs       2026-10-01 10:00
src/server/mod.rs            2026-10-01 10:00
src/walk.rs                  2026-10-01 10:00
tests/integration.rs         2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-tools-session/`: a small Rust project. Fourteen Rust files call `unwrap(`
outside of comments, 57 times in 52 lines: `src/eval/mod.rs` 10 times in 8 lines, `src/config.rs` 8 in 7,
`src/cache.rs` 7 in 6, `src/server/handlers.rs` 6 in 5, `src/main.rs` 5 in 5, `src/cli.rs` 4, `src/eval/ops.rs` and
`src/server/mod.rs` 3, `build.rs`, `src/output.rs`, `src/walk.rs`, `src/parser/lexer.rs` and `tests/integration.rs` 2,
`src/parser/ast.rs` 1. `src/lib.rs` and `src/parser/mod.rs` have `unwrap()` only in comments, `docs/errors.md` is not
Rust.

### spec-tools-examples

```text tree
.patches/dbg.patch         2026-10-01 10:00
LICENSE                    2026-10-01 10:00
data/events.json           2026-10-01 10:00
data/users.json            2026-10-01 10:00
docs/config-loader.md      2026-10-01 10:00
scripts/fetch.py           2026-10-01 10:00
scripts/report.py          2026-10-01 10:00
src/config_loader.rs       2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
web/api.ts                 2026-10-01 10:00
web/app.tsx                2026-10-01 10:00
web/configLoader.ts        2026-10-01 10:00
```

```text setup
git . init -q -b main
git . add .
git . commit -q -m init
git . apply .patches/dbg.patch
```

File contents are in `tests/fixtures/spec-tools-examples/`. The setup makes the fixture a git repository with one
commit, then the patch adds a `dbg!(&config);` line to `src/main.rs`, so it has uncommitted changes; `src/lib.rs` has
a committed `dbg!(`. The source files start with an `SPDX-License-Identifier` line, `LICENSE` is the license;
`scripts/report.py`, `docs/config-loader.md` and the files of `data/` don't mention a license. `data/events.json` has
`"role": "admin"` inside a JSON string, `data/users.json` as it is.

### spec-query-find

```text tree
README.md                    2026-10-08 09:00
docs/readme.txt              2025-09-01 10:00
src/main.rs                  2026-10-01 10:00
src/app.ts                   2026-10-01 10:00
src/view.tsx                 2026-10-01 10:00
src/view.jsx                 2026-10-01 10:00
db/migrations/001_init.sql   2026-10-01 10:00
db/seed.sql                  2026-10-01 10:00
vendor/lib.rs                2026-10-01 10:00
tmp/                         2026-10-01 10:00
data/dump.bin                2026-10-01 10:00
latest.sql                   2026-10-01 10:00
```

```text setup
mkdir tmp
file data/dump.bin 104857601
symlink latest.sql -> db/migrations/001_init.sql
```

Now: `2026-10-08 12:00`

File contents are in `tests/fixtures/spec-query-find/`. `tmp/` is an empty directory, `data/dump.bin` has one byte
more than 100M, `latest.sql` is a symlink to `db/migrations/001_init.sql`. `README.md` was changed during the last 24
hours, `docs/readme.txt` more than a year ago. Times are UTC.

### spec-query-grep

```text tree
app.conf          2026-10-01 10:00
notes.txt         2026-10-01 10:00
src/main.py       2026-10-01 10:00
src/util.py       2026-10-01 10:00
src/parse.rs      2026-10-01 10:00
LICENSE           2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-grep/`. `notes.txt` has `error_code`, `src/parse.rs` `errors` and
`fn test_parse`; `LICENSE` and `src/parse.rs` mention a license. Times are UTC.

### spec-query-words

```text tree
src/cache.rs      2026-10-01 10:00
src/state.rs      2026-10-01 10:00
src/pool.rs       2026-10-01 10:00
src/server.rs     2026-10-01 10:00
src/worker.rs     2026-10-01 10:00
src/bridge.rs     2026-10-01 10:00
docs/logging.md   2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-words/`. `src/state.rs` has `Mutex` and `async` on different lines,
`src/cache.rs` has `tokio`, but no `rayon`; `src/server.rs` uses `log::`, `src/worker.rs` `tracing::`,
`src/bridge.rs` both, and `docs/logging.md` mentions `log::`. Times are UTC.

### spec-query-js

```text tree
web/app.js            2026-10-01 10:00
web/bundle.js         2026-10-01 10:00
web/vendor.min.js     2026-10-01 10:00
web/data.json         2026-10-01 10:00
```

```text setup
file web/bundle.js 600
file web/vendor.min.js 800
file web/data.json 700
```

`web/app.js` is in `tests/fixtures/spec-query-js/`, its lines are short. The other files are one line of `x` of the
given number of characters; `vendor.min.js` is `generated` by its name. Times are UTC.

### spec-query-rust

```text tree
src/config.rs     2026-10-07 18:00
src/main.rs       2026-10-06 09:00
src/old.rs        2026-09-20 10:00
src/lib.rs        2026-10-05 10:00
src/eval.rs       2026-10-05 10:00
tests/eval.rs     2026-10-07 10:00
```

Now: `2026-10-08 12:00`

File contents are in `tests/fixtures/spec-query-rust/`. `src/lib.rs` doesn't end with a line break; `src/main.rs` has
`.unwrap()` only in a comment; `src/old.rs` was changed more than 7 days ago; in `src/eval.rs` a comment is between
`#[test]` and `fn eval_huge_input_slow`. Times are UTC.

### spec-query-secrets

```text tree
config/app.yml          2026-10-01 10:00
config/dev.env          2026-10-01 10:00
config/db.properties    2026-10-01 10:00
src/login.py            2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-secrets/`. `config/app.yml` has a commented out password on line 4,
`config/db.properties` has `db.password=` without a value. Times are UTC.

### spec-query-roles

```text tree
tests/fixtures/users.json   2026-10-01 10:00
src/roles.json              2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-roles/`. Times are UTC.

### spec-query-big-log

```text tree
logs/app.log      2026-10-01 10:00
big/pad.txt       2026-10-01 10:00
big/server.log    2026-10-01 10:00
```

```text setup
file big/pad.txt 10485760
git big init -q -b main
git big add pad.txt
git big commit -q -m req-7f3a9c
git big log -p --output=server.log
```

`logs/app.log` is in `tests/fixtures/spec-query-big-log/`: `started` and `request req-7f3a9c done`. `big/server.log`
is the output of `git log -p` of a commit with the message `req-7f3a9c` which adds `pad.txt`, 10M of `x`: a log of a
little more than 10M with the request id on line 5. Times are UTC.

### spec-query-logs

```text tree
logs/app.log        2026-10-01 10:00
logs/other.log      2026-10-01 10:00
logs/app.log.gz     2026-10-01 10:00
logs/2026-10.zip    2026-10-01 10:00
logs/old.tar.gz     2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-logs/`. Every log has `started` on line 1 and a request on line 2:
`logs/app.log` `request req-7f3a9c done`, `logs/other.log` `request req-1b2c3d done`, the compressed `logs/app.log.gz`
`request req-7f3a9c failed`; the zip archive has the member `app/server.log` with `request req-7f3a9c retried`, the
tar.gz archive the member `old/worker.log` with `request req-7f3a9c queued`. Times are UTC.

### spec-query-vague

```text tree
src/config_loader.rs          2026-10-01 10:00
src/ConfigLoader.java         2026-10-01 10:00
src/cfg_load.go               2026-10-01 10:00
docs/config_loader.md         2026-10-01 10:00
tests/config_loader_test.rs   2026-10-01 10:00
src/query.rs                  2026-10-01 10:00
py/query.py                   2026-10-01 10:00
go/query.go                   2026-10-01 10:00
web/query.js                  2026-10-01 10:00
web/main.js                   2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-vague/`. `src/query.rs` also defines `parse_query_fast` and calls
`parse_query`, `web/main.js` calls `parseQuery`. Times are UTC.

### spec-query-branch

```text tree
src/a.rs          2026-10-01 10:00
src/c.rs          2026-10-01 10:00
src/d.rs          2026-10-01 10:00
tests/b_test.rs   2026-10-01 10:00
docs/notes.md     2026-10-01 10:00
```

```text setup
git . init -q -b main
git . add src/c.rs
git . commit -q -m init
git . checkout -q -b feature
git . add src/a.rs src/d.rs
git . commit -q -m add-a-and-d
```

File contents are in `tests/fixtures/spec-query-branch/`. The repository is on the branch `feature`, made from
`main`: `src/a.rs` and `src/d.rs` were added by a commit of the branch, `tests/b_test.rs` and `docs/notes.md` are
untracked, `src/c.rs` is unchanged. Every file has a `TODO` except `src/d.rs`. Times are UTC.

### spec-query-basics

```text tree
notes/a.txt                  2026-10-01 10:00
notes/b.txt                  2026-10-01 10:00
db/migrations/001_init.sql   2026-10-01 10:00
data/k1_5.bin                2026-10-01 10:00
src/lib.rs                   2026-10-01 10:00
```

```text setup
file data/k1_5.bin 1536
```

File contents are in `tests/fixtures/spec-query-basics/`: `notes/a.txt` has `TODO`, `notes/b.txt` `todo`;
`data/k1_5.bin` has 1536 bytes. Times are UTC.

### spec-query-content

```text tree
img/logo.png      2026-10-01 10:00
data/blob.bin     2026-10-01 10:00
docs/brand.md     2026-10-01 10:00
notes.txt         2026-10-01 10:00
```

```text setup
bytes img/logo.png 89504e470d0a1a0a0000000d49484452
bytes data/blob.bin 006c6f676f00
```

`docs/brand.md` (`Use the logo on a white background.`) and `notes.txt` are in `tests/fixtures/spec-query-content/`.
`img/logo.png` starts like a PNG image; `data/blob.bin` is `logo` between two zero bytes. Both are binary. Times are
UTC.

### spec-query-scope

```text tree
Cargo.toml          2026-10-01 10:00
src/main.rs         2026-10-01 10:00
src/lib.rs          2026-10-01 10:00
conf/nginx.conf     2026-10-01 10:00
scripts/short.py    2026-10-01 10:00
scripts/long.py     2026-10-01 10:00
db/migrate.sql      2026-10-01 10:00
db/broken.sql       2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-scope/`. Line 1 of `scripts/long.py` has 131 characters,
`db/broken.sql` has a `BEGIN` without a `COMMIT`. Times are UTC.

### spec-query-comments

```text tree
src/a.rs          2026-10-01 10:00
src/b.rs          2026-10-01 10:00
src/c.rs          2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-comments/`: `src/a.rs` is the code of the block of
QUERY.md#comments, `src/b.rs` has `unsafe` only in a comment, `src/c.rs` in the code. Times are UTC.

### spec-query-escaped

```text tree
data/types.txt    2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-escaped/`: the three lines of QUERY.md#escaping and a fourth one, the
object escaped on level 3. Times are UTC.

### spec-query-escaping

```text tree
data/events.json    2026-10-01 10:00
db/users.sql        2026-10-01 10:00
web/page.html       2026-10-01 10:00
web/links.txt       2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-escaping/`. Line 2 of `data/events.json` is the object
`{"type": "admin"}` stringified in three levels of objects. Line 1 of `db/users.sql`, of `web/page.html` and line 3 of
`web/links.txt` have the values unescaped. Times are UTC.

### spec-query-near

```text tree
src/copy.py       2026-10-01 10:00
src/reload.py     2026-10-01 10:00
src/cache.rs      2026-10-01 10:00
src/store.rs      2026-10-01 10:00
tests/api.rs      2026-10-01 10:00
pkg/user.go       2026-10-01 10:00
pkg/order.go      2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-near/`. `src/reload.py` calls `close(` before `open(`; in
`src/store.rs` `.await` is 6 lines after `.lock()`; `pkg/order.go` returns the error. Times are UTC.

### spec-query-naming

```text tree
src/styles.txt          2026-10-01 10:00
src/user.rs             2026-10-01 10:00
src/config_loader.rs    2026-10-01 10:00
src/ConfigLoader.java   2026-10-01 10:00
web/config-loader.ts    2026-10-01 10:00
src/loader_config.rs    2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-naming/`: `src/styles.txt` has `parse query` in the eight naming
styles of QUERY.md#n--any-naming-style and then `parse__query`. Times are UTC.

### spec-query-fuzzy

```text tree
src/config_loader.rs        2026-10-01 10:00
src/ConfigLoader.java       2026-10-01 10:00
src/server/handlers.rs      2026-10-01 10:00
src/server/mod.rs           2026-10-01 10:00
services/http/handler.go    2026-10-01 10:00
src/cache.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-fuzzy/`. Times are UTC.

### spec-query-labels

```text tree
src/config.rs     2026-10-01 10:00
src/main.rs       2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-labels/`. Times are UTC.

### spec-query-gotchas

```text tree
tests/a.rs                2026-10-01 10:00
crates/core/tests/b.rs    2026-10-01 10:00
src/furs                  2026-10-01 10:00
src/contest.rs            2026-10-01 10:00
src/main.rs               2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-query-gotchas/`. Times are UTC.

### spec-output-config

```text tree
src/config.rs              2026-10-01 10:00
```

In `tests/fixtures/spec-output-config/`: `src/config.rs` is a Rust module of 55 lines; its lines 41–43, in
`load_config`, are:

```text
    let raw = fs::read_to_string(path)?;
    let cfg: Config = toml::from_str(&raw).unwrap();
    let port = cfg.port.expect("port is required");
```

Times are UTC.

### spec-output-funnel

```text tree
src/main.rs                2026-09-01 10:00
src/lib.rs                 2026-09-01 10:00
src/cli.rs                 2026-09-01 10:00
src/config.rs              2026-09-01 10:00
src/server.rs              2026-09-01 10:00
src/eval/value.rs          2026-09-01 10:00
tests/eval.rs              2026-09-01 10:00
src/parser/mod.rs          2026-10-05 10:00
src/parser/lexer.rs        2026-10-06 10:00
src/parser/ast.rs          2026-10-06 10:00
src/eval/mod.rs            2026-10-07 10:00
tests/parse.rs             2026-10-07 10:00
```

Now: `2026-10-08 12:00`

```text setup
file src/main.rs
file src/lib.rs
file src/cli.rs
file src/config.rs
file src/server.rs
file src/eval/value.rs
file tests/eval.rs
file src/parser/mod.rs
file src/parser/lexer.rs
file src/parser/ast.rs
file src/eval/mod.rs
file tests/parse.rs
```

Twelve empty Rust files, five of them changed in the last seven days. Times are UTC.

### spec-output-hints

```text tree
src/main.rs                2026-09-01 10:00
src/lib.rs                 2026-09-01 10:00
src/cli.rs                 2026-09-01 10:00
src/config.rs              2026-09-01 10:00
src/server.rs              2026-09-01 10:00
src/eval/mod.rs            2026-09-01 10:00
tests/eval.rs              2026-09-01 10:00
src/api/handlers.rs        2026-10-06 10:00
src/api/routes.rs          2026-10-06 10:00
src/parser/mod.rs          2026-10-07 10:00
src/parser/query.rs        2026-10-07 10:00
src/tables.rs              2026-10-05 10:00
src/gen/schema.rs          2026-10-05 10:00
src/gen/types.rs           2026-10-05 10:00
vendor/qparse/src/lexer.rs 2026-10-05 10:00
vendor/qparse/src/parser.rs 2026-10-05 10:00
vendor/qparse/src/query.rs 2026-10-05 10:00
vendor/qparse/src/tokens.rs 2026-10-05 10:00
vendor/qparse/src/util.rs  2026-10-05 10:00
```

Now: `2026-10-08 12:00`

In `tests/fixtures/spec-output-hints/`: twelve Rust files which are searched, five of them changed in the last seven
days: `src/api/handlers.rs` and `src/api/routes.rs` call `parse_query`, `src/parser/mod.rs` and
`src/parser/query.rs` have `ParseQuery`, `src/tables.rs` (1169 bytes) has `parseQuery`. `parseQuery` is also in the
five files of `vendor/` (ignored by `.ignore`) and in the two files of `src/gen/` (generated: `// @generated` on their
first line). Times are UTC.

### spec-output-facets

```text tree
README.md                  2026-10-01 10:00
Cargo.toml                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/parser/mod.rs          2026-10-01 10:00
src/parser/lexer.rs        2026-10-01 10:00
src/parser/ast.rs          2026-10-01 10:00
src/eval/mod.rs            2026-10-01 10:00
tests/unit/parse.rs        2026-10-01 10:00
tests/unit/eval.rs         2026-10-01 10:00
tests/integration/api.rs   2026-10-01 10:00
tests/integration/db.rs    2026-10-01 10:00
tests/integration/cli.rs   2026-10-01 10:00
tests/integration/test_ui.py 2026-10-01 10:00
```

In `tests/fixtures/spec-output-facets/`: a small Rust project with Rust tests and one Python test
(`tests/integration/test_ui.py`, `expect(page)`). `unwrap(` is in seven source and test files, `expect(` in four, both
in `tests/unit/eval.rs`; `README.md` mentions `unwrap()` too, but it is documentation. Times are UTC.

### spec-output-huge

```text tree
src/main.rs                2026-10-01 10:00
src/parser/error.rs        2026-10-01 10:00
src/parser/lexer.rs        2026-10-01 10:00
src/parser/mod.rs          2026-10-01 10:00
tests/integration/eval.rs  2026-10-01 10:00
tests/integration/parse.rs 2026-10-01 10:00
vendor/hyper/capi/client.c 2026-10-01 10:00
vendor/hyper/capi/server.c 2026-10-01 10:00
vendor/hyper/src/client.rs 2026-10-01 10:00
vendor/hyper/src/server.rs 2026-10-01 10:00
vendor/tokio/src/io.rs     2026-10-01 10:00
vendor/tokio/src/net.rs    2026-10-01 10:00
vendor/tokio/src/runtime.rs 2026-10-01 10:00
vendor/tokio/src/sync.rs   2026-10-01 10:00
vendor/tokio/src/time.rs   2026-10-01 10:00
```

In `tests/fixtures/spec-output-huge/`: `error` is in fourteen files: five Rust files in `vendor/tokio/`, two Rust and
two C files in `vendor/hyper/`, three files in `src/parser/` and two tests in `tests/integration/`;
`src/parser/error.rs` has it on lines 1 and 7. Times are UTC.

### spec-output-since

```text tree
src/auth.rs                2026-10-01 10:00
src/cache.rs               2026-10-08 11:00
src/config.rs              2026-10-01 10:00
src/db.rs                  2026-10-01 10:00
src/handlers.rs            2026-10-01 10:00
src/jobs.rs                2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/metrics.rs             2026-10-01 10:00
src/queue.rs               2026-10-01 10:00
src/routes.rs              2026-10-01 10:00
src/server.rs              2026-10-01 10:00
src/session.rs             2026-10-01 10:00
src/util.rs                2026-10-01 10:00
```

In `tests/fixtures/spec-output-since/`: fourteen Rust files. `src/config.rs` is the one of `spec-output-config`
(`unwrap(` on line 42, `expect(` on line 43); `src/cache.rs` has `unwrap(` on line 63, indented by eight spaces;
`src/main.rs` has `unwrap(` only in a comment and calls `expect(`; ten more files have 28 lines with `unwrap(`, one of
them twice, and no `expect(`; `src/lib.rs` has neither. `src/cache.rs` is the newest file. Times are UTC.

### spec-output-batch

```text tree
src/cli.rs                 2026-10-01 10:00
src/config.rs              2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/main.rs                2026-10-01 10:00
src/server/mod.rs          2026-10-01 10:00
tests/test_cli.py          2026-10-01 10:00
tests/test_config.py       2026-10-01 10:00
```

In `tests/fixtures/spec-output-batch/`: `src/config.rs` is the one of `spec-output-config`, it defines `load_config`;
`src/main.rs` (6 lines) and `src/cli.rs` (17 lines) call it once, `src/server/mod.rs` (47 lines) twice. The Python
tests name `load_config` on five lines. Times are UTC.

### spec-output-warnings

```text tree
config/                    2026-10-01 10:00
config/app.toml            2026-10-01 10:00
root-only/                 2026-10-01 10:00
root-only/token.txt        2026-10-01 10:00
secrets/                   2026-10-01 10:00
secrets/a.key              2026-10-01 10:00
secrets/b.key              2026-10-01 10:00
```

```text setup
chmod secrets/a.key 000
chmod secrets/b.key 000
chmod root-only 000
```

In `tests/fixtures/spec-output-warnings/`: `config/app.toml` has `password` on line 3, `root-only/token.txt` has it
too. The setup removes all permissions of the two keys and of `root-only/`, so the example runs only where
permissions can be removed (Unix, not as root). Times are UTC.

### spec-output-dedent

```text tree
src/server/handlers.rs     2026-10-01 10:00
```

In `tests/fixtures/spec-output-dedent/`: `src/server/handlers.rs` has 147 lines; `unwrap(` is only on line 141, lines
140–142 are indented by twelve spaces. Times are UTC.

### spec-output-conditions

```text tree
README.md                  2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
src/c.rs                   2026-10-01 10:00
```

In `tests/fixtures/spec-output-conditions/`: `src/a.rs` has `.unwrap()` on one line, `src/b.rs` `.expect(` on two
lines, `src/c.rs` one line of each; the small `README.md` has a `TODO`. Times are UTC.

### spec-output-groups

```text tree
src/eval/mod.rs            2026-10-01 10:00
src/parser/ast.rs          2026-10-01 10:00
src/parser/lexer.rs        2026-10-01 10:00
```

In `tests/fixtures/spec-output-groups/`: `unwrap(` is on two lines of `src/parser/lexer.rs` (5 lines), on one line of
`src/parser/ast.rs` (7 lines) and of `src/eval/mod.rs` (16 lines). Times are UTC.

### spec-output-many

```text tree
src/main.rs                2026-10-08 10:00
src/config.rs              2026-10-08 09:00
src/eval/mod.rs            2026-10-08 08:00
src/api/admin.rs           2026-10-01 10:00
src/api/health.rs          2026-10-01 10:00
src/api/items.rs           2026-10-01 10:00
src/api/orders.rs          2026-10-01 10:00
src/api/search.rs          2026-10-01 10:00
src/api/users.rs           2026-10-01 10:00
src/db/items.rs            2026-10-01 10:00
src/db/migrate.rs          2026-10-01 10:00
src/db/orders.rs           2026-10-01 10:00
src/db/pool.rs             2026-10-01 10:00
src/db/schema.rs           2026-10-01 10:00
src/db/users.rs            2026-10-01 10:00
src/eval/errors.rs         2026-10-01 10:00
src/eval/ops.rs            2026-10-01 10:00
src/eval/scope.rs          2026-10-01 10:00
src/eval/types.rs          2026-10-01 10:00
src/eval/value.rs          2026-10-01 10:00
src/jobs/cleanup.rs        2026-10-01 10:00
src/jobs/export.rs         2026-10-01 10:00
src/jobs/import.rs         2026-10-01 10:00
src/jobs/mail.rs           2026-10-01 10:00
src/jobs/report.rs         2026-10-01 10:00
src/jobs/sync.rs           2026-10-01 10:00
src/net/client.rs          2026-10-01 10:00
src/net/dns.rs             2026-10-01 10:00
src/net/proxy.rs           2026-10-01 10:00
src/net/retry.rs           2026-10-01 10:00
src/net/server.rs          2026-10-01 10:00
src/net/tls.rs             2026-10-01 10:00
src/store/blobs.rs         2026-10-01 10:00
src/store/cache.rs         2026-10-01 10:00
src/store/files.rs         2026-10-01 10:00
src/store/index.rs         2026-10-01 10:00
src/store/queue.rs         2026-10-01 10:00
```

In `tests/fixtures/spec-output-many/`: 37 Rust files of five lines; every one has the line `use anyhow::Result;`,
`src/main.rs` on line 3, `src/config.rs` on line 5, `src/eval/mod.rs` on line 2, the others on line 1. Line 3 of the
first 21 others in path order is a `// TODO: ...` of its own. Times are UTC.

### spec-output-long

```text tree
dist/app.js                2026-10-01 10:00
```

In `tests/fixtures/spec-output-long/`: `dist/app.js` is one minified line of 18777 characters; `unwrap` is at column
18342, after `var a=require("./x");var b=a.` and before `();var c=require("./y");var d=c;`. Times are UTC.

### spec-output-highlight

```text tree
data/users.json            2026-10-01 10:00
src/user.rs                2026-10-01 10:00
web/app.js                 2026-10-01 10:00
```

In `tests/fixtures/spec-output-highlight/`: `data/users.json` has `{"type": "admin"}` on line 1 and the same object
stringified on line 2; `src/user.rs` is `let user = request.name;`; `web/app.js` has 58 lines and ends with
`const xy = {` and `};`. Times are UTC.

### spec-examples-rust

```text tree
Cargo.toml                   2026-10-01 10:00
README.md                    2026-10-01 10:00
config/app.conf              2026-10-01 10:00
src/cache.rs                 2026-10-05 14:20
src/cli.rs                   2026-10-03 17:45
src/config.rs                2026-10-06 16:40
src/error.rs                 2026-10-03 09:30
src/eval.rs                  2026-10-02 09:10
src/eval/builtins.rs         2026-10-01 10:00
src/eval/ops.rs              2026-10-01 10:00
src/lexer.rs                 2026-10-04 10:15
src/main.rs                  2026-10-06 11:05
src/parser.rs                2026-10-07 18:22
tests/fixtures/users.json    2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-examples-rust/`: the small Rust project of OUTPUT.md#examples, an
interpreter. Seven `.rs` files have 12 lines with `TODO` (`src/parser.rs` 3, `src/cache.rs`, `src/eval.rs` and
`src/lexer.rs` 2 each, `src/cli.rs`, `src/error.rs` and `src/main.rs` 1 each), no other file has one. `unwrap(` is
only in `src/config.rs:42` and `src/main.rs:18`, `expect(` only in `src/config.rs:43`, `dbg!(` only in a comment of
`src/eval.rs`. `src/parser.rs` has 49357 bytes (48.2K), `src/eval.rs` 24474 (23.9K), the other files less than 4K.
`src/cache.rs` has `.lock()` on line 57 and `.await` on line 59, nowhere else. `src/eval.rs` has `#[test]` on line 210
and `fn eval_big_input_slow() {` on line 211. `config/app.conf` has comments, empty lines, `listen = 8080` on line 4
and `log_level = debug` on line 9. `tests/fixtures/users.json` has `"role": "admin"` on line 3 and the same escaped
once on line 8. `src/config.rs` is shorter and newer than `src/main.rs`, so `sort = relevance` lists it first;
`src/main.rs` is the newest file with `TODO` after `src/parser.rs`. Times are UTC.

### spec-examples-warnings

```text tree
config/dev.env               2026-10-01 10:00
data/dump.sql                2026-10-01 10:00
secrets/prod.env             2026-10-01 10:00
secrets/stage.env            2026-10-01 10:00
src/main.rs                  2026-10-01 10:00
```

```text setup
file data/dump.sql 10485761
file secrets/prod.env 32
file secrets/stage.env 32
chmod secrets/prod.env 000
chmod secrets/stage.env 000
```

File contents are in `tests/fixtures/spec-examples-warnings/`: `config/dev.env` has `DB_password=changeme` on line 3.
The setup adds `data/dump.sql`, one byte over the default `traversal.max_file_size` of 10M, and the two files of
`secrets/` without any permissions, so the example runs only where permissions can be removed (Unix, not as root).
Times are UTC.

### spec-config-request

```text tree
logs/app.log.gz            2026-10-01 10:00
src/api.rs                 2026-10-01 10:00
src/client.rs              2026-10-01 10:00
target/debug/bindings.rs   2026-10-01 10:00
vendor/acme/log/log.go     2026-10-01 10:00
```

```text setup
bytes target/debug/bindings.rs 235b646570726563617465645d0a70756220666e206f6c645f62696e64696e672829207b7d0a
```

File contents are in `tests/fixtures/spec-config-request/`. `logs/app.log.gz` is a gzip compressed log with the line
`2026-10-01 10:05 WARN DEPRECATED endpoint /v1/users called`; `target/debug/bindings.rs` (`#[deprecated]`) is
created by the setup, because git ignores `target` and `debug` directories. Times are UTC.

### spec-config-home

```text tree
logs/my-app/2026-09.zip                                      2026-10-01 00:00
logs/my-app/app.log                                          2026-10-08 10:05
logs/my-app/app.log.1.gz                                     2026-10-08 00:00
projects/my-app/Cargo.lock                                   2026-10-01 10:00
projects/my-app/Cargo.toml                                   2026-10-01 10:00
projects/my-app/src/main.rs                                  2026-10-01 10:00
projects/my-app/target/debug/build/my-app/out/version.rs     2026-10-01 10:00
work/web/coverage/lcov-report/app.ts.html                    2026-10-01 10:00
work/web/dist/main.js                                        2026-10-01 10:00
work/web/node_modules/react/index.js                         2026-10-01 10:00
work/web/package.json                                        2026-10-01 10:00
work/web/src/app.ts                                          2026-10-01 10:00
work/web/src/chart.js                                        2026-10-01 10:00
```

```text setup
bytes projects/my-app/target/debug/build/my-app/out/version.rs 70756220636f6e73742056455253494f4e3a2026737472203d2022302e332e30223b0a
```

A home directory with a Rust project, logs and a frontend; file contents are in `tests/fixtures/spec-config-home/`.
`logs/my-app/app.log.1.gz` is a gzip compressed log with the line
`2026-10-07 23:59 ERROR payment failed: card declined`; `logs/my-app/2026-09.zip` has the member `server.log` with
`2026-09-30 12:01 ERROR payment failed: timeout`, modified 2026-09-30 12:01 UTC. The build output
`projects/my-app/target/debug/build/my-app/out/version.rs` (`pub const VERSION: &str = "0.3.0";`) is created by the
setup, because git ignores `target` and `debug` directories. `work/web/src/chart.js` is one line of 251 characters.
Times are UTC.

### spec-config-session

```text tree
.gitignore                         2026-10-01 10:00
node_modules/some-lib/index.js     2026-10-01 10:00
node_modules/some-lib/strict.js    2026-10-01 10:00
src/app.js                         2026-10-01 10:00
```

```text setup
bytes .gitignore 6e6f64655f6d6f64756c65732f0a
```

File contents are in `tests/fixtures/spec-config-session/`. `.gitignore` (`node_modules/`) is created by the setup,
so git doesn't ignore `node_modules` of the fixture. Times are UTC.

### spec-config-ignore

```text tree
.aiignore                    2026-10-01 10:00
fixtures/large/users.json    2026-10-01 10:00
fixtures/small/user.json     2026-10-01 10:00
secrets/README.md            2026-10-01 10:00
secrets/prod.env             2026-10-01 10:00
web/app.js                   2026-10-01 10:00
web/app.min.js               2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-config-ignore/`; `.aiignore` is the one of CONFIG.md#ignore-files. Times
are UTC.

### spec-config-kinds

```text tree
Cargo.lock                 2026-10-01 10:00
api/api.pb.go              2026-10-01 10:00
api/handler.go             2026-10-01 10:00
api/handler_test.go        2026-10-01 10:00
e2e/login.ts               2026-10-01 10:00
src/gen/api.rs             2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
tests/integration.rs       2026-10-01 10:00
web/button.test.tsx        2026-10-01 10:00
web/button.tsx             2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-config-kinds/`; no file has a generated marker. Times are UTC.

### spec-config-languages

```text tree
infra/main.tf              2026-10-01 10:00
infra/prod.tfvars          2026-10-01 10:00
src/main.rs                2026-10-01 10:00
templates/page.tpl         2026-10-01 10:00
web/index.html             2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-config-languages/`. Times are UTC.

### spec-config-legacy

```text tree
src/app/report.py          2026-10-01 10:00
work/legacy/db/report.sql  2026-10-01 10:00
```

File contents are in `tests/fixtures/spec-config-legacy/`: `work/legacy/db/report.sql` is in windows-1251
(`-- Отчёт по продажам`, not valid UTF-8), `src/app/report.py` in UTF-8. Times are UTC.

## E121.1 README.md: the query of the overview

The first query of README.md, with its line break.

Fixture: `spec-readme-query`

```json request
{ "query": "name ~ '*.rs' and modified > -7d and not path ~ '**/tests/**'\nand line(text has 'unwrap' /i and not text starts '//')" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name ~ '*.rs'
#2 modified > -7d
#3 path ~ '**/tests/**'
#4 line(text has 'unwrap' /i and not text starts '//')
#5 text has 'unwrap' /i
#6 text starts '//'

src/
main.rs  [#1 #2 #4]  lines: 2  matches: 2
⇥4
8:let cfg = config::load(&path).⟦#4,#5:unwrap⟧();
9:let port = cfg.port.⟦#4,#5:unwrap⟧_or(8080);
```

## E121.2 README.md: the request

The request of README.md#request and its response, as they are.

Fixture: `spec-readme-request`

```json request
{
  "dirs": ["."],
  "query": "name ~ '*.rs' and line(text has 'unwrap(' /c @unwrap or text has 'expect(' /c @expect)",
  "traversal": { "exclude": ["**/target/**"] },
  "output": { "context": 1, "max_files": 20 }
}
```

```text response
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' /c or text has 'expect(' /c)
@unwrap text has 'unwrap(' /c
@expect text has 'expect(' /c

src/
config.rs  [#1 #2]  lines: 2  matches: 2
⇥4
41-let raw = fs::read_to_string(path)?;
42:let cfg: Config = toml::from_str(&raw).⟦#2,@unwrap:unwrap(⟧);
43:let port = cfg.port.⟦#2,@expect:expect(⟧"port is required");
44-Ok(cfg)
main.rs  [#1 #2]  lines: 1  matches: 1
⇥4
17-let args = Args::parse();
18:let cfg = config::load(&args.config).⟦#2,@unwrap:unwrap(⟧);
19-run(cfg)
```

## E121.3 TOOLS.md: Results

The result of TOOLS.md#results: one text content item, `isError: false`, no `structuredContent`.

Fixture: `spec-tools-results`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'TODO remove'"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {
  "content": [ { "type": "text", "text": "files: 1  lines: 1  matches: 1  result: r1\n#1 text has 'TODO remove'\n\nsrc/\nmain.rs  [#1]  lines: 1  matches: 1\n12:// ⟦#1:TODO remove⟧\n" } ],
  "isError": false
}}
```

```json never
{"id": 2, "result": {"structuredContent": "*"}}
```

## E121.4 TOOLS.md: Errors, a tool error

The tool error of TOOLS.md#errors: a normal result with `isError: true`, not a JSON-RPC error.

Fixture: `spec-tools-results`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name ~ '*.rs' and text contains 'x'"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {
  "content": [ { "type": "text", "text": "error[unknown_operator]: unknown operator 'contains'\n  name ~ '*.rs' and text contains 'x'\n                         ^^^^^^^^\n  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~\n" } ],
  "isError": true
}}
```

```json never
{"id": 2, "error": "*"}
```

## E121.5 TOOLS.md: Errors, tool errors and protocol errors

The two kinds of TOOLS.md#errors: an unknown tool and a malformed JSON-RPC request are JSON-RPC errors; input which
doesn't match the schema is a tool error (`invalid_option`).

Fixture: `spec-tools-results`

```text command
easy-peasy-mcp
```

1. An unknown tool

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "grep", "arguments": {"query": "TODO"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "error": {"code": "*", "message": "*"}}
```

2. Malformed JSON-RPC: `params` is not an object

```json send
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": "find"}
```

```json receive
{"jsonrpc": "2.0", "error": {"code": "*", "message": "*"}}
```

3. Input which doesn't match the schema

```json send
{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "find", "arguments": {"query": 5}}}
```

```json receive
{"jsonrpc": "2.0", "id": 4, "result": {"content": [{"type": "text", "text": "error[invalid_option]: 'query' must be a string\n"}], "isError": true}}
```

## E121.6 TOOLS.md: Progress

The request of TOOLS.md#progress as it is. The search takes a few seconds, so progress notifications come with its
token: the number of scanned files, the files matched so far and the time with one decimal; no `total`.

Fixture: `spec-tools-progress`

Slow reads: `300ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{ "jsonrpc": "2.0", "id": 7, "method": "tools/call",
  "params": { "name": "find", "arguments": { "query": "text has 'req-7f3a9c'" }, "_meta": { "progressToken": "p7" } } }
```

```json receive
{ "jsonrpc": "2.0", "method": "notifications/progress",
  "params": { "progressToken": "p7", "progress": "*", "message": "scanned * files, * matched, *.*s" } }
```

```json receive
{"jsonrpc": "2.0", "id": 7, "result": {"content": [{"type": "text", "text": "files: 3  lines: 3  matches: 3  result: r1\n#1 text has 'req-7f3a9c'\n\nlogs/\nsearch-1.log  [#1]  lines: 1  matches: 1\n1:2026-10-07T10:01:12Z WARN ⟦#1:req-7f3a9c⟧ slow query: 41ms\napi-2.log  [#1]  lines: 1  matches: 1\n1:2026-10-07T10:01:10Z INFO ⟦#1:req-7f3a9c⟧ POST /search 200 41ms\nworker-1.log  [#1]  lines: 1  matches: 1\n1:2026-10-07T10:01:13Z ERROR ⟦#1:req-7f3a9c⟧ index lock held for 40ms\n"}], "isError": false}}
```

```json never
{"method": "notifications/progress", "params": {"total": "*"}}
```

## E121.7 TOOLS.md: Cancellation

The cancellation of TOOLS.md#cancellation as it is, for the search of request 7. The search stops: the next request
gets the only search thread and is answered at once, and request 7 gets no response.

Fixture: `spec-tools-progress`

Slow reads: `2000ms`

```toml conf/o.toml
[server]
threads = 1
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json send
{"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "text has 'req-7f3a9c'"}}}
```

```json send
{ "jsonrpc": "2.0", "method": "notifications/cancelled", "params": { "requestId": 7, "reason": "user aborted" } }
```

```json send
{"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "find", "arguments": {"query": "name = 'api-2.log'"}}}
```

```json receive
{"jsonrpc": "2.0", "id": 8, "result": {"content": [{"type": "text", "text": "files: 1  lines: 0  matches: 0  result: r1\n#1 name = 'api-2.log'\n\nlogs/\napi-2.log  [#1]\n"}], "isError": false}}
```

```json never
{"id": 7}
```

## E121.8 TOOLS.md: Session example

The session of TOOLS.md#session-example with its requests as they are: count first, then the five files with the
most `unwrap(` calls, then the lines with context in one directory. The responses which the spec only sums up are
checked for what the summary says.

Fixture: `spec-tools-session`

```text command
easy-peasy-mcp
```

1. `initialize` ← `serverInfo easy-peasy-mcp 0.1.0, capabilities: tools`

```json receive
{"jsonrpc": "2.0", "id": 1, "result": {"capabilities": {"tools": {"listChanged": false}}, "serverInfo": {"name": "easy-peasy-mcp", "version": "*"}}}
```

2. `tools/list` ← `find, get_config, update_config`

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find"}, {"name": "get_config"}, {"name": "update_config"}]}}
```

3. `get_config {}` ← the allowed roots (the fixture), the defaults, spec 1

```json send
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_config", "arguments": {}}}
```

```json receive
{"jsonrpc": "2.0", "id": 3, "result": {"content": [{"type": "text", "text": "# easy-peasy-mcp *, spec 1, supported specs: 1\n# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size\n\n[server]\nallowed_roots = ['*']\ndefault_dirs = ['.']\n*"}], "isError": false}}
```

4. Count ← `files: 14  lines: 52  matches: 57  result: r1`

```json send
{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "find", "arguments": { "query": "lang = rust and text has 'unwrap(' /c", "output": { "detail": "count" } }}}
```

```json receive
{"jsonrpc": "2.0", "id": 4, "result": {"content": [{"type": "text", "text": "files: 14  lines: 52  matches: 57  result: r1\n"}], "isError": false}}
```

5. The files of `r1` ← 5 files with the most unwraps, `truncated: yes (max_files)`, `result: r2`

```json send
{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "find", "arguments": { "within": "r1", "query": "text has 'unwrap(' /c", "output": { "detail": "files", "sort": "matches desc", "max_files": 5 } }}}
```

```json receive
{"jsonrpc": "2.0", "id": 5, "result": {"content": [{"type": "text", "text": "files: 14  lines: 52  matches: 57  result: r2  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiNzJiMjI3NmQiLCJmIjo1LCJ0IjpbMTQsNTIsNTddfQ\n#1 text has 'unwrap(' /c\n\nsrc/eval/\nmod.rs  [#1]  lines: 8  matches: 10\nsrc/\nconfig.rs  [#1]  lines: 7  matches: 8\ncache.rs  [#1]  lines: 6  matches: 7\nsrc/server/\nhandlers.rs  [#1]  lines: 5  matches: 6\nsrc/\nmain.rs  [#1]  lines: 5  matches: 5\n"}], "isError": false}}
```

6. `src/parser` ← matched lines with context

```json send
{"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "find", "arguments": { "dirs": ["src/parser"], "query": "text has 'unwrap(' /c", "output": { "context": 2 } }}}
```

```json receive
{"jsonrpc": "2.0", "id": 6, "result": {"content": [{"type": "text", "text": "files: 2  lines: 3  matches: 3  result: r3\n#1 text has 'unwrap(' /c\n\nsrc/parser/\nlexer.rs  [#1]  lines: 2  matches: 2\n⇥8\n11-    continue;\n12-}\n13:let kind = TokenKind::of(c).⟦#1:unwrap(⟧);\n14-let end = chars.peek().map_or(query.len(), |&(i, _)| i);\n15-tokens.push(Token { kind, start, end });\n--\n21-fn number(text: &str) -> u64 {\n22-    // Digits only, checked by the caller, so unwrap() can't fail.\n23:    text.parse().⟦#1:unwrap(⟧)\n24-}\nast.rs  [#1]  lines: 1  matches: 1\n⇥4\n15-    match self {\n16-        Node::Not(inner) => inner,\n17:        _ => self.children().first().⟦#1:unwrap(⟧),\n18-    }\n19-}\n"}], "isError": false}}
```

## E121.9 TOOLS.md: the `set` examples of the `update_config` description

Both forms of the description, `"output.context = 2"` and `"[output]\ncontext = 2\nmax_files = 30"`, with
`reset: ["all"]` of the input schema between them; the next `find` uses the changed configuration.

Fixture: `spec-tools-results`

1. A dotted key

```json request update_config
{ "set": "output.context = 2" }
```

```text response
[output]
context = 2    # was 0
```

2. Reset everything

```json request update_config
{ "reset": ["all"] }
```

```text response
[output]
context = 0    # was 2, reset
```

3. A section

```json request update_config
{ "set": "[output]\ncontext = 2\nmax_files = 30" }
```

```text response
[output]
context = 2    # was 0
max_files = 30    # was 100
```

4. `find` with two lines of context; the empty line 11 is left out (`blank_context = false`)

```json request
{ "query": "text has 'TODO remove'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'TODO remove'

src/
main.rs  [#1]  lines: 1  matches: 1
10-}
12:// ⟦#1:TODO remove⟧
13-fn legacy_main() {
14-    println!("hello from the old entry point");
```

## E121.10 TOOLS.md: the `find` description, `name ~ '*.py' and text has 'import requests'`

Fixture: `spec-tools-examples`

```json request
{ "query": "name ~ '*.py' and text has 'import requests'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ '*.py'
#2 text has 'import requests'

scripts/
fetch.py  [#1 #2]  lines: 1  matches: 1
3:⟦#2:import requests⟧
```

## E121.11 TOOLS.md: the `find` description, `not text has 'license' /i`

Files without a match: no conditions and no lines are listed for them.

Fixture: `spec-tools-examples`

```json request
{ "query": "not text has 'license' /i" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 text has 'license' /i

scripts/
report.py
docs/
config-loader.md
data/
events.json
users.json
```

## E121.12 TOOLS.md: the `find` description, `name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')`

`any` as a word in a line which is not a `//` comment: not in `company`, not in the comment of `web/app.tsx`.

Fixture: `spec-tools-examples`

```json request
{ "query": "name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')" }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 name ~ '*.{ts,tsx}'
#2 line(text has 'any' /w and not text starts '//')
#3 text has 'any' /w
#4 text starts '//'

web/
app.tsx  [#1 #2]  lines: 1  matches: 1
5:export function App(props: ⟦#2,#3:any⟧) {
api.ts  [#1 #2]  lines: 1  matches: 1
7:export function parse(raw: string): ⟦#2,#3:any⟧ {
```

## E121.13 TOOLS.md: the `find` description, ``text has `"role": "admin"` /e(backslash, any)``

Also inside stringified JSON: `data/events.json` has it with one level of backslash escaping, `data/users.json` as it
is.

Fixture: `spec-tools-examples`

```json request
{ "query": "text has `\"role\": \"admin\"` /e(backslash, any)" }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has `"role": "admin"` /e(backslash, any)

data/
events.json  [#1]  lines: 1  matches: 1
⇥2
2:{"type": "login", "user": 7, "payload": "{⟦#1/e1:\"role\": \"admin\"⟧, \"ip\": \"10.0.0.7\"}"},
users.json  [#1]  lines: 1  matches: 1
⇥2
2:{"id": 7, "name": "ann", ⟦#1/e0:"role": "admin"⟧},
```

## E121.14 TOOLS.md: the `find` description, `kind = source and name has 'config loader' /n`

A file by a vague name: `config_loader.rs` and `configLoader.ts`; `config-loader.md` is not source.

Fixture: `spec-tools-examples`

```json request
{ "query": "kind = source and name has 'config loader' /n" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 kind = source
#2 name has 'config loader' /n

src/
config_loader.rs  [#1 #2]
web/
configLoader.ts  [#1 #2]
```

## E121.15 TOOLS.md: the `find` description, `git = changed and text has 'dbg!('`

Leftovers in files with uncommitted changes: `src/main.rs`, not the committed `dbg!(` of `src/lib.rs`.

Fixture: `spec-tools-examples`

```json request
{ "query": "git = changed and text has 'dbg!('" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 git = changed
#2 text has 'dbg!('

src/
main.rs  [#1 #2]  lines: 1  matches: 1
⇥4
6:⟦#2:dbg!(⟧&config);
```

## E121.20 QUERY.md recipes: like `find`

Every query of the table QUERY.md#like-find.

Fixture: `spec-query-find`

1. Files by extension:

```json request
{ "query": "name ~ '*.rs'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'

src/
main.rs  [#1]
vendor/
lib.rs  [#1]
```

2. Several extensions; `src/view.jsx` is left out:

```json request
{ "query": "name ~ '*.{ts,tsx}'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r2
#1 name ~ '*.{ts,tsx}'

src/
app.ts  [#1]
view.tsx  [#1]
```

3. Case-insensitive name:

```json request
{ "query": "name ~ 'readme*' /i", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r3
#1 name ~ 'readme*' /i

./
README.md  [#1]
docs/
readme.txt  [#1]
```

4. By path; `db/seed.sql` is not in `migrations`:

```json request
{ "query": "path ~ '**/migrations/*.sql'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r4
#1 path ~ '**/migrations/*.sql'

db/migrations/
001_init.sql  [#1]
```

5. Empty directories:

```json request
{ "query": "type = dir and empty" }
```

```text response
files: 1  lines: 0  matches: 0  result: r5
#1 type = dir
#2 empty

./
tmp/  [#1 #2]
```

6. Big files:

```json request
{ "query": "size > 100M" }
```

```text response
files: 1  lines: 0  matches: 0  result: r6
#1 size > 100M

data/
dump.bin  [#1]
```

7. Changed in the last 24 hours:

```json request
{ "query": "modified > -1d" }
```

```text response
files: 1  lines: 0  matches: 0  result: r7
#1 modified > -1d

./
README.md  [#1]
```

8. Not changed for a year:

```json request
{ "query": "modified < -365d" }
```

```text response
files: 1  lines: 0  matches: 0  result: r8
#1 modified < -365d

docs/
readme.txt  [#1]
```

9. Symlinks; without `follow_symlinks` the link is not a `file`, so the other queries leave it out:

```json request
{ "query": "type = link" }
```

```text response
files: 1  lines: 0  matches: 0  result: r9
#1 type = link

./
latest.sql  [#1]
```

10. Everything except a directory; a file which matches only through `not` has no conditions:

```json request
{ "query": "not path starts 'vendor/'", "output": { "sort": "path asc" } }
```

```text response
files: 9  lines: 0  matches: 0  result: r10
#1 path starts 'vendor/'

./
README.md
data/
dump.bin
db/migrations/
001_init.sql
db/
seed.sql
docs/
readme.txt
src/
app.ts
main.rs
view.jsx
view.tsx
```

## E121.21 QUERY.md recipes: like `grep`

Every query of the table QUERY.md#like-grep.

Fixture: `spec-query-grep`

1. Fixed string:

```json request
{ "query": "text has 'TODO'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'TODO'

./
notes.txt  [#1]  lines: 1  matches: 1
1:⟦#1:TODO⟧: write the docs
src/
main.py  [#1]  lines: 1  matches: 1
3:# ⟦#1:TODO⟧: read the port from the environment
```

2. Case-insensitive whole word: not `error_code` and not `errors`:

```json request
{ "query": "text has 'error' /i/w", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r2
#1 text has 'error' /i/w

./
notes.txt  [#1]  lines: 1  matches: 1
2:error_code 42 is not an ⟦#1:error⟧
src/
util.py  [#1]  lines: 1  matches: 1
⇥4
4:print("⟦#1:Error⟧:", message, file=sys.stderr)
```

3. Regex: `fn parse_test`, not `fn test_parse`:

```json request
{ "query": "text =~ `fn \\w+_test`" }
```

```text response
files: 1  lines: 1  matches: 1  result: r3
#1 text =~ `fn \w+_test`

src/
parse.rs  [#1]  lines: 1  matches: 1
2:⟦#1:fn parse_test⟧() {}
```

4. Files without a match:

```json request
{ "query": "not text has 'license' /i", "output": { "sort": "path asc" } }
```

```text response
files: 4  lines: 0  matches: 0  result: r4
#1 text has 'license' /i

./
app.conf
notes.txt
src/
main.py
util.py
```

5. Lines without a match:

```json request
{ "query": "name = 'app.conf' and line(not text starts '#')" }
```

```text response
files: 1  lines: 2  matches: 2  result: r5
#1 name = 'app.conf'
#2 line(not text starts '#')
#3 text starts '#'

./
app.conf  [#1 #2]  lines: 2  matches: 2
2:⟦#2:listen = 8080⟧
--
4:⟦#2:log_level = debug⟧
```

6. Only in some files:

```json request
{ "query": "name ~ '*.py' and text has 'import os'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r6
#1 name ~ '*.py'
#2 text has 'import os'

src/
main.py  [#1 #2]  lines: 1  matches: 1
1:⟦#2:import os⟧
```

## E121.22 QUERY.md recipes: both words on the same line, both words in the file

QUERY.md#beyond-grep-and-find.

Fixture: `spec-query-words`

1. Both words on the same line; `src/state.rs` has them on different lines:

```json request
{ "query": "line(text has 'Mutex' and text has 'async')" }
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 line(text has 'Mutex' and text has 'async')
#2 text has 'Mutex'
#3 text has 'async'

src/
cache.rs  [#1]  lines: 1  matches: 2
3:pub ⟦#1,#3:async⟧ fn refresh(cache: &⟦#1,#2:Mutex⟧<Vec<u8>>) {
```

2. Both words in the file, any lines:

```json request
{ "query": "text has 'tokio' and text has 'rayon'" }
```

```text response
files: 1  lines: 2  matches: 2  result: r2
#1 text has 'tokio'
#2 text has 'rayon'

src/
pool.rs  [#1 #2]  lines: 2  matches: 2
⇥4
2:let (a, b) = ⟦#2:rayon⟧::join(|| 1, || 2);
3:let runtime = ⟦#1:tokio⟧::runtime::Runtime::new().unwrap();
```

## E121.23 QUERY.md recipe: Rust files which mention exactly one of two logging libraries

QUERY.md#beyond-grep-and-find. `src/bridge.rs` mentions both and is left out; every file lists the branch of `xor`
which matched.

Fixture: `spec-query-words`

```json request
{ "query": "name ~ '*.rs' and (text has 'log::' xor text has 'tracing::')", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 name ~ '*.rs'
#2 text has 'log::'
#3 text has 'tracing::'

src/
server.rs  [#1 #2]  lines: 1  matches: 1
⇥4
2:⟦#2:log::⟧info!("server started");
worker.rs  [#1 #3]  lines: 1  matches: 1
⇥4
2:⟦#3:tracing::⟧info!("worker started");
```

## E121.24 QUERY.md recipe: JavaScript with very long lines

QUERY.md#beyond-grep-and-find. The `kind` condition brings back the generated `vendor.min.js`.

Fixture: `spec-query-js`

```json request
{ "query": "lang = javascript and kind in (source, generated) and line(len > 500)", "output": { "detail": "files", "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 lang = javascript
#2 kind in (source, generated)
#3 line(len > 500)
#4 len > 500

web/
bundle.js  [#1 #2 #3]  lines: 1  matches: 1
vendor.min.js  [#1 #2 #3]  lines: 1  matches: 1
```

## E121.25 QUERY.md recipe: Rust files which don't end with a line break

QUERY.md#beyond-grep-and-find.

Fixture: `spec-query-rust`

```json request
{ "query": "name ~ '*.rs' and not text ends '\\n'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 text ends '\n'

src/
lib.rs  [#1]
```

## E121.26 QUERY.md recipe: a test function whose name ends with `_slow`

QUERY.md#beyond-grep-and-find. In `src/eval.rs` a comment is between `#[test]` and the function.

Fixture: `spec-query-rust`

```json request
{ "query": "name ~ '*.rs' and text =~ `#\\[test\\]\\s*\\n\\s*fn \\w+_slow` /m" }
```

```text response
files: 1  lines: 2  matches: 1  result: r1
#1 name ~ '*.rs'
#2 text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m

tests/
eval.rs  [#1 #2]  lines: 2  matches: 1
6:⟦#2:#[test]
7:fn eval_big_input_slow⟧() {
```

## E121.27 QUERY.md recipe: `.unwrap()` and `.expect(` outside of comments, with labels

QUERY.md#beyond-grep-and-find. `src/main.rs` has `.unwrap()` only in a comment, `src/old.rs` is too old,
`tests/eval.rs` is in a `tests` directory.

Fixture: `spec-query-rust`

```json request
{ "query": "name ~ '*.rs'\nand modified > -7d\nand not path ~ '**/tests/**'\nand (text has '.unwrap()' /c @unwrap or text has '.expect(' /c @expect)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name ~ '*.rs'
#2 modified > -7d
#3 path ~ '**/tests/**'
@unwrap text has '.unwrap()' /c
@expect text has '.expect(' /c

src/
config.rs  [#1 #2 @unwrap @expect]  lines: 2  matches: 2
⇥4
4:let raw = fs::read_to_string(path)⟦@unwrap:.unwrap()⟧;
5:let cfg: Config = toml::from_str(&raw)⟦@expect:.expect(⟧"valid config");
```

## E121.28 QUERY.md recipe: a password key with a non-empty value

QUERY.md#beyond-grep-and-find. Line 4 of `config/app.yml` starts with `#`, the password of `config/db.properties` is
empty, `src/login.py` is not a configuration file.

Fixture: `spec-query-secrets`

```json request
{ "query": "name ~ '*.{yml,yaml,env,properties}' and line(text =~ `(?i)password\\s*[:=]\\s*\\S+` and not text starts '#')", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 name ~ '*.{yml,yaml,env,properties}'
#2 line(text =~ `(?i)password\s*[:=]\s*\S+` and not text starts '#')
#3 text =~ `(?i)password\s*[:=]\s*\S+`
#4 text starts '#'

config/
app.yml  [#1 #2]  lines: 1  matches: 1
⇥2
3:⟦#2,#3:password: s3cret⟧
dev.env  [#1 #2]  lines: 1  matches: 1
2:DB_⟦#2,#3:PASSWORD=changeme⟧
```

## E121.29 QUERY.md recipe: the admin role in test fixtures, also inside stringified JSON

QUERY.md#beyond-grep-and-find. `src/roles.json` is not a fixture.

Fixture: `spec-query-roles`

```json request
{ "query": "path ~ '**/fixtures/**' and text has `\"role\": \"admin\"` /e(backslash, any)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 path ~ '**/fixtures/**'
#2 text has `"role": "admin"` /e(backslash, any)

tests/fixtures/
users.json  [#1 #2]  lines: 2  matches: 2
⇥4
3:{ "name": "root", ⟦#2/e0:"role": "admin"⟧ },
--
⇥2
6:"payload": "{⟦#2/e1:\"role\": \"admin\"⟧}"
```

## E121.30 QUERY.md recipe: big log files that mention a request id

QUERY.md#beyond-grep-and-find. Files bigger than `traversal.max_file_size` (default 10M) are not searched by content,
so the request raises it.

Fixture: `spec-query-big-log`

1. With the default `max_file_size` the content of `big/server.log` is not searched:

```json request
{ "query": "name ~ '*.log' and size > 10M and text has 'req-7f3a9c'" }
```

```text response
files: 0  lines: 0  matches: 0  result: r1
funnel:
  name ~ '*.log'                         2
  and size > 10M                         1
  and text has 'req-7f3a9c'              0
hints:
  skipped: 1 too big (traversal.max_file_size)

warning: too_big (1): big/server.log
```

2. With the raised `max_file_size`:

```json request
{ "query": "name ~ '*.log' and size > 10M and text has 'req-7f3a9c'", "traversal": { "max_file_size": "1G" } }
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 name ~ '*.log'
#2 size > 10M
#3 text has 'req-7f3a9c'

big/
server.log  [#1 #2 #3]  lines: 1  matches: 1
⇥4
5:⟦#3:req-7f3a9c⟧
```

## E121.31 QUERY.md recipes: log files, also compressed and archived; only inside archives

QUERY.md#beyond-grep-and-find, with `traversal.archives = all`.

Fixture: `spec-query-logs`

1. Log files that mention the request id, also compressed and archived ones:

```json request
{ "query": "name ~ '*.{log,log.gz}' and text has 'req-7f3a9c'", "traversal": { "archives": "all" }, "output": { "sort": "path asc" } }
```

```text response
files: 4  lines: 4  matches: 4  result: r1
#1 name ~ '*.{log,log.gz}'
#2 text has 'req-7f3a9c'

logs/2026-10.zip!/app/
server.log  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ retried
logs/
app.log  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ done
app.log.gz  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ failed
logs/old.tar.gz!/old/
worker.log  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ queued
```

2. Only inside archives; the compressed `logs/app.log.gz` keeps its own path:

```json request
{ "query": "path ~ '**/*!/**' and text has 'req-7f3a9c'", "traversal": { "archives": "all" }, "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r2
#1 path ~ '**/*!/**'
#2 text has 'req-7f3a9c'

logs/2026-10.zip!/app/
server.log  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ retried
logs/old.tar.gz!/old/
worker.log  [#1 #2]  lines: 1  matches: 1
2:request ⟦#2:req-7f3a9c⟧ queued
```

## E121.32 QUERY.md recipe: the file of a class which loads the configuration

QUERY.md#finding-a-place-by-a-vague-idea. `src/cfg_load.go` is found only by the fuzzy name; the doc and the test are
not `source`.

Fixture: `spec-query-vague`

```json request
{ "query": "kind = source and (name has 'config loader' /n or name ~~ 'cfgload')", "output": { "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 kind = source
#2 name has 'config loader' /n
#3 name ~~ 'cfgload'

src/
ConfigLoader.java  [#1 #2 #3]
cfg_load.go  [#1 #3]
config_loader.rs  [#1 #2 #3]
```

## E121.33 QUERY.md recipe: where a function is defined

QUERY.md#finding-a-place-by-a-vague-idea. The definition of `parse_query_fast` and the calls are left out.

Fixture: `spec-query-vague`

```json request
{ "query": "kind = source and line(text has 'parse query' /n/w and text =~ `\\b(fn|def|func|function)\\b`)", "output": { "sort": "path asc" } }
```

```text response
files: 4  lines: 4  matches: 8  result: r1
#1 kind = source
#2 line(text has 'parse query' /n/w and text =~ `\b(fn|def|func|function)\b`)
#3 text has 'parse query' /n/w
#4 text =~ `\b(fn|def|func|function)\b`

go/
query.go  [#1 #2]  lines: 1  matches: 2
3:⟦#2,#4:func⟧ ⟦#2,#3:ParseQuery⟧(s string) []string {
py/
query.py  [#1 #2]  lines: 1  matches: 2
1:⟦#2,#4:def⟧ ⟦#2,#3:parse_query⟧(text):
src/
query.rs  [#1 #2]  lines: 1  matches: 2
1:pub ⟦#2,#4:fn⟧ ⟦#2,#3:parse_query⟧(input: &str) -> Query {
web/
query.js  [#1 #2]  lines: 1  matches: 2
1:export ⟦#2,#4:function⟧ ⟦#2,#3:parseQuery⟧(text) {
```

## E121.34 QUERY.md recipe: source and test files changed in this branch which still have a TODO

QUERY.md#finding-a-place-by-a-vague-idea. `src/c.rs` is unchanged, `docs/notes.md` is not source or test.

Fixture: `spec-query-branch`

```json request
{ "query": "git = branch and kind in (source, test) and text has 'TODO'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 git = branch
#2 kind in (source, test)
#3 text has 'TODO'

src/
a.rs  [#1 #2 #3]  lines: 1  matches: 1
1:// ⟦#3:TODO⟧: handle errors
tests/
b_test.rs  [#1 #2 #3]  lines: 1  matches: 1
1:// ⟦#3:TODO⟧: more cases
```

## E121.35 QUERY.md errors: the error block

Fixture: `spec-query-grep`

```json request
{ "query": "name ~ '*.rs' and text contains 'x'" }
```

```text error
error[unknown_operator]: unknown operator 'contains'
  name ~ '*.rs' and text contains 'x'
                         ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

## E121.36 QUERY.md errors: one example per code

Every row of the table QUERY.md#errors, in its order, then the two string tokens of the text under the table.

Fixture: `spec-query-grep`

1. `unexpected_token`:

```json request
{ "query": "name ~ '*.rs' text has 'x'" }
```

```text error
error[unexpected_token]: unexpected 'text'
  name ~ '*.rs' text has 'x'
                ^^^^
  hint: missing 'and' / 'or' between conditions?
```

2. `unexpected_end`:

```json request
{ "query": "name ~ '*.rs' and" }
```

```text error
error[unexpected_end]: unexpected end of the query
  name ~ '*.rs' and
                   ^
  hint: a condition is expected after 'and'
```

3. `unknown_target`:

```json request
{ "query": "content has 'x'" }
```

```text error
error[unknown_target]: unknown target 'content'
  content has 'x'
  ^^^^^^^
  hint: did you mean 'text'? expected one of: path, name, type, size, modified, created, accessed, empty, lang, kind, git, text, len
```

4. `unknown_operator`:

```json request
{ "query": "text contains 'x'" }
```

```text error
error[unknown_operator]: unknown operator 'contains'
  text contains 'x'
       ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

5. `operator_not_allowed`:

```json request
{ "query": "text ~ '*foo*'" }
```

```text error
error[operator_not_allowed]: operator '~' is not allowed for 'text'
  text ~ '*foo*'
       ^
  hint: wildcards work on 'path' and 'name'; use 'has'
```

6. `modifier_not_allowed`:

```json request
{ "query": "name = 'a' /m" }
```

```text error
error[modifier_not_allowed]: modifier '/m' is not allowed here
  name = 'a' /m
             ^^
  hint: '/m' works with '=~' on 'text' in file scope
```

7. `unterminated_string`, marked from the opening quote to the end of the query:

```json request
{ "query": "text has 'abc" }
```

```text error
error[unterminated_string]: unterminated string
  text has 'abc
           ^^^^
  hint: closing ' is missing
```

8. `invalid_regex`:

```json request
{ "query": "text =~ `(?<=a)b`" }
```

```text error
error[invalid_regex]: invalid regex
  text =~ `(?<=a)b`
          ^^^^^^^^^
  hint: look-arounds are not supported
```

9. `invalid_wildcard`, marked on the whole string value:

```json request
{ "query": "name ~ '[a-'" }
```

```text error
error[invalid_wildcard]: invalid wildcard '[a-'
  name ~ '[a-'
         ^^^^^
  hint: unclosed '['
```

10. `invalid_size`:

```json request
{ "query": "size > 10MB" }
```

```text error
error[invalid_size]: invalid size '10MB'
  size > 10MB
         ^^^^
  hint: units: B, K, M, G
```

11. `invalid_date`:

```json request
{ "query": "modified > -1y" }
```

```text error
error[invalid_date]: invalid date '-1y'
  modified > -1y
             ^^^
  hint: units: s, m, h, d, w
```

12. `invalid_range`:

```json request
{ "query": "size in 10M..1M" }
```

```text error
error[invalid_range]: invalid range '10M..1M'
  size in 10M..1M
          ^^^^^^^
  hint: the start must not be greater than the end
```

13. `line_target_outside`:

```json request
{ "query": "len > 100" }
```

```text error
error[line_target_outside]: 'len' outside of line( ... )
  len > 100
  ^^^
  hint: 'len' works only inside line( ... ) or near( ... )
```

14. `file_target_inside`:

```json request
{ "query": "line(name ~ '*.rs')" }
```

```text error
error[file_target_inside]: 'name' inside line( ... )
  line(name ~ '*.rs')
       ^^^^
  hint: move 'name' out of line( ... )
```

15. `nested_line`:

```json request
{ "query": "line(line(text has 'a'))" }
```

```text error
error[nested_line]: nested line( ... )
  line(line(text has 'a'))
       ^^^^
  hint: line( ... ) can't be nested
```

16. `unknown_value`:

```json request
{ "query": "kind = tests" }
```

```text error
error[unknown_value]: unknown value 'tests'
  kind = tests
         ^^^^^
  hint: did you mean 'test'? expected one of: source, test, config, doc, generated, other
```

17. `invalid_near_option`:

```json request
{ "query": "near(text has 'a', text has 'b', 3, nearest)" }
```

```text error
error[invalid_near_option]: unknown option 'nearest'
  near(text has 'a', text has 'b', 3, nearest)
                                      ^^^^^^^
  hint: options: any, ordered, all, closest, first
```

18. `conflicting_options`:

```json request
{ "query": "near(text has 'a', text has 'b', 3, any, ordered)" }
```

```text error
error[conflicting_options]: conflicting options 'any' and 'ordered'
  near(text has 'a', text has 'b', 3, any, ordered)
                                           ^^^^^^^
  hint: choose one of: any, ordered
```

19. `invalid_escape_level`:

```json request
{ "query": "text has 'a' /e(backslash, 9)" }
```

```text error
error[invalid_escape_level]: invalid escape level 9
  text has 'a' /e(backslash, 9)
                             ^
  hint: max level is 3 (configuration escape.max_level)
```

20. `duplicate_label`:

```json request
{ "query": "text has 'a' @x or text has 'b' @x" }
```

```text error
error[duplicate_label]: duplicate label '@x'
  text has 'a' @x or text has 'b' @x
                                  ^^
  hint: labels must be unique
```

21. A string token is shown with its own quotes, `'x'`:

```json request
{ "query": "name ~ '*.rs' 'x'" }
```

```text error
error[unexpected_token]: unexpected 'x'
  name ~ '*.rs' 'x'
                ^^^
  hint: missing 'and' / 'or' between conditions?
```

22. and `"x"`:

```json request
{ "query": "name ~ '*.rs' \"x\"" }
```

```text error
error[unexpected_token]: unexpected "x"
  name ~ '*.rs' "x"
                ^^^
  hint: missing 'and' / 'or' between conditions?
```

## E121.37 QUERY.md: the first example

Rust files changed during the last 7 days, outside of `tests` directories, with a line which contains `unwrap` in any
case and doesn't start with `//`: line 1 of `src/main.rs` starts with `//`.

Fixture: `spec-query-rust`

```json request
{ "query": "name ~ '*.rs' and modified > -7d and not path ~ '**/tests/**'\nand line(text has 'unwrap' /i and not text starts '//')" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ '*.rs'
#2 modified > -7d
#3 path ~ '**/tests/**'
#4 line(text has 'unwrap' /i and not text starts '//')
#5 text has 'unwrap' /i
#6 text starts '//'

src/
config.rs  [#1 #2 #4]  lines: 1  matches: 1
⇥4
4:let raw = fs::read_to_string(path).⟦#4,#5:unwrap⟧();
```

## E121.38 QUERY.md basics and values

QUERY.md#basics, QUERY.md#files-and-directories, QUERY.md#sizes and QUERY.md#strings.

Fixture: `spec-query-basics`

1. Keywords are case-insensitive, text values are not: `TODO`, but not `todo`:

```json request
{ "query": "TEXT HAS 'TODO'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 TEXT HAS 'TODO'

notes/
a.txt  [#1]  lines: 1  matches: 1
1:⟦#1:TODO⟧: one
```

2. Directories are returned when asked for:

```json request
{ "query": "type = dir and name = 'migrations'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 type = dir
#2 name = 'migrations'

db/
migrations/  [#1 #2]
```

3. `size = 1.5K` is 1536 bytes:

```json request
{ "query": "size = 1.5K" }
```

```text response
files: 1  lines: 0  matches: 0  result: r3
#1 size = 1.5K

data/
k1_5.bin  [#1]
```

4. `size = 0.3K` matches no file:

```json request
{ "query": "size = 0.3K" }
```

```text response
files: 0  lines: 0  matches: 0  result: r4
funnel:
  size = 0.3K                            0
```

5. The JSON of QUERY.md#strings is the query ``text =~ `\bfoo\(` and name ~ '*.rs'``; `barfoo(` has no word boundary
   before `foo`:

```json request
{ "query": "text =~ `\\bfoo\\(` and name ~ '*.rs'" }
```

```text response
files: 1  lines: 2  matches: 2  result: r5
#1 text =~ `\bfoo\(`
#2 name ~ '*.rs'

src/
lib.rs  [#1 #2]  lines: 2  matches: 2
1:fn ⟦#1:foo(⟧) {}
2:fn bar() { ⟦#1:foo(⟧); }
```

## E121.39 QUERY.md files whose content is not searched

PNG files by the name, text files by the content. The binary files are not searched by content, so `data/blob.bin`
is not returned, although it has `logo`.

Fixture: `spec-query-content`

```json request
{ "query": "name ~ '*.png' or text has 'logo'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 1  matches: 1  result: r1
#1 name ~ '*.png'
#2 text has 'logo'

docs/
brand.md  [#2]  lines: 1  matches: 1
1:Use the ⟦#2:logo⟧ on a white background.
img/
logo.png  [#1]

warning: binary_skipped (2): data/blob.bin, img/logo.png
```

## E121.40 QUERY.md condition scope: file scope and line scope

Every query of QUERY.md#file-scope and QUERY.md#line-scope.

Fixture: `spec-query-scope`

1. Files which contain both `serde` and `tokio`, possibly on different lines:

```json request
{ "query": "text has 'serde' and text has 'tokio'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 3  matches: 4  result: r1
#1 text has 'serde'
#2 text has 'tokio'

./
Cargo.toml  [#1 #2]  lines: 2  matches: 2
2:⟦#1:serde⟧ = "1"
3:⟦#2:tokio⟧ = "1"
src/
main.rs  [#1 #2]  lines: 1  matches: 2
2:use {⟦#1:serde⟧::Deserialize, ⟦#2:tokio⟧::fs};
```

2. Rust files without `SPDX-License-Identifier` anywhere in the file:

```json request
{ "query": "name ~ '*.rs' and not text has 'SPDX-License-Identifier'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name ~ '*.rs'
#2 text has 'SPDX-License-Identifier'

src/
lib.rs  [#1]
```

3. Files with a line which contains both `serde` and `tokio`:

```json request
{ "query": "line(text has 'serde' and text has 'tokio')" }
```

```text response
files: 1  lines: 1  matches: 2  result: r3
#1 line(text has 'serde' and text has 'tokio')
#2 text has 'serde'
#3 text has 'tokio'

src/
main.rs  [#1]  lines: 1  matches: 2
2:use {⟦#1,#2:serde⟧::Deserialize, ⟦#1,#3:tokio⟧::fs};
```

4. Lines of `nginx.conf` which don't start with `#` and are not empty:

```json request
{ "query": "name = 'nginx.conf' and line(not text starts '#' and len > 0)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r4
#1 name = 'nginx.conf'
#2 line(not text starts '#' and len > 0)
#3 text starts '#'
#4 len > 0

conf/
nginx.conf  [#1 #2]  lines: 2  matches: 2
2:⟦#2:worker_processes 4;⟧
--
5:⟦#2:events {}⟧
```

5. Python files where every line fits into 120 characters:

```json request
{ "query": "name ~ '*.py' and not line(len > 120)" }
```

```text response
files: 1  lines: 0  matches: 0  result: r5
#1 name ~ '*.py'
#2 line(len > 120)
#3 len > 120

scripts/
short.py  [#1]
```

6. Several groups are independent, each of them may match a different line:

```json request
{ "query": "line(text has 'BEGIN') and line(text has 'COMMIT')" }
```

```text response
files: 1  lines: 2  matches: 2  result: r6
#1 line(text has 'BEGIN')
#2 text has 'BEGIN'
#3 line(text has 'COMMIT')
#4 text has 'COMMIT'

db/
migrate.sql  [#1 #3]  lines: 2  matches: 2
1:BEGIN;
--
3:COMMIT;
```

## E121.41 QUERY.md comments: matches inside comments are ignored with `/c`

Fixture: `spec-query-comments`

1. The block of QUERY.md#comments: lines 1 and 3 are ignored, line 2 matches before the comment, in line 4 the comment
   marker is inside a string literal:

```json request
{ "query": "text has '.unwrap()' /c" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has '.unwrap()' /c

src/
a.rs  [#1]  lines: 2  matches: 2
2:let y = bar⟦#1:.unwrap()⟧; // ok
--
4:let s = "// ⟦#1:.unwrap()⟧";
```

2. Rust files where `unsafe` appears only in comments, if at all:

```json request
{ "query": "name ~ '*.rs' and not text has 'unsafe' /c", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r2
#1 name ~ '*.rs'
#2 text has 'unsafe' /c

src/
a.rs  [#1]
b.rs  [#1]
```

## E121.42 QUERY.md escaping: one value matches all of these

The three lines of QUERY.md#escaping, each on its level; the fourth line is escaped on level 3.

Fixture: `spec-query-escaped`

```json request
{ "query": "text has `\"type\": \"admin\"` /e(backslash, 0..2)" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 text has `"type": "admin"` /e(backslash, 0..2)

data/
types.txt  [#1]  lines: 3  matches: 3
1:{⟦#1/e0:"type": "admin"⟧}
2:"{⟦#1/e1:\"type\": \"admin\"⟧}"
3:"{⟦#1/e2:\\\"type\\\": \\\"admin\\\"⟧}"
```

## E121.43 QUERY.md escaping: examples

Every query of QUERY.md#examples under QUERY.md#escaping. Every value is also in its file unescaped, which is not
matched on level 1.

Fixture: `spec-query-escaping`

1. `"type": "admin"` in JSON files, also inside stringified JSON nested up to `escape.max_level` levels deep:

```json request
{ "query": "name ~ '*.json' and text has `\"type\": \"admin\"` /e(backslash, any)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name ~ '*.json'
#2 text has `"type": "admin"` /e(backslash, any)

data/
events.json  [#1 #2]  lines: 2  matches: 2
1:{"id": 1, "body": {⟦#2/e0:"type": "admin"⟧}}
2:{"id": 2, "body": "{\"payload\": \"{\\\"data\\\": \\\"{⟦#2/e3:\\\\\\\"type\\\\\\\": \\\\\\\"admin\\\\\\\"⟧}\\\"}\"}"}
```

2. The name inside SQL string literals:

```json request
{ "query": "name ~ '*.sql' and text has `O'Brien` /e(quote_doubling, 1)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 name ~ '*.sql'
#2 text has `O'Brien` /e(quote_doubling, 1)

db/
users.sql  [#1 #2]  lines: 1  matches: 1
2:INSERT INTO users (name) VALUES ('⟦#2/e1:O''Brien⟧');
```

3. An escaped tag in HTML:

```json request
{ "query": "name ~ '*.html' and text has '<script>' /e(xml, 1)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r3
#1 name ~ '*.html'
#2 text has '<script>' /e(xml, 1)

web/
page.html  [#1 #2]  lines: 1  matches: 1
2:<p>Write ⟦#2/e1:&lt;script&gt;⟧ tags at the end of the body.</p>
```

4. `q%3Dhello%20world` or `q%3Dhello+world`:

```json request
{ "query": "text has 'q=hello world' /e(url, 1)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r4
#1 text has 'q=hello world' /e(url, 1)

web/
links.txt  [#1]  lines: 2  matches: 2
1:/redirect?next=%2Fsearch%3F⟦#1/e1:q%3Dhello%20world⟧
2:/redirect?next=%2Fsearch%3F⟦#1/e1:q%3Dhello+world⟧
```

## E121.44 QUERY.md escaping: invalid levels

The errors of QUERY.md#levels.

Fixture: `spec-query-escaping`

1. Levels which are not a number, a range or `any`:

```json request
{ "query": "text has 'a' /e(backslash, all)" }
```

```text error
error[invalid_escape_level]: invalid escape level 'all'
  text has 'a' /e(backslash, all)
                             ^^^
  hint: levels: a number, a range like 0..2, or any
```

2. A level whose variants are too many: `'` has 2¹¹ forms on level 11:

```json request
{ "query": "text has \"it's\" /e(backslash, 11)", "escape": { "max_level": 11 } }
```

```text error
error[invalid_escape_level]: invalid escape level 11
  text has "it's" /e(backslash, 11)
                                ^^
  hint: the escaped forms of this value are too many on level 11, use a lower one
```

## E121.45 QUERY.md proximity

Every query of QUERY.md#proximity.

Fixture: `spec-query-near`

1. Every `open(` with the nearest `close(` after it within 20 lines; `src/reload.py` closes before it opens:

```json request
{ "query": "near(text has 'open(', text has 'close(', 20, ordered, closest)" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 near(text has 'open(', text has 'close(', 20, ordered, closest)
#2 text has 'open('
#3 text has 'close('

src/
copy.py  [#1]  lines: 3  matches: 3
⇥4
2:src = ⟦#1,#2:open(⟧a)
3:dst = ⟦#1,#2:open(⟧b, "w")
--
⇥4
5:dst.⟦#1,#3:close(⟧)
```

2. A lock and an `await` close to each other; in `src/store.rs` they are 6 lines apart:

```json request
{ "query": "near(text has '.lock()', text has '.await', 3)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r2
#1 near(text has '.lock()', text has '.await', 3)
#2 text has '.lock()'
#3 text has '.await'

src/
cache.rs  [#1]  lines: 2  matches: 2
⇥4
2:let mut guard = self.inner⟦#1,#2:.lock()⟧.unwrap();
--
⇥4
4:let value = fetch(key)⟦#1,#3:.await⟧?;
```

3. Ignored tests:

```json request
{ "query": "near(text =~ `^\\s*#\\[test\\]`, text has '#[ignore]', 1)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r3
#1 near(text =~ `^\s*#\[test\]`, text has '#[ignore]', 1)
#2 text =~ `^\s*#\[test\]`
#3 text has '#[ignore]'

tests/
api.rs  [#1]  lines: 2  matches: 2
1:#[test]
2:⟦#1,#3:#[ignore]⟧
```

4. An error check which returns `nil`; `pkg/order.go` returns the error:

```json request
{ "query": "name ~ '*.go' and near(text has 'err != nil', text has 'return nil', 2)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r4
#1 name ~ '*.go'
#2 near(text has 'err != nil', text has 'return nil', 2)
#3 text has 'err != nil'
#4 text has 'return nil'

pkg/
user.go  [#1 #2]  lines: 2  matches: 2
⇥1
3:if ⟦#2,#3:err != nil⟧ {
4:	⟦#2,#4:return nil⟧
```

## E121.46 QUERY.md identifiers: any naming style

QUERY.md#n--any-naming-style.

Fixture: `spec-query-naming`

1. The eight styles of the table; `parse__query` doesn't join the words:

```json request
{ "query": "text has 'parse query' /n" }
```

```text response
files: 1  lines: 8  matches: 8  result: r1
#1 text has 'parse query' /n

src/
styles.txt  [#1]  lines: 8  matches: 8
1:⟦#1:parseQuery⟧
2:⟦#1:ParseQuery⟧
3:⟦#1:parse_query⟧
4:⟦#1:PARSE_QUERY⟧
5:⟦#1:parse-query⟧
6:⟦#1:parse.query⟧
7:⟦#1:parse query⟧
8:⟦#1:parsequery⟧
```

2. With `/w` the whole identifier must match: `userId` and `user_id`, but not `userIdentity`:

```json request
{ "query": "text has 'user id' /n/w" }
```

```text response
files: 1  lines: 2  matches: 2  result: r2
#1 text has 'user id' /n/w

src/
user.rs  [#1]  lines: 2  matches: 2
1:let ⟦#1:userId⟧ = 1;
2:let ⟦#1:user_id⟧ = 2;
```

3. On names; the words of `loader_config.rs` are in the other order:

```json request
{ "query": "name has 'config loader' /n", "output": { "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r3
#1 name has 'config loader' /n

src/
ConfigLoader.java  [#1]
config_loader.rs  [#1]
web/
config-loader.ts  [#1]
```

## E121.47 QUERY.md identifiers: fuzzy names

The block of QUERY.md#--fuzzy.

Fixture: `spec-query-fuzzy`

1. `config_loader.rs`, `ConfigLoader.java`:

```json request
{ "query": "name ~~ 'cfgldr'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~~ 'cfgldr'

src/
ConfigLoader.java  [#1]
config_loader.rs  [#1]
```

2. `src/server/handlers.rs`, `services/http/handler.go`:

```json request
{ "query": "path ~~ 'srv hndl'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r2
#1 path ~~ 'srv hndl'

services/http/
handler.go  [#1]
src/server/
handlers.rs  [#1]
```

## E121.48 QUERY.md labels and condition ids

Fixture: `spec-query-labels`

1. The ids of the table of QUERY.md#labels-and-condition-ids:

```json request
{ "query": "name ~ '*.rs' and line(text has 'unwrap(' @unwrap or text has 'expect(' @expect) @panics", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
@panics line(text has 'unwrap(' or text has 'expect(')
@unwrap text has 'unwrap('
@expect text has 'expect('

src/
config.rs  [#1 @panics]  lines: 2  matches: 2
⇥4
2:let raw = std::fs::read_to_string(path).⟦@panics,@unwrap:unwrap(⟧);
3:let cfg: Config = toml::from_str(&raw).⟦@panics,@expect:expect(⟧"valid TOML");
main.rs  [#1 @panics]  lines: 1  matches: 1
⇥4
3:run(cfg).⟦@panics,@unwrap:unwrap(⟧);
```

2. Only conditions without a label are numbered: the second condition is `#1`:

```json request
{ "query": "text has 'a' @x or text has 'b'", "output": { "detail": "files", "sort": "path asc" } }
```

```text response
files: 2  lines: 6  matches: 13  result: r2
@x text has 'a'
#1 text has 'b'

src/
config.rs  [@x #1]  lines: 3  matches: 9
main.rs  [@x]  lines: 3  matches: 4
```

## E121.49 QUERY.md gotchas

Fixture: `spec-query-gotchas`

1. `path ~ 'tests/**'` matches only the top level `tests` directory:

```json request
{ "query": "path ~ 'tests/**'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path ~ 'tests/**'

tests/
a.rs  [#1]
```

2. `path ~ '**/tests/**'` matches any of them:

```json request
{ "query": "path ~ '**/tests/**'", "output": { "sort": "path asc" } }
```

```text response
files: 2  lines: 0  matches: 0  result: r2
#1 path ~ '**/tests/**'

crates/core/tests/
b.rs  [#1]
tests/
a.rs  [#1]
```

3. `path has '/tests/'` doesn't match `tests/a.rs`, the path has no leading `/`:

```json request
{ "query": "path has '/tests/'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r3
#1 path has '/tests/'

crates/core/tests/
b.rs  [#1]
```

4. ``name =~ `.rs$` `` also matches `furs`:

```json request
{ "query": "name =~ `.rs$`", "output": { "sort": "path asc" } }
```

```text response
files: 5  lines: 0  matches: 0  result: r4
#1 name =~ `.rs$`

crates/core/tests/
b.rs  [#1]
src/
contest.rs  [#1]
furs  [#1]
main.rs  [#1]
tests/
a.rs  [#1]
```

5. ``name =~ `test` `` matches `contest.rs`:

```json request
{ "query": "name =~ `test`" }
```

```text response
files: 1  lines: 0  matches: 0  result: r5
#1 name =~ `test`

src/
contest.rs  [#1]
```

## E121.50 OUTPUT.md Response: the JSON format

Fixture: `spec-output-config`

```json request
{ "query": "name = 'config.rs' and text has 'unwrap(' /c", "output": { "context": 1, "format": "json" } }
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": {
    "#1": "name = 'config.rs'",
    "#2": "text has 'unwrap(' /c"
  },
  "files": [
    {
      "path": "src/config.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1", "#2"],
      "blocks": [
        {
          "indent": 4,
          "lines": [
            { "n": 41, "text": "let raw = fs::read_to_string(path)?;" },
            { "n": 42, "text": "let cfg: Config = toml::from_str(&raw).unwrap();",
              "fragments": [ { "col": [44, 51], "text": "unwrap(", "ids": ["#2"] } ] },
            { "n": 43, "text": "let port = cfg.port.expect(\"port is required\");" }
          ]
        }
      ]
    }
  ]
}
```

## E121.51 OUTPUT.md Response: the text format

Fixture: `spec-output-config`

```json request
{ "query": "name = 'config.rs' and text has 'unwrap(' /c", "output": { "context": 1 } }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap(' /c

src/
config.rs  [#1 #2]  lines: 1  matches: 1
⇥4
41-let raw = fs::read_to_string(path)?;
42:let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
43-let port = cfg.port.expect("port is required");
```

## E121.52 OUTPUT.md Errors: in JSON

Fixture: `spec-output-config`

```json request
{ "query": "name ~ '*.rs' and text contains 'x'", "output": { "format": "json" } }
```

```json error
{
  "error": {
    "code": "unknown_operator",
    "message": "unknown operator 'contains'",
    "query": "name ~ '*.rs' and text contains 'x'",
    "position": { "col": 24, "len": 8 },
    "hint": "did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~"
  }
}
```

## E121.53 OUTPUT.md Errors: in the text format

Fixture: `spec-output-config`

```json request
{ "query": "name ~ '*.rs' and text contains 'x'" }
```

```text error
error[unknown_operator]: unknown operator 'contains'
  name ~ '*.rs' and text contains 'x'
                         ^^^^^^^^
  hint: did you mean 'has'? expected one of: =, has, starts, ends, ~, ~~, =~
```

## E121.54 OUTPUT.md Funnel: the steps

The spec leaves out the result id.

Fixture: `spec-output-funnel`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0"}
```

```text response
[server]
result_cache = 0    # was 50
```

2. `find`

```json request
{"query": "lang = rust and modified > -7d and text has 'parseQuery'"}
```

```text response
files: 0  lines: 0  matches: 0
funnel:
  lang = rust                           12
  and modified > -7d                     5
  and text has 'parseQuery'              0
```

## E121.55 OUTPUT.md Funnel: the hints of an empty result

The spec leaves out the result id; `src/tables.rs` is too big with `max_file_size = "1K"`.

Fixture: `spec-output-hints`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\ntraversal.max_file_size = \"1K\""}
```

```text response
[server]
result_cache = 0    # was 50

[traversal]
max_file_size = "1K"    # was "10M"
```

2. `find`

```json request
{"query": "lang = rust and modified > -7d and text has 'parseQuery'"}
```

```text response
files: 0  lines: 0  matches: 0
funnel:
  lang = rust                           12
  and modified > -7d                     5
  and text has 'parseQuery'              0
hints:
  text has 'parseQuery' /i               2 files: ParseQuery
  text has 'parseQuery' /n               4 files: parse_query, ParseQuery
  skipped: 5 ignored (traversal.ignore_files), 2 generated (traversal.skip_generated), 1 too big (traversal.max_file_size)

warning: too_big (1): src/tables.rs
```

## E121.56 OUTPUT.md Facets

The spec leaves out the result id.

Fixture: `spec-output-facets`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0"}
```

```text response
[server]
result_cache = 0    # was 50
```

2. `find`

```json request
{ "query": "kind in (source, test) and (text has 'unwrap(' @unwrap or text has 'expect(' @expect)", "output": { "detail": "facets" } }
```

```text response
files: 10  lines: 16  matches: 17
#1 kind in (source, test)
@unwrap text has 'unwrap('
@expect text has 'expect('
dir:        tests/integration/ 4 · src/parser/ 3 · tests/unit/ 2 · src/eval/ 1
lang:       rust 9 · python 1
kind:       test 6 · source 4
condition:  #1 10 · @unwrap 7 · @expect 4
```

## E121.57 OUTPUT.md Funnel: the hints of a huge result

Fourteen files are more than 10 × `max_files`. The spec leaves out the result id.

Fixture: `spec-output-huge`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0"}
```

```text response
[server]
result_cache = 0    # was 50
```

2. `find`

```json request
{ "query": "text has 'error'", "output": { "max_files": 1 } }
```

```text response
files: 14  lines: 19  matches: 19  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiYTk4MzkzN2MiLCJmIjoxLCJ0IjpbMTQsMTksMTldfQ
#1 text has 'error'
funnel:
  text has 'error'                      14
hints:
  dir: vendor/tokio/ 5 · vendor/hyper/ 4 · src/parser/ 3 · tests/integration/ 2
  lang: rust 12 · c 2
  narrow with: not path ~ 'vendor/**', kind = source, /w

src/parser/
error.rs  [#1]  lines: 2  matches: 2
1:/// A syntax ⟦#1:error⟧ with its position.
--
7:/// The message of an ⟦#1:error⟧: where and what.
```

## E121.58 OUTPUT.md Result ids: changes since `r12`

The spec shows `dedent = false`. The files don't change between the requests: the differences to `r12` come from
its query, which also matched `expect(` and left out `src/cache.rs`. The other requests only use up result ids.

Fixture: `spec-output-since`

1. `update_config`

```json request update_config
{"set": "output.dedent = false"}
```

```text response
[output]
dedent = false    # was true
```

2. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r1
```

3. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r2
```

4. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r3
```

5. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r4
```

6. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r5
```

7. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r6
```

8. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r7
```

9. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r8
```

10. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r9
```

11. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r10
```

12. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r11
```

13. `find` — the result `r12`

```json request
{"query": "lang = rust and (text has 'unwrap(' /c or text has 'expect(' /c) and not name = 'cache.rs'", "output": {"detail": "files"}}
```

```text response
files: 12  lines: 31  matches: 32  result: r12
#1 lang = rust
#2 text has 'unwrap(' /c
#3 text has 'expect(' /c
#4 name = 'cache.rs'

src/
db.rs  [#1 #2]  lines: 3  matches: 4
auth.rs  [#1 #2]  lines: 3  matches: 3
handlers.rs  [#1 #2]  lines: 3  matches: 3
jobs.rs  [#1 #2]  lines: 2  matches: 2
metrics.rs  [#1 #2]  lines: 3  matches: 3
queue.rs  [#1 #2]  lines: 3  matches: 3
routes.rs  [#1 #2]  lines: 3  matches: 3
server.rs  [#1 #2]  lines: 3  matches: 3
session.rs  [#1 #2]  lines: 3  matches: 3
util.rs  [#1 #2]  lines: 2  matches: 2
config.rs  [#1 #2 #3]  lines: 2  matches: 2
main.rs  [#1 #3]  lines: 1  matches: 1
```

14. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r13
```

15. `find`

```json request
{"query": "lang = rust", "output": {"detail": "count"}}
```

```text response
files: 14  lines: 0  matches: 0  result: r14
```

16. `find` — the differences to `r12`

```json request
{ "since": "r12", "query": "lang = rust and text has 'unwrap(' /c" }
```

```text response
files: 12  lines: 30  matches: 31  result: r15  since: r12  unchanged: 10
#1 lang = rust
#2 text has 'unwrap(' /c

+ src/cache.rs  [#1 #2]  lines: 1  matches: 1
63:        let v = map.get(k).⟦#2:unwrap(⟧);
~ src/config.rs  [#1 #2]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
- src/main.rs
```

## E121.59 OUTPUT.md Batch

The spec shows `group_dirs = false`.

Fixture: `spec-output-batch`

1. `update_config`

```json request update_config
{"set": "output.group_dirs = false"}
```

```text response
[output]
group_dirs = false    # was true
```

2. `find`

```json request
{
  "dirs": ["."],
  "output": { "detail": "files", "budget": 3000 },
  "queries": [
    { "label": "defs", "query": "lang = rust and text =~ `fn\\s+load_config`" },
    { "label": "calls", "query": "lang = rust and text has 'load_config(' and not text =~ `fn\\s+load_config`" },
    { "label": "tests", "query": "kind = test and text has 'load_config' /n", "output": { "detail": "count" } }
  ]
}
```

```text response
== defs ==  files: 1  lines: 1  matches: 1  result: r1
#1 lang = rust
#2 text =~ `fn\s+load_config`

src/config.rs  [#1 #2]  lines: 1  matches: 1
== calls ==  files: 3  lines: 4  matches: 4  result: r2
#1 lang = rust
#2 text has 'load_config('
#3 text =~ `fn\s+load_config`

src/main.rs  [#1 #2]  lines: 1  matches: 1
src/cli.rs  [#1 #2]  lines: 1  matches: 1
src/server/mod.rs  [#1 #2]  lines: 2  matches: 2
== tests ==  files: 2  lines: 5  matches: 5  result: r3
```

## E121.60 OUTPUT.md Warnings: in JSON

The content of every file is needed: the keys can't be opened, `root-only/` can't be listed. The example paths are in
path order.

Fixture: `spec-output-warnings`

```json request
{"query": "text has 'password'", "output": {"format": "json", "detail": "count"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "warnings": [
    {
      "code": "permission_denied",
      "count": 3,
      "paths": [
        "root-only/",
        "secrets/a.key",
        "secrets/b.key"
      ]
    }
  ]
}
```

## E121.61 OUTPUT.md Compact output: dedent

The spec shows the end of the response with `group_dirs = false`.

Fixture: `spec-output-dedent`

```json request
{"query": "text has 'unwrap('", "output": {"context": 1, "group_dirs": false}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'unwrap('

src/server/handlers.rs  [#1]  lines: 1  matches: 1
⇥12
140-let user = session.user()?;
141:let id = user.⟦#1:unwrap(⟧).id;
142-audit(id);
```

## E121.62 OUTPUT.md Condition ids: file conditions

The spec shows the end of the response; the files are in path order.

Fixture: `spec-output-conditions`

```json request
{"query": "name ~ '*.rs' and (text has '.unwrap()' @unwrap or text has '.expect(' @expect)", "output": {"detail": "files", "group_dirs": false, "sort": "path asc"}}
```

```text response
files: 3  lines: 5  matches: 5  result: r1
#1 name ~ '*.rs'
@unwrap text has '.unwrap()'
@expect text has '.expect('

src/a.rs  [#1 @unwrap]  lines: 1  matches: 1
src/b.rs  [#1 @expect]  lines: 2  matches: 2
src/c.rs  [#1 @unwrap @expect]  lines: 2  matches: 2
```

## E121.63 OUTPUT.md Condition ids: a true condition in a false branch

The small `README.md` has a `TODO`, but the branch of `@todo` is false because of `@big`: the file is listed with
`[@md]` only, and `@todo` produces no fragment.

Fixture: `spec-output-conditions`

```json request
{"query": "(text has 'TODO' @todo and size > 1M @big) or name ~ '*.md' @md"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
@todo text has 'TODO'
@big size > 1M
@md name ~ '*.md'

./
README.md  [@md]
```

## E121.64 OUTPUT.md Compact output: directory groups

The spec leaves out the lines of the files (`...`).

Fixture: `spec-output-groups`

```json request
{"query": "text has 'unwrap('"}
```

```text response
files: 3  lines: 4  matches: 4  result: r1
#1 text has 'unwrap('

src/parser/
lexer.rs  [#1]  lines: 2  matches: 2
⇥4
2:let first = text.chars().next().⟦#1:unwrap(⟧);
3:let last = text.chars().last().⟦#1:unwrap(⟧);
ast.rs  [#1]  lines: 1  matches: 1
⇥4
6:Expr::Num(text.parse().⟦#1:unwrap(⟧))
src/eval/
mod.rs  [#1]  lines: 1  matches: 1
⇥4
14:let n: f64 = text.trim().parse().⟦#1:unwrap(⟧);
```

## E121.65 OUTPUT.md Compact output: repeated lines

The line is in all 37 files and they have no other matched line, so no file is printed; the spec shows the end of the
response.

Fixture: `spec-output-many`

```json request
{"query": "text has 'Result'"}
```

```text response
files: 37  lines: 37  matches: 37  result: r1
#1 text has 'Result'

repeated:
  ×37 use anyhow::⟦#1:Result⟧;    src/main.rs:3, src/config.rs:5, src/eval/mod.rs:2, … 34 more
```

## E121.66 OUTPUT.md Fragments: long lines

The spec shows the cut line of the file.

Fixture: `spec-output-long`

```json request
{"query": "text has 'unwrap'", "output": {"format": "json", "max_line_length": 64}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": { "#1": "text has 'unwrap'" },
  "files": [
    { "path": "dist/app.js", "lines": 1, "matches": 1, "conditions": ["#1"],
      "blocks": [ { "lines": [
        { "n": 1, "cut": true, "text": "…var a=require(\"./x\");var b=a.unwrap();var c=require(\"./y\");var d…",
          "fragments": [ { "col": [18342, 18348], "text": "unwrap", "ids": ["#1"] } ] }
      ] } ] }
  ]
}
```

## E121.67 OUTPUT.md Pagination

The first page has 20 of the 21 files with `TODO`; the second request is the one of the spec.

Fixture: `spec-output-many`

1. `find`

```json request
{ "dirs": ["."], "query": "text has 'TODO'", "output": { "max_files": 20 } }
```

```text response
files: 21  lines: 21  matches: 21  result: r1  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiZTJlOTNjN2QiLCJmIjoyMCwidCI6WzIxLDIxLDIxXX0
#1 text has 'TODO'

src/api/
admin.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the admin of api
health.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the health of api
items.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the items of api
orders.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the orders of api
search.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the search of api
users.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the users of api
src/db/
items.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the items of db
migrate.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the migrate of db
orders.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the orders of db
pool.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the pool of db
schema.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the schema of db
users.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the users of db
src/eval/
errors.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the errors of eval
ops.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the ops of eval
scope.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the scope of eval
types.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the types of eval
value.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the value of eval
src/jobs/
cleanup.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the cleanup of jobs
export.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the export of jobs
import.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the import of jobs
```

2. `find` — the next page

```json request
{ "dirs": ["."], "query": "text has 'TODO'", "output": { "max_files": 20, "cursor": "eyJ2IjoxLCJoIjoiZTJlOTNjN2QiLCJmIjoyMCwidCI6WzIxLDIxLDIxXX0" } }
```

```text response
files: 21  lines: 21  matches: 21  result: r2
#1 text has 'TODO'

src/jobs/
mail.rs  [#1]  lines: 1  matches: 1
3:// ⟦#1:TODO⟧: test the mail of jobs
```

## E121.68 OUTPUT.md Condition ids: the legend

The conditions are written as in the query, without the labels and with one space where the query has whitespace.

Fixture: `spec-output-config`

```json request
{"query": "NAME   ~\n'*.rs' and line( text has 'a' @x )", "output": {"detail": "files"}}
```

```text response
files: 1  lines: 21  matches: 47  result: r1
#1 NAME ~ '*.rs'
#2 line( text has 'a' )
@x text has 'a'

src/
config.rs  [#1 #2]  lines: 21  matches: 47
```

## E121.69 OUTPUT.md Options

The spec shows only the request.

Fixture: `spec-output-config`

```json request
{
  "dirs": ["."],
  "query": "name ~ '*.rs' and text has 'unwrap(' /c",
  "output": {
    "format": "text",
    "detail": "lines",
    "context": 2,
    "max_files": 20
  }
}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap(' /c

src/
config.rs  [#1 #2]  lines: 1  matches: 1
⇥4
40-// the file is small, read it at once
41-let raw = fs::read_to_string(path)?;
42:let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
43-let port = cfg.port.expect("port is required");
44-let workers = if cfg.workers == 0 {
```

## E121.70 OUTPUT.md Budget

The spec shows only the request; the response fits into the budget as requested.

Fixture: `spec-output-config`

```json request
{ "query": "lang = rust and text has 'unwrap(' /c", "output": { "budget": 1500 } }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 lang = rust
#2 text has 'unwrap(' /c

src/
config.rs  [#1 #2]  lines: 1  matches: 1
⇥4
42:let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
```

## E121.71 OUTPUT.md Fragments: path fragments with markers

Fixture: `spec-output-config`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'config'

src/
config.rs  [#1]
  path: src/⟦#1:config⟧.rs
```

## E121.72 OUTPUT.md Fragments: path fragments with columns

Fixture: `spec-output-config`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true, "highlight": "columns"}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name has 'config'

src/
config.rs  [#1]
  path @ 5..11 #1
```

## E121.73 OUTPUT.md Fragments: path fragments in JSON

Fixture: `spec-output-config`

```json request
{"query": "name has 'config'", "output": {"path_fragments": true, "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 0, "matches": 0 },
  "legend": { "#1": "name has 'config'" },
  "files": [
    { "path": "src/config.rs", "conditions": ["#1"],
      "path_fragments": [{ "col": [5, 11], "text": "config", "ids": ["#1"] }] }
  ]
}
```

## E121.74 OUTPUT.md Highlight: markers

The spec shows the line with `dedent = false`.

Fixture: `spec-output-config`

```json request
{"query": "name = 'config.rs' and text has 'unwrap(' /c", "output": {"dedent": false}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'config.rs'
#2 text has 'unwrap(' /c

src/
config.rs  [#1 #2]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).⟦#2:unwrap(⟧);
```

## E121.75 OUTPUT.md Highlight: escape levels in the markers

Fixture: `spec-output-highlight`

```json request
{"query": "text has '\"type\": \"admin\"' /e(backslash, any)"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has '"type": "admin"' /e(backslash, any)

data/
users.json  [#1]  lines: 2  matches: 2
1:{⟦#1/e0:"type": "admin"⟧}
2:"{⟦#1/e1:\"type\": \"admin\"⟧}"
```

## E121.76 OUTPUT.md Highlight: columns of merged fragments

`name` is matched by `#2` and `#3`: the identical fragments are one with both ids.

Fixture: `spec-output-highlight`

```json request
{"query": "text has 'user' and (text has 'name' or text =~ `n.me`)", "output": {"highlight": "columns"}}
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'user'
#2 text has 'name'
#3 text =~ `n.me`

src/
user.rs  [#1 #2 #3]  lines: 1  matches: 2
1:let user = request.name;
  @ 5..9 #1  @ 20..24 #2,#3
```

## E121.77 OUTPUT.md Highlight: columns of a multiline fragment

Fixture: `spec-output-highlight`

```json request
{"query": "text =~ `\\{\\n\\};` /m", "output": {"highlight": "columns"}}
```

```text response
files: 1  lines: 2  matches: 1  result: r1
#1 text =~ `\{\n\};` /m

web/
app.js  [#1]  lines: 2  matches: 1
57:const xy = {
  @ 12..58:3 #1
58:};
```

## E121.78 OUTPUT.md Result ids: refine `within`

The request of the spec, on result `r12`: the first requests only use up result ids.

Fixture: `spec-output-many`

1. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r1
```

2. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r2
```

3. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r3
```

4. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r4
```

5. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r5
```

6. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r6
```

7. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r7
```

8. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r8
```

9. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r9
```

10. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r10
```

11. `find`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r11
```

12. `find` — the result `r12`

```json request
{"query": "text has 'Result'", "output": {"detail": "count"}}
```

```text response
files: 37  lines: 37  matches: 37  result: r12
```

13. `find` — only the files of `r12`

```json request
{ "within": "r12", "query": "text has 'retry'" }
```

```text response
files: 1  lines: 2  matches: 2  result: r13
#1 text has 'retry'

src/net/
retry.rs  [#1]  lines: 2  matches: 2
4:pub fn ⟦#1:retry⟧() {}
5:// the end of net::⟦#1:retry⟧
```

## E121.79 OUTPUT.md Compact output: repeated lines in JSON

As in E121.65, no file has another matched line, so the response has no `files`.

Fixture: `spec-output-many`

```json request
{"query": "text has 'Result'", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "repeated": [
    { "count": 37, "text": "use anyhow::Result;",
      "fragments": [ { "col": [13, 19], "text": "Result", "ids": ["#1"] } ],
      "locations": ["src/main.rs:3", "src/config.rs:5", "src/eval/mod.rs:2"] }
  ],
  "counts": { "files": 37, "lines": 37, "matches": 37 },
  "legend": { "#1": "text has 'Result'" }
}
```

## E121.80 OUTPUT.md Examples: does anything match?

[OUTPUT.md#does-anything-match](../OUTPUT.md#does-anything-match): `dbg!(` is only in a comment.

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text has 'dbg!(' /c", "output": { "detail": "exists" } }
```

```text response
exists: no
```

## E121.81 OUTPUT.md Examples: how many?

[OUTPUT.md#how-many](../OUTPUT.md#how-many)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "name ~ '*.rs' and text has 'TODO'", "output": { "detail": "count" } }
```

```text response
files: 7  lines: 12  matches: 12
```

## E121.82 OUTPUT.md Examples: which files

[OUTPUT.md#which-files-like-find--grep--l](../OUTPUT.md#which-files-like-find--grep--l)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "name ~ '*.rs' and size > 20K", "output": { "detail": "files", "meta": true, "sort": "size desc" } }
```

```text response
files: 2  lines: 0  matches: 0
#1 name ~ '*.rs'
#2 size > 20K

src/parser.rs  [#1 #2]  file 48.2K 2026-10-07 18:22 rust source
src/eval.rs  [#1 #2]  file 23.9K 2026-10-02 09:10 rust source
```

## E121.83 OUTPUT.md Examples: lines with context

[OUTPUT.md#lines-with-context-like-grep--n--c1](../OUTPUT.md#lines-with-context-like-grep--n--c1)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{
  "query": "name ~ '*.rs' and line(text has 'unwrap(' @unwrap or text has 'expect(' @expect)",
  "output": { "context": 1 }
}
```

```text response
files: 2  lines: 3  matches: 3
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' or text has 'expect(')
@unwrap text has 'unwrap('
@expect text has 'expect('

src/config.rs  [#1 #2]  lines: 2  matches: 2
41-    let raw = fs::read_to_string(path)?;
42:    let cfg: Config = toml::from_str(&raw).⟦#2,@unwrap:unwrap(⟧);
43:    let port = cfg.port.⟦#2,@expect:expect(⟧"port is required");
44-    Ok(cfg)
src/main.rs  [#1 #2]  lines: 1  matches: 1
17-    let args = Args::parse();
18:    let cfg = config::load(&args.config).⟦#2,@unwrap:unwrap(⟧);
19-    run(cfg)
```

## E121.84 OUTPUT.md Examples: columns instead of markers

[OUTPUT.md#columns-instead-of-markers](../OUTPUT.md#columns-instead-of-markers)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text has 'unwrap(' @unwrap", "output": { "highlight": "columns" } }
```

```text response
files: 2  lines: 2  matches: 2
@unwrap text has 'unwrap('

src/config.rs  [@unwrap]  lines: 1  matches: 1
42:    let cfg: Config = toml::from_str(&raw).unwrap();
  @ 44..51 @unwrap
src/main.rs  [@unwrap]  lines: 1  matches: 1
18:    let cfg = config::load(&args.config).unwrap();
  @ 42..49 @unwrap
```

## E121.85 OUTPUT.md Examples: lines without a match

[OUTPUT.md#lines-without-a-match-like-grep--v](../OUTPUT.md#lines-without-a-match-like-grep--v)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "name = 'app.conf' and line(not text starts '#' and len > 0)" }
```

```text response
files: 1  lines: 2  matches: 2
#1 name = 'app.conf'
#2 line(not text starts '#' and len > 0)
#3 text starts '#'
#4 len > 0

config/app.conf  [#1 #2]  lines: 2  matches: 2
4:⟦#2:listen = 8080⟧
--
9:⟦#2:log_level = debug⟧
```

## E121.86 OUTPUT.md Examples: proximity

[OUTPUT.md#proximity](../OUTPUT.md#proximity): a lock and an `.await` two lines apart.

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "near(text has '.lock()' @lock, text has '.await' @await, 3)", "output": { "context": 0 } }
```

```text response
files: 1  lines: 2  matches: 2
#1 near(text has '.lock()', text has '.await', 3)
@lock text has '.lock()'
@await text has '.await'

src/cache.rs  [#1]  lines: 2  matches: 2
57:        let mut guard = self.inner⟦#1,@lock:.lock()⟧;
--
59:        let value = fetch(key)⟦#1,@await:.await⟧?;
```

## E121.87 OUTPUT.md Examples: escaped JSON

[OUTPUT.md#escaped-json](../OUTPUT.md#escaped-json)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text has `\"role\": \"admin\"` /e(backslash, any)", "output": { "format": "json" } }
```

```json response
{
  "version": 1,
  "exists": true,
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": { "#1": "text has `\"role\": \"admin\"` /e(backslash, any)" },
  "files": [
    {
      "path": "tests/fixtures/users.json",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1"],
      "blocks": [
        { "lines": [
          { "n": 3, "text": "  { \"name\": \"root\", \"role\": \"admin\" },",
            "fragments": [ { "col": [21, 36], "text": "\"role\": \"admin\"", "ids": ["#1"], "escape": 0 } ] }
        ] },
        { "lines": [
          { "n": 8, "text": "  \"payload\": \"{\\\"role\\\": \\\"admin\\\"}\"",
            "fragments": [ { "col": [16, 35], "text": "\\\"role\\\": \\\"admin\\\"", "ids": ["#1"], "escape": 1 } ] }
        ] }
      ]
    }
  ]
}
```

## E121.88 OUTPUT.md Examples: multiline match

[OUTPUT.md#multiline-match](../OUTPUT.md#multiline-match)

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text =~ `#\\[test\\]\\s*\\n\\s*fn \\w+_slow` /m" }
```

```text response
files: 1  lines: 2  matches: 1
#1 text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m

src/eval.rs  [#1]  lines: 2  matches: 1
210:    ⟦#1:#[test]
211:    fn eval_big_input_slow⟧() {
```

## E121.89 OUTPUT.md Examples: truncated

[OUTPUT.md#truncated](../OUTPUT.md#truncated): the first page and the next page, with the cursor of the first one.
The spec shows the cursor this server writes for the request.

Fixture: `spec-examples-rust`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text has 'TODO'", "output": { "detail": "files", "max_files": 2, "sort": "modified desc" } }
```

```text response
files: 7  lines: 12  matches: 12  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiODVjMzMxMTUiLCJmIjoyLCJ0IjpbNywxMiwxMl19
#1 text has 'TODO'

src/parser.rs  [#1]  lines: 3  matches: 3
src/main.rs  [#1]  lines: 1  matches: 1
```

3. `find` — the next page, with a cursor of its own: `max_files` cuts it too

```json request
{
  "query": "text has 'TODO'",
  "output": { "detail": "files", "max_files": 2, "sort": "modified desc", "cursor": "eyJ2IjoxLCJoIjoiODVjMzMxMTUiLCJmIjoyLCJ0IjpbNywxMiwxMl19" }
}
```

```text response
files: 7  lines: 12  matches: 12  truncated: yes (max_files)  next cursor: eyJ2IjoxLCJoIjoiODVjMzMxMTUiLCJmIjo0LCJ0IjpbNywxMiwxMl19
#1 text has 'TODO'

src/cache.rs  [#1]  lines: 2  matches: 2
src/lexer.rs  [#1]  lines: 2  matches: 2
```

## E121.90 OUTPUT.md Examples: with warnings

[OUTPUT.md#with-warnings](../OUTPUT.md#with-warnings): the two files of `secrets/` can't be opened, `data/dump.sql`
is too big.

Fixture: `spec-examples-warnings`

1. `update_config`

```json request update_config
{"set": "server.result_cache = 0\noutput.dedent = false\noutput.group_dirs = false"}
```

```text response
[server]
result_cache = 0    # was 50

[output]
dedent = false    # was true
group_dirs = false    # was true
```

2. `find`

```json request
{ "query": "text has 'password'" }
```

```text response
files: 1  lines: 1  matches: 1
#1 text has 'password'

config/dev.env  [#1]  lines: 1  matches: 1
3:DB_⟦#1:password⟧=changeme

warning: permission_denied (2): secrets/prod.env, secrets/stage.env
warning: too_big (1): data/dump.sql
```

## E121.100 CONFIG.md Where the values come from: request fields

The fields of the request replace the configuration for this request only: `target` and `vendor` are excluded, the
compressed log is searched. The same query without them searches `target` and `vendor` and skips the compressed log
as a binary file.

Fixture: `spec-config-request`

1. The request of the spec

```json request
{
  "dirs": ["."],
  "query": "text has 'deprecated' /i",
  "timeout": "5s",
  "traversal": { "exclude": ["**/target/**", "**/vendor/**"], "archives": "compressed" },
  "output": { "max_files": 10 }
}
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'deprecated' /i

src/
client.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:Deprecated⟧: kept for old clients
logs/
app.log.gz  [#1]  lines: 1  matches: 1
2:2026-10-01 10:05 WARN ⟦#1:DEPRECATED⟧ endpoint /v1/users called
src/
api.rs  [#1]  lines: 1  matches: 1
3:#[⟦#1:deprecated⟧(note = "use fetch_v2")]
```

2. Without the fields

```json request
{ "query": "text has 'deprecated' /i" }
```

```text response
files: 4  lines: 4  matches: 4  result: r2
#1 text has 'deprecated' /i

src/
client.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:Deprecated⟧: kept for old clients
target/debug/
bindings.rs  [#1]  lines: 1  matches: 1
1:#[⟦#1:deprecated⟧]
vendor/acme/log/
log.go  [#1]  lines: 1  matches: 1
1:// ⟦#1:Deprecated⟧: use log/slog.
src/
api.rs  [#1]  lines: 1  matches: 1
3:#[⟦#1:deprecated⟧(note = "use fetch_v2")]

warning: binary_skipped (1): logs/app.log.gz
```

## E121.101 CONFIG.md Command line: the registration in an MCP client

The arguments of the registration: `--root` replaces the allowed roots, `~` is expanded by the server (an MCP client
starts it without a shell). The start directory is no longer a root.

Fixture: `spec-config-home`

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --root ~/projects
```

1. `get_config`

```json request get_config
{ "section": "server", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>/projects']    # cli
default_dirs = ['.']    # default
timeout = "30s"    # default
threads = 0    # default
allow_update = true    # default
allow_write = false    # default
result_cache = 50    # default
output_dir = '<output_dir>'    # default
max_output_file_size = "100M"    # default
```

2. The start directory

```json request
{ "query": "name = 'main.rs'" }
```

```text error
error[dir_not_allowed]: the working directory '.' is outside of the allowed roots
  hint: get_config shows the allowed roots
```

3. Inside the root

```json request
{ "dirs": ["projects"], "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'main.rs'

projects/my-app/src/
main.rs  [#1]
```

## E121.102 CONFIG.md Command line: the arguments

`--timeout`, `--no-update` and `--log-level` of the table; the command line is the source `cli`.

Fixture: `spec-config-session`

```text command
easy-peasy-mcp --timeout 10s --no-update --log-level debug
```

1. `get_config`

```json request get_config
{ "section": "server", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']    # default
default_dirs = ['.']    # default
timeout = "10s"    # cli
threads = 0    # default
allow_update = false    # cli
allow_write = false    # default
result_cache = 50    # default
output_dir = '<output_dir>'    # default
max_output_file_size = "100M"    # default
```

2. `update_config`

```json request update_config
{ "set": "output.context = 2" }
```

```text error
error[update_disabled]: update_config is disabled
  hint: the configuration has server.allow_update = false
```

## E121.103 CONFIG.md Config file: full example

The full example loads; every value is the default, the keys written in the file have it as their source.

Fixture: `spec-config-session`

```toml config.toml
[server]
allowed_roots = []              # empty: the directory the server is started in
default_dirs = ["."]            # working directories when the request has no `dirs`
timeout = "30s"
threads = 0                     # 0: number of CPU cores
allow_update = true             # whether `update_config` is available
result_cache = 50               # results kept for `within` / `since`

[traversal]
min_depth = 0
# max_depth = 10                # not set: unlimited
follow_symlinks = false
hidden = false
ignore_files = [".gitignore", ".ignore", ".aiignore"]
git_excludes = true             # .git/info/exclude and the global git excludes file
ignore_paths = []               # extra ignore files applied to every working directory
exclude = []                    # wildcards of paths which are never walked
path_case = "native"            # native | sensitive | insensitive
binary = "skip"                 # skip | search | exclude
max_file_size = "10M"
encoding = "auto"
# fallback_encoding = "windows-1251"   # not set: invalid bytes are replaced
archives = "none"               # none | compressed | all
max_archive_size = "100M"
skip_generated = true

[output]
format = "text"                 # text | json
detail = "lines"                # exists | count | facets | files | lines
# budget = 2000                 # not set: no token budget
chars_per_token = 4
funnel = "auto"                 # auto | always | never
facets = ["dir", "lang", "kind", "condition"]
facet_depth = 2
facet_limit = 8
meta = false
context = 0
# before = 0                    # not set: equal to `context`
# after = 0                     # not set: equal to `context`
path_fragments = false
highlight = "auto"              # auto | none | columns | markers
markers = ["⟦", "⟧"]
max_files = 100
max_lines = 500
max_matches = 1000
max_matches_per_file = 50
max_line_length = 300
max_output_per_file = 4000
max_output = 40000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"         # relative | absolute

[escape]
max_level = 3

[query]
fuzzy_min_score = 0.5

[kinds]
generated_markers = ["@generated", "DO NOT EDIT", "Code generated by", "auto-generated"]
generated_marker_lines = 5
generated = ["**/*.lock", "**/package-lock.json", "**/pnpm-lock.yaml", "**/*.min.js", "**/*.min.css", "**/*.pb.go", "**/*_pb2.py", "**/*.g.dart", "**/*.generated.*"]
test = ["**/test/**", "**/tests/**", "**/__tests__/**", "**/spec/**", "**/*_test.*", "**/test_*.py", "**/*.test.*", "**/*.spec.*", "**/*Test.java", "**/*Tests.cs"]
doc = ["**/*.md", "**/*.rst", "**/*.adoc", "**/README*", "**/LICENSE*", "**/CHANGELOG*", "**/docs/**"]
config = ["**/*.toml", "**/*.yaml", "**/*.yml", "**/*.ini", "**/*.cfg", "**/*.conf", "**/*.properties", "**/*.env", "**/package.json", "**/tsconfig*.json", "**/*.config.json", "**/*.config.js", "**/*.config.ts", "**/Dockerfile", "**/Makefile", "**/CMakeLists.txt", "**/requirements*.txt", "**/.github/**", "**/.gitlab-ci.yml"]

[git]
base = "auto"

# Languages: the built-in table is in "Languages" below, here one entry as an example.
# Built-in languages can be overridden, new ones added.
[languages.rust]
extensions = ["rs"]
line_comments = ["//"]
block_comments = [["/*", "*/"]]
nested_block_comments = true
strings = ['"']
source = true
```

```text command
easy-peasy-mcp --config config.toml
```

```json request get_config
{ "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>']    # file: config.toml
default_dirs = ['.']    # file: config.toml
timeout = "30s"    # file: config.toml
threads = 0    # file: config.toml
allow_update = true    # file: config.toml
allow_write = false    # default
result_cache = 50    # file: config.toml
output_dir = '<output_dir>'    # default
max_output_file_size = "100M"    # default

[traversal]
min_depth = 0    # file: config.toml
# max_depth =  (not set)    # default
follow_symlinks = false    # file: config.toml
hidden = false    # file: config.toml
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # file: config.toml
git_excludes = true    # file: config.toml
ignore_paths = []    # file: config.toml
exclude = []    # file: config.toml
path_case = "native"    # file: config.toml
binary = "skip"    # file: config.toml
max_file_size = "10M"    # file: config.toml
encoding = "auto"    # file: config.toml
# fallback_encoding =  (not set)    # default
archives = "none"    # file: config.toml
max_archive_size = "100M"    # file: config.toml
skip_generated = true    # file: config.toml

[output]
format = "text"    # file: config.toml
detail = "lines"    # file: config.toml
count_by = "total"    # default
# budget =  (not set)    # default
chars_per_token = 4    # file: config.toml
funnel = "auto"    # file: config.toml
facets = ['dir', 'lang', 'kind', 'condition']    # file: config.toml
facet_depth = 2    # file: config.toml
facet_limit = 8    # file: config.toml
meta = false    # file: config.toml
context = 0    # file: config.toml
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # file: config.toml
highlight = "auto"    # file: config.toml
markers = ['⟦', '⟧']    # file: config.toml
max_files = 100    # file: config.toml
max_lines = 500    # file: config.toml
max_matches = 1000    # file: config.toml
max_matches_per_file = 50    # file: config.toml
max_line_length = 300    # file: config.toml
max_output_per_file = 4000    # file: config.toml
max_output = 40000    # file: config.toml
dedent = true    # file: config.toml
blank_context = false    # file: config.toml
group_dirs = true    # file: config.toml
group_repeated = 3    # file: config.toml
repeated_locations = 3    # default
sort = "relevance"    # file: config.toml
path_style = "relative"    # file: config.toml
omit = []    # default

[escape]
max_level = 3    # file: config.toml

[query]
fuzzy_min_score = 0.5    # file: config.toml

[kinds]
generated_markers = ['@generated', 'DO NOT EDIT', 'Code generated by', 'auto-generated']    # file: config.toml
generated_marker_lines = 5    # file: config.toml
generated = ['**/*.lock', '**/package-lock.json', '**/pnpm-lock.yaml', '**/*.min.js', '**/*.min.css', '**/*.pb.go', '**/*_pb2.py', '**/*.g.dart', '**/*.generated.*']    # file: config.toml
test = ['**/test/**', '**/tests/**', '**/__tests__/**', '**/spec/**', '**/*_test.*', '**/test_*.py', '**/*.test.*', '**/*.spec.*', '**/*Test.java', '**/*Tests.cs']    # file: config.toml
doc = ['**/*.md', '**/*.rst', '**/*.adoc', '**/README*', '**/LICENSE*', '**/CHANGELOG*', '**/docs/**']    # file: config.toml
config = ['**/*.toml', '**/*.yaml', '**/*.yml', '**/*.ini', '**/*.cfg', '**/*.conf', '**/*.properties', '**/*.env', '**/package.json', '**/tsconfig*.json', '**/*.config.json', '**/*.config.js', '**/*.config.ts', '**/Dockerfile', '**/Makefile', '**/CMakeLists.txt', '**/requirements*.txt', '**/.github/**', '**/.gitlab-ci.yml']    # file: config.toml

[git]
base = "auto"    # file: config.toml

[languages.c]
extensions = ['c', 'h']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.cpp]
extensions = ['cpp', 'cc', 'cxx', 'hpp', 'hh', 'hxx']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.csharp]
extensions = ['cs']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.java]
extensions = ['java']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.kotlin]
extensions = ['kt', 'kts']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = true    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.scala]
extensions = ['scala', 'sc']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = true    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.go]
extensions = ['go']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', '`']    # default
source = true    # default

[languages.swift]
extensions = ['swift']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = true    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.dart]
extensions = ['dart']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = true    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.groovy]
extensions = ['groovy', 'gradle']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.javascript]
extensions = ['js', 'jsx', 'mjs', 'cjs']    # default
names = []    # default
shebangs = ['node']    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'", '`']    # default
source = true    # default

[languages.typescript]
extensions = ['ts', 'tsx', 'mts', 'cts']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'", '`']    # default
source = true    # default

[languages.rust]
extensions = ['rs']    # file: config.toml
names = []    # file: config.toml
shebangs = []    # file: config.toml
line_comments = ['//']    # file: config.toml
block_comments = [['/*', '*/']]    # file: config.toml
nested_block_comments = true    # file: config.toml
strings = ['"']    # file: config.toml
source = true    # file: config.toml

[languages.php]
extensions = ['php']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//', '#']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.css]
extensions = ['css']    # default
names = []    # default
shebangs = []    # default
line_comments = []    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.scss]
extensions = ['scss', 'less']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.python]
extensions = ['py', 'pyi']    # default
names = []    # default
shebangs = ['python', 'python3']    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.ruby]
extensions = ['rb']    # default
names = ['Gemfile', 'Rakefile']    # default
shebangs = ['ruby']    # default
line_comments = ['#']    # default
block_comments = [['=begin', '=end']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.shell]
extensions = ['sh', 'bash', 'zsh']    # default
names = []    # default
shebangs = ['sh', 'bash', 'zsh']    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.perl]
extensions = ['pl', 'pm']    # default
names = []    # default
shebangs = ['perl']    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.r]
extensions = ['r']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.powershell]
extensions = ['ps1', 'psm1']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = [['<#', '#>']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.batch]
extensions = ['bat', 'cmd']    # default
names = []    # default
shebangs = []    # default
line_comments = ['REM', '::']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default

[languages.sql]
extensions = ['sql']    # default
names = []    # default
shebangs = []    # default
line_comments = ['--']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.lua]
extensions = ['lua']    # default
names = []    # default
shebangs = []    # default
line_comments = ['--']    # default
block_comments = [['--[[', ']]']]    # default
nested_block_comments = false    # default
strings = ['"', "'"]    # default
source = true    # default

[languages.haskell]
extensions = ['hs']    # default
names = []    # default
shebangs = []    # default
line_comments = ['--']    # default
block_comments = [['{-', '-}']]    # default
nested_block_comments = true    # default
strings = ['"']    # default
source = true    # default

[languages.clojure]
extensions = ['clj', 'cljs', 'cljc']    # default
names = []    # default
shebangs = []    # default
line_comments = [';']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default

[languages.lisp]
extensions = ['lisp', 'el', 'scm']    # default
names = []    # default
shebangs = []    # default
line_comments = [';']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default

[languages.html]
extensions = ['html', 'htm']    # default
names = []    # default
shebangs = []    # default
line_comments = []    # default
block_comments = [['<!--', '-->']]    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default

[languages.xml]
extensions = ['xml', 'svg', 'xsd', 'xsl']    # default
names = []    # default
shebangs = []    # default
line_comments = []    # default
block_comments = [['<!--', '-->']]    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.vue]
extensions = ['vue']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['<!--', '-->'], ['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"', "'", '`']    # default
source = true    # default

[languages.markdown]
extensions = ['md', 'markdown']    # default
names = []    # default
shebangs = []    # default
line_comments = []    # default
block_comments = [['<!--', '-->']]    # default
nested_block_comments = false    # default
strings = []    # default
source = false    # default

[languages.yaml]
extensions = ['yml', 'yaml']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.toml]
extensions = ['toml']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.ini]
extensions = ['ini', 'cfg', 'conf']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#', ';']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.properties]
extensions = ['properties']    # default
names = []    # default
shebangs = []    # default
line_comments = ['#', '!']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.dotenv]
extensions = ['env']    # default
names = ['.env']    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.json]
extensions = ['json']    # default
names = []    # default
shebangs = []    # default
line_comments = []    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.jsonc]
extensions = ['jsonc', 'json5']    # default
names = []    # default
shebangs = []    # default
line_comments = ['//']    # default
block_comments = [['/*', '*/']]    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = false    # default

[languages.dockerfile]
extensions = ['dockerfile']    # default
names = ['Dockerfile']    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default

[languages.makefile]
extensions = ['mk']    # default
names = ['Makefile', 'GNUmakefile']    # default
shebangs = []    # default
line_comments = ['#']    # default
block_comments = []    # default
nested_block_comments = false    # default
strings = ['"']    # default
source = true    # default
```

## E121.104 CONFIG.md Ignore files: `.aiignore`

`fixtures/large/`, `*.min.js` and `secrets/**` are not searched, `secrets/README.md` is re-included. Generated files
are not skipped here, so `web/app.min.js` is hidden only by the `.aiignore`.

Fixture: `spec-config-ignore`

1. With the `.aiignore`

```json request
{
  "query": "type = file",
  "traversal": { "skip_generated": false },
  "output": { "detail": "files", "sort": "path asc" }
}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 type = file

fixtures/small/
user.json  [#1]
secrets/
README.md  [#1]
web/
app.js  [#1]
```

2. Without ignore files

```json request
{
  "query": "type = file",
  "traversal": { "skip_generated": false, "ignore_files": [] },
  "output": { "detail": "files", "sort": "path asc" }
}
```

```text response
files: 6  lines: 0  matches: 0  result: r2
#1 type = file

fixtures/large/
users.json  [#1]
fixtures/small/
user.json  [#1]
secrets/
README.md  [#1]
prod.env  [#1]
web/
app.js  [#1]
app.min.js  [#1]
```

## E121.105 CONFIG.md kinds: a list in the config file replaces the built-in list

`web/button.test.tsx` is no longer a test, `e2e/` and `src/gen/` are new.

Fixture: `spec-config-kinds`

```toml conf/kinds.toml
[kinds]
test = ["**/tests/**", "**/*_test.go", "**/e2e/**"]
generated = ["**/*.lock", "**/src/gen/**", "**/*.pb.go"]
```

```text command
easy-peasy-mcp --config conf/kinds.toml
```

1. Tests

```json request
{ "query": "kind = test", "output": { "detail": "files", "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 kind = test

api/
handler_test.go  [#1]
e2e/
login.ts  [#1]
tests/
integration.rs  [#1]
```

2. Generated files

```json request
{ "query": "kind = generated", "output": { "detail": "files", "sort": "path asc" } }
```

```text response
files: 3  lines: 0  matches: 0  result: r2
#1 kind = generated

./
Cargo.lock  [#1]
api/
api.pb.go  [#1]
src/gen/
api.rs  [#1]
```

## E121.106 CONFIG.md Languages: override or add a language

Terraform is a new language with its comments; `.tpl` files are HTML.

Fixture: `spec-config-languages`

```toml conf/languages.toml
# Terraform
[languages.terraform]
extensions = ["tf", "tfvars"]
line_comments = ["#", "//"]
block_comments = [["/*", "*/"]]
strings = ['"']

# treat .tpl files as HTML
[languages.html]
extensions = ["html", "htm", "tpl"]
block_comments = [["<!--", "-->"]]
```

```text command
easy-peasy-mcp --config conf/languages.toml
```

1. The languages

```json request
{ "query": "lang = terraform or lang = html", "output": { "detail": "files" } }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 lang = terraform
#2 lang = html

infra/
main.tf  [#1]
prod.tfvars  [#1]
templates/
page.tpl  [#2]
web/
index.html  [#2]
```

2. The comments of Terraform: only the match in the string is outside of a comment

```json request
{ "query": "lang = terraform and text has 'TODO' /c" }
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 lang = terraform
#2 text has 'TODO' /c

infra/
main.tf  [#1 #2]  lines: 1  matches: 1
⇥2
3:bucket = "logs-⟦#2:TODO⟧" // TODO: a real name
```

## E121.107 CONFIG.md get_config: one section

`exclude` comes from the config file, `max_file_size` from `update_config`.

Fixture: `spec-config-session`

```toml .config/easy-peasy-mcp/config.toml
[traversal]
exclude = ["**/target/**", "**/node_modules/**"]
```

```text env
EASY_PEASY_CONFIG=<fixture>/.config/easy-peasy-mcp/config.toml
```

```text command
easy-peasy-mcp
```

1. `update_config`

```json request update_config
{ "set": "traversal.max_file_size = \"50M\"" }
```

```text response
[traversal]
max_file_size = "50M"    # was "10M"
```

2. The request of the spec

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0
# max_depth =  (not set)
follow_symlinks = false
hidden = false
ignore_files = ['.gitignore', '.ignore', '.aiignore']
git_excludes = true
ignore_paths = []
exclude = ['**/target/**', '**/node_modules/**']
path_case = "native"
binary = "skip"
max_file_size = "50M"
encoding = "auto"
# fallback_encoding =  (not set)
archives = "none"
max_archive_size = "100M"
skip_generated = true
```

## E121.108 CONFIG.md get_config: the sources

The config file is `/home/me/.config/easy-peasy-mcp/config.toml` of the spec.

Fixture: `spec-config-session`

```toml .config/easy-peasy-mcp/config.toml
[traversal]
exclude = ["**/target/**", "**/node_modules/**"]
```

```text env
EASY_PEASY_CONFIG=<fixture>/.config/easy-peasy-mcp/config.toml
```

```text command
easy-peasy-mcp
```

1. `update_config`

```json request update_config
{ "set": "traversal.max_file_size = \"50M\"" }
```

```text response
[traversal]
max_file_size = "50M"    # was "10M"
```

2. The request of the spec

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = ['**/target/**', '**/node_modules/**']    # file: <fixture>/.config/easy-peasy-mcp/config.toml
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "50M"    # update
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "none"    # default
max_archive_size = "100M"    # default
skip_generated = true    # default
```

## E121.109 CONFIG.md update_config: set and reset

The keys in the order of the spec, the reset one with `, reset`.

Fixture: `spec-config-session`

1. An earlier call

```json request update_config
{ "set": "output.format = \"json\"" }
```

```text response
[output]
format = "json"    # was "text"
```

2. The request of the spec

```json request update_config
{
  "set": "[output]\nmax_files = 30\ncontext = 2\n\n[traversal]\nexclude = [\"**/target/**\"]",
  "reset": ["output.format"]
}
```

```text response
[traversal]
exclude = ['**/target/**']    # was []

[output]
format = "text"    # was "json", reset
context = 2    # was 0
max_files = 30    # was 100
```

## E121.110 CONFIG.md update_config: dotted keys

Fixture: `spec-config-session`

```json request update_config
{ "set": "output.context = 2" }
```

```text response
[output]
context = 2    # was 0
```

## E121.111 CONFIG.md update_config: values which are not set, nothing changed

Fixture: `spec-config-session`

1. Set

```json request update_config
{ "set": "traversal.max_depth = 3" }
```

```text response
[traversal]
max_depth = 3    # was (not set)
```

2. Reset

```json request update_config
{ "reset": ["traversal.max_depth"] }
```

```text response
[traversal]
# max_depth =  (not set)    # was 3, reset
```

3. An empty `reset`

```json request update_config
{ "reset": [] }
```

```text response
# nothing changed
```

## E121.112 CONFIG.md update_config: the errors

The rows of the table and the errors described below it.

Fixture: `spec-config-session`

1. `invalid_toml`

```json request update_config
{ "set": "output.context = " }
```

```text error
error[invalid_toml]: invalid TOML in 'set'
  hint: line 1, col 18: string values must be quoted, expected literal string
```

2. `read_only` in `set`

```json request update_config
{ "set": "server.allowed_roots = [\"/\"]" }
```

```text error
error[read_only]: 'server.allowed_roots' is read-only
  hint: allowed roots are set only at start
```

3. `read_only` in `reset`

```json request update_config
{ "reset": ["server.allow_update"] }
```

```text error
error[read_only]: 'server.allow_update' is read-only
  hint: allow_update is set only at start
```

4. `unknown_key`

```json request update_config
{ "set": "output.max_file = 10" }
```

```text error
error[unknown_key]: unknown key 'output.max_file'
  hint: did you mean 'output.max_files'?
```

5. `unknown_key`, a section

```json request update_config
{ "set": "[colors]" }
```

```text error
error[unknown_key]: unknown key 'colors'
  hint: expected one of: server, traversal, output, escape, query, kinds, git, languages
```

6. `invalid_value`

```json request update_config
{ "set": "output.detail = \"all\"" }
```

```text error
error[invalid_value]: invalid value for 'output.detail'
  hint: expected one of: exists, count, facets, files, lines
```

7. A section written as a value

```json request update_config
{ "set": "output = 1" }
```

```text error
error[unknown_key]: 'output' must be a section
```

8. A section name in `reset`

```json request update_config
{ "reset": ["output"] }
```

```text error
error[unknown_key]: unknown key 'output'
  hint: expected a key like 'output.max_files', or 'all'
```

## E121.113 CONFIG.md Examples: one project, defaults tuned for a Rust workspace

`target` and the lock file are excluded, a match has one line of context; blank context lines are left out.

Fixture: `spec-config-home`

```toml config.toml
[server]
allowed_roots = ["~/projects/my-app"]

[traversal]
exclude = ["**/target/**", "**/*.lock"]

[output]
context = 1
max_files = 50
```

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --config config.toml
```

1. `get_config`

```json request get_config
{ "section": "server" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>/projects/my-app']
default_dirs = ['.']
timeout = "30s"
threads = 0
allow_update = true
allow_write = false
result_cache = 50
output_dir = '<output_dir>'
max_output_file_size = "100M"
```

2. `find`

```json request
{ "dirs": ["projects/my-app"], "query": "text has '0.3.0'" }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has '0.3.0'

projects/my-app/
Cargo.toml  [#1]  lines: 1  matches: 1
2-name = "my-app"
3:version = "⟦#1:0.3.0⟧"
4-edition = "2024"
projects/my-app/src/
main.rs  [#1]  lines: 1  matches: 1
3:const VERSION: &str = "⟦#1:0.3.0⟧";
```

## E121.114 CONFIG.md Examples: several roots, read-only configuration, logs with archives

The logs are searched with their rotated `.gz` and `.zip` logs; the configuration can't be changed.

Fixture: `spec-config-home`

```toml config.toml
[server]
allowed_roots = ["~/projects", "~/logs/my-app"]
default_dirs = ["~/projects"]
timeout = "60s"
allow_update = false

[traversal]
max_file_size = "200M"          # logs are big
archives = "all"
max_archive_size = "1G"
```

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --config config.toml
```

1. `get_config` of `server`

```json request get_config
{ "section": "server" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>/projects', '<fixture>/logs/my-app']
default_dirs = ['~/projects']
timeout = "60s"
threads = 0
allow_update = false
allow_write = false
result_cache = 50
output_dir = '<output_dir>'
max_output_file_size = "100M"
```

2. `get_config` of `traversal`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0
# max_depth =  (not set)
follow_symlinks = false
hidden = false
ignore_files = ['.gitignore', '.ignore', '.aiignore']
git_excludes = true
ignore_paths = []
exclude = []
path_case = "native"
binary = "skip"
max_file_size = "200M"
encoding = "auto"
# fallback_encoding =  (not set)
archives = "all"
max_archive_size = "1G"
skip_generated = true
```

3. The logs

```json request
{ "dirs": ["logs/my-app"], "query": "text has 'payment failed'" }
```

```text response
files: 3  lines: 3  matches: 3  result: r1
#1 text has 'payment failed'

logs/my-app/
app.log  [#1]  lines: 1  matches: 1
2:2026-10-08 10:05 ERROR ⟦#1:payment failed⟧: card expired
app.log.1.gz  [#1]  lines: 1  matches: 1
2:2026-10-07 23:59 ERROR ⟦#1:payment failed⟧: card declined
logs/my-app/2026-09.zip!/
server.log  [#1]  lines: 1  matches: 1
2:2026-09-30 12:01 ERROR ⟦#1:payment failed⟧: timeout

warning: binary_skipped (1): logs/my-app/2026-09.zip
```

4. `update_config`

```json request update_config
{ "set": "output.context = 3" }
```

```text error
error[update_disabled]: update_config is disabled
  hint: the configuration has server.allow_update = false
```

5. A search without `dirs`: the default directory `~/projects`

```json request
{ "query": "name = 'main.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r2
#1 name = 'main.rs'

~/projects/my-app/src/
main.rs  [#1]
```

## E121.115 CONFIG.md Examples: frontend monorepo

`node_modules`, `dist` and `coverage` are not walked; the minified line is cut to windows of `max_line_length = 200`.

Fixture: `spec-config-home`

```toml config.toml
[server]
allowed_roots = ["~/work/web"]

[traversal]
exclude = ["**/node_modules/**", "**/dist/**", "**/.next/**", "**/coverage/**"]
max_file_size = "2M"            # don't search the content of big bundles and snapshots

[output]
max_line_length = 200           # minified code has huge lines
```

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --config config.toml
```

```json request
{ "dirs": ["work/web"], "query": "text has 'useState'" }
```

```text response
files: 2  lines: 3  matches: 3  result: r1
#1 text has 'useState'

work/web/src/
chart.js  [#1]  lines: 1  matches: 1
1:import{⟦#1:useState⟧ as u,useEffect as e}from"react";export function Chart(p){const[d,s]=u([]);e(()=>{fetch(p.url).t…
app.ts  [#1]  lines: 2  matches: 2
1:import { ⟦#1:useState⟧ } from "react";
--
⇥2
4:const [count, setCount] = ⟦#1:useState⟧(0);
```

## E121.116 CONFIG.md Examples: Windows, legacy encoding

The source in windows-1251 is decoded with the fallback encoding, the UTF-8 one as it is.

Fixture: `spec-config-legacy`

Platform: `windows`

```toml conf/legacy.toml
[server]
allowed_roots = ['~\work\legacy', '~\src']

[traversal]
encoding = "auto"
fallback_encoding = "windows-1251"
```

```text env
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --config conf\legacy.toml
```

1. `get_config`

```json request get_config
{ "section": "server" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[server]
allowed_roots = ['<fixture>\work\legacy', '<fixture>\src']
default_dirs = ['.']
timeout = "30s"
threads = 0
allow_update = true
allow_write = false
result_cache = 50
output_dir = '<output_dir>'
max_output_file_size = "100M"
```

2. `find`

```json request
{ "dirs": ["work\\legacy", "src"], "query": "text has 'Отчёт'" }
```

```text response
files: 2  lines: 2  matches: 2  result: r1
#1 text has 'Отчёт'

work\legacy\db\
report.sql  [#1]  lines: 1  matches: 1  windows-1251
1:-- ⟦#1:Отчёт⟧ по продажам
src\app\
report.py  [#1]  lines: 1  matches: 1
1:# ⟦#1:Отчёт⟧ по продажам
```

## E121.117 CONFIG.md Examples: tune the session from the agent

More context for the rest of the session; one search of a library in `node_modules` without ignore files and with a
short timeout. The next search ignores `node_modules` again and keeps the context.

Fixture: `spec-config-session`

1. `update_config`

```json request update_config
{ "set": "output.context = 3" }
```

```text response
[output]
context = 3    # was 0
```

2. Once inside `node_modules`

```json request
{
  "query": "path starts 'node_modules/some-lib/' and text has 'deprecated' /i",
  "timeout": "5s",
  "traversal": { "ignore_files": [] }
}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 path starts 'node_modules/some-lib/'
#2 text has 'deprecated' /i

node_modules/some-lib/
index.js  [#1 #2]  lines: 1  matches: 1
5-/**
6- * Parses the input.
7: * @⟦#2:deprecated⟧ since 2.0, use parseStrict
8- */
9-function parse(input) {
10-  return strict.parse(input, { loose: true });
```

3. The next search

```json request
{ "query": "text has 'deprecated' /i" }
```

```text response
files: 1  lines: 1  matches: 1  result: r2
#1 text has 'deprecated' /i

src/
app.js  [#1]  lines: 1  matches: 1
1-import { parse } from 'some-lib';
2:// parse is ⟦#1:deprecated⟧, use parseStrict
3-export const run = (s) => parse(s);
```

## E121.118 CONFIG.md Ignore files: a working directory inside an ignored directory

`node_modules/` is in `.gitignore`, but a working directory inside it is searched, like a path given to `rg`.

Fixture: `spec-config-session`

```json request
{ "dirs": ["node_modules/some-lib"], "query": "text has 'deprecated' /i" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'deprecated' /i

node_modules/some-lib/
index.js  [#1]  lines: 1  matches: 1
⇥1
7:* @⟦#1:deprecated⟧ since 2.0, use parseStrict
```

## E121.119 CONFIG.md Command line: `--config` with `~`

`~` in the path of `--config` is the home directory, also without a shell, as in the registration of an MCP client.

Fixture: `spec-config-session`

```toml conf/work.toml
[server]
timeut = "10s"
```

```text env
HOME=<fixture>
USERPROFILE=<fixture>
```

```text command
easy-peasy-mcp --config ~/conf/work.toml
```

```text stderr exit 1
easy-peasy-mcp: <fixture>/conf/work.toml:2: unknown key 'server.timeut', did you mean 'server.timeout'?
```
