# Summary of task 153: A hint for `=` with a wildcard pattern

Server version: `0.152.0`.

## What was built

- a new variation of the hints of an empty result (OUTPUT.md#hints): `~` instead of `=` for a `name` or `path`
  condition whose value has a wildcard character (`*`, `?`, `[`, `{`). Like every variation it is searched with the
  steps of the funnel before it and reported only when it matches, with its number of files and up to three names
  (`name`) or paths (`path`):

  ```
  funnel:
    name = '*.rs'                          0
  hints:
    name ~ '*.rs'                          2 files: lib.rs, main.rs
  ```

- the modifiers of the condition are kept (`name = '*.MD' /i` → `name ~ '*.MD' /i`); a variation which doesn't
  parse is not tried; `text =` gets no such variation
- the similar names (`name =`) and similar directories (`path =`, `path starts`) are left out for a name or a
  directory with a wildcard character: a pattern is not a misspelt name, and `no such directory` for `**` would
  mislead
- QUERY.md#wildcards says that `=` takes no wildcards and points to the hint
- spec version 1 gets the hint too: a new hint is not a breaking change (TOOLS.md#versioning); it is a line of
  `hints`, left out with `omit = ["hints"]`, in JSON an entry of `"hints"`

## Architecture

- `src/query/mod.rs`: `variations` adds the rewritten condition (`rewritten`, the operator token replaced by `~`)
  after `has` instead of `=`; `has_wildcard` tests the value; `lookup` splits into `lookup_value` (unchanged rules)
  and the wildcard exception
- `src/hints.rs` is unchanged: it already searches, reports and aligns every variation of a step

## Deviations from the plan

- `name = '*.rs'` also gets the `/n` variation (`naming::words` splits `*.rs` into two words); it matches nothing and
  is harmless, the unit test lists it
- the example response block planned for QUERY.md became an inline example: `tests/spec_queries.rs` reads every block
  of QUERY.md as a query (its own commit)

## Tests

- examples E153.1–E153.8 in `tasks/153__EXAMPLES__EQUALS_WITH_WILDCARDS.md` (fixtures `equals-wildcards` in
  `tests/fixtures/`, `equals-wildcards-literal` from the setup, Linux only, since `*` can't be in a file name on
  Windows): `name =`, `path =`, no hint when `~` matches nothing, a file really named `*.rs`, a larger query with
  `/i`, `omit = ["hints"]`, JSON, `"version": 1`
- unit tests `tilde_instead_of_equals_for_a_wildcard_pattern` and `no_lookup_for_a_wildcard_pattern` in
  `src/query/mod.rs`
- all examples of the earlier tasks still pass

## Limitations

- only the first step with 0 files (or every branch of `or`) is varied, like the other variations; a `name =` with
  wildcards inside a bracket group with other conditions gets no hint
- the hint shows the files of `~`; it doesn't say in words that `=` compares the whole value (QUERY.md says it)

## How to try it

```sh
cargo build --release
./target/release/easy-peasy-mcp --root .
```

```json
{ "query": "name = '*.rs'", "output": { "detail": "files" } }
```

The empty result comes with `hints:` and `name ~ '*.rs'` with the number of Rust files.
