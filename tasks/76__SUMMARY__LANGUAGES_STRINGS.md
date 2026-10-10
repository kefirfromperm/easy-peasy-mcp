# Summary of task 76: `languages.<name>.strings`

Server version: `0.76.0`.

## What was built

- The key `strings` of a language (CONFIG.md#languages, CONFIG.md#comment-syntax): string delimiters; a comment marker
  inside a string is text for `/c`. From the config file, shown by `get_config`, set and reset by `update_config` with
  the language, replaced for one request by `languages.<name>.strings`.

## Architecture

- `src/config/languages.rs`: `strings` joins the implemented keys (a list of strings).
- `src/description.rs`: `strings` in the `languages` object.

## Deviations from the plan

- The examples of tasks 35–75 show the key (separate commit): the `get_config` tables, the added and removed
  languages, and `groovy` of E71.5 and `html` of E70.7, which are replaced without their strings.

## Tests

- examples E76.1–E76.7 (fixture `strings`)
- unit: `request::tests::languages`

## Limitations and known gaps

- `source` comes with task 77.

## Try it

`{ "query": "text has 'TODO' /c", "languages": { "terraform": { "extensions": ["tf"], "line_comments": ["#"], "strings": ["\""] } } }`
