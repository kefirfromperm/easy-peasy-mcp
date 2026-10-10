# Summary of task 75: `languages.<name>.nested_block_comments`

Server version: `0.75.0`.

## What was built

- The key `nested_block_comments` of a language (CONFIG.md#languages, CONFIG.md#comment-syntax), `false` for a new
  language: with it a block comment ends after as many end markers as start markers, for `/c`; from the config file,
  shown by `get_config`, set and reset by `update_config` with the language, replaced for one request by
  `languages.<name>.nested_block_comments`; a wrong value is `true or false is expected` / `must be true or false`.

## Architecture

- `src/config/languages.rs`: `nested_block_comments` joins the implemented keys (the boolean type of task 71).
- `src/description.rs`: the key in the `languages` object.

## Deviations from the plan

- The examples of tasks 35–74 show the key (separate commit): the `get_config` tables and the added and removed
  languages.

## Tests

- examples E75.1–E75.7 (fixture `nested`)
- unit: `request::tests::languages`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO' /c", "languages": { "pascal": { "extensions": ["pas"], "block_comments": [["(*", "*)"]], "nested_block_comments": true } } }`
