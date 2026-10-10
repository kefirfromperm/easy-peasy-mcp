# Summary of task 74: `languages.<name>.block_comments`

Server version: `0.74.0`.

## What was built

- The key `block_comments` of a language (CONFIG.md#languages, CONFIG.md#comment-syntax): `[start, end]` pairs of
  block comment markers, used by `/c`; a block comment ends at its end marker, also on a later line. From the config
  file, shown by `get_config`, set and reset by `update_config` with the language, replaced for one request by
  `languages.<name>.block_comments`.
- A wrong value: `a list of [start, end] pairs of strings is expected` (in a request `must be a list of [start, end]
  pairs of strings`); the types of all keys of a language are in CONFIG.md now.

## Architecture

- `src/config/languages.rs`: `block_comments` joins the implemented keys (the pair type of task 71).
- `src/description.rs`: `block_comments` in the `languages` object.

## Deviations from the plan

- The examples of tasks 35–73 show the key (separate commit): the `get_config` tables, the added and removed
  languages, and `groovy` of E71.5 and `html` of E70.7, which are replaced without their block comments.

## Tests

- examples E74.1–E74.7 (fixture `pascal`)
- unit: `request::tests::languages`

## Limitations and known gaps

- Nested block comments and strings come with tasks 75–76.

## Try it

`{ "query": "text has 'TODO' /c", "languages": { "pascal": { "extensions": ["pas"], "block_comments": [["(*", "*)"]] } } }`
