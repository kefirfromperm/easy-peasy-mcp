# Plan for task 71: `languages.<name>.names`

## Design

- `src/config/languages.rs`: the checks of a key by its type — a list of strings (`extensions`, `names`, and later
  `shebangs`, `line_comments`, `strings`), a list of pairs (`block_comments`), a boolean (`nested_block_comments`,
  `source`) — from a TOML item or a JSON value; `names` is implemented: parsed, written by `get_config` and
  `update_config`, and checked for conflicts like the extensions (the conflict check has covered names since task 70).
- `src/description.rs`: `names` in the `languages` object.

## Tests

- examples E71.1–E71.9 (fixture `filenames`: `Gemfile`, `Jenkinsfile`, `build/Dockerfile`)
- unit: a language with names, a name conflict, the request field

## Steps

1. failing tests
2. the key types of a language, `names`

## Risks

- none
