# Task 162: A hint for `in` with a list of names

Status: not implemented

Depends on: task 21, task 153

## Goal

`kind in (source, test)` takes a list, so an agent writes `name in ('Cargo.toml', 'Cargo.lock')` too. It fails with
the generic hint of `operator_not_allowed`, which lists the operators but not the way to write the list. Found in run
6 of the experiment (EXPERIMENT.md, task 154, the easy-peasy arm), reproduced on 0.152.4:

```
find  query = "name in ('Cargo.toml','Cargo.lock')"

error[operator_not_allowed]: operator 'in' is not allowed for 'name'
  name in ('Cargo.toml','Cargo.lock')
       ^^
  hint: expected one of: =, has, starts, ends, ~, ~~, =~
```

The agent then found `path ~ 'Cargo.{toml,lock}'` on its own, a second call. A hint with the rewritten condition,
`name ~ '{Cargo.toml,Cargo.lock}'`, saves that call (~100 tokens of request and error) and the guessing.

## Scope

A spec change first (its own commit; QUERY.md#errors, the `operator_not_allowed` row, and QUERY.md#wildcards), then
the code.

- `name in (...)` and `path in (...)` with a list of strings: the hint gives the condition with `~` and a `{a,b}`
  alternation, the values escaped for wildcards (`*`, `?`, `[`, `{`, `,` in a value)
- the spec step decides: the same for `ext in (...)`, `lang in (...)` if not allowed already, and a value list of
  `text` (`text in ('a', 'b')` → `(text has 'a' or text has 'b')`)
- the hint is a suggestion of the error only; the query is not run with it (unlike the variations of task 153 for
  an empty result)
- the message and the code of the error are unchanged

## Out of scope

- allowing `in` with a list for `name` and `path` (a new operator)

## Examples

`162__EXAMPLES__NAME_IN_LIST_HINT.md` covers:

- `name in ('Cargo.toml','Cargo.lock')`: the hint with `name ~ '{Cargo.toml,Cargo.lock}'`
- `path in (...)` with a value that has a wildcard character or a comma
- one value in the list; an empty list
- `"version": 1` (errors are the same in both spec versions)

## Acceptance criteria

- the spec states the hint before the code is written
- every example in `162__EXAMPLES__NAME_IN_LIST_HINT.md` passes as a test; the examples of all earlier tasks still
  pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
