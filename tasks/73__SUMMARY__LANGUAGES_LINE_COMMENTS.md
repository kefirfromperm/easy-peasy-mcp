# Summary of task 73: `languages.<name>.line_comments`

Server version: `0.73.0`.

## What was built

- The key `line_comments` of a language (CONFIG.md#languages, CONFIG.md#comment-syntax): markers which start a
  comment until the end of the line, used by `/c`; from the config file, shown by `get_config`, set and reset by
  `update_config` with the language, replaced for one request by `languages.<name>.line_comments`. A file of a
  language without comment keys is searched as it is.

## Architecture

- `src/config/languages.rs`: `line_comments` joins the implemented keys (a list of strings).
- `src/description.rs`: `line_comments` in the `languages` object.

## Deviations from the plan

- The examples of tasks 35–72 show the key (separate commit): the `get_config` tables, the added and removed language
  of E70.6 and E72.3, and E71.5, which replaces `groovy` without its line comments.

## Tests

- examples E73.1–E73.8 (fixture `terraform`)
- unit: `request::tests::languages`

## Limitations and known gaps

- Block comments, their nesting and strings come with tasks 74–76.

## Try it

`{ "query": "text has 'TODO' /c", "languages": { "terraform": { "extensions": ["tf"], "line_comments": ["#", "//"] } } }`
