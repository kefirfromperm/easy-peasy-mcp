# Task 164: A hint for `false` given to a list option

Status: not implemented

Depends on: task 48

## Goal

`traversal.ignore_files` is a list of file names (`.gitignore`, `.ignore`, ...), while its neighbours `hidden`,
`git_excludes` and `skip_generated` are booleans, so an agent turns it off with `false`. The error says what type is
expected but not how to turn the option off. Found in run 8 of the experiment (EXPERIMENT.md, task 158, the easy-peasy
arm), reproduced on 0.154.0:

```
find  query = "path ~ 'Cargo.toml'"
      traversal = {"ignore_files": false}

error[invalid_option]: 'traversal.ignore_files' must be a list of strings
```

The agent found `"ignore_files": []` by trial, a second failed call. A hint (`hint: [] turns it off`) saves it,
~100 tokens per occurrence.

## Scope

A spec change first (its own commit; CONFIG.md, the errors of the list options), then the code.

- a boolean given to a list-of-strings option: `invalid_option` with the same message and a hint: `false` → `[]`
  turns it off (an empty list), `true` → the default list from `get_config`
- the spec step decides which list options get it: `traversal.ignore_files`, `ignore_paths`, `exclude`,
  `kinds.generated_markers`, the lists of `output` (`omit`, `show`, `groups`), and the request and the configuration
  file alike
- the message and the code of the error are unchanged

## Out of scope

- accepting `false` as `[]`

## Examples

`164__EXAMPLES__LIST_OPTION_BOOL_HINT.md` covers:

- `traversal = {"ignore_files": false}`: the hint with `[]`
- `true` for a list option
- another list option chosen by the spec step
- a string or a number for a list option: unchanged

## Acceptance criteria

- the spec states the hint before the code is written
- every example in `164__EXAMPLES__LIST_OPTION_BOOL_HINT.md` passes as a test; the examples of all earlier tasks
  still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
