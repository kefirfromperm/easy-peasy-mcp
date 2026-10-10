# Summary of task 72: `languages.<name>.shebangs`

Server version: `0.72.0`.

## What was built

- The key `shebangs` of a language (CONFIG.md#languages): interpreters in the shebang of files without an extension
  — the program, or the first argument of `env` which is not an option, as it is or without a trailing version
  (`deno2.1` → `deno`); from the config file, shown by `get_config`, set and reset by `update_config` with the
  language, replaced for one request by `languages.<name>.shebangs`.

## Architecture

- `src/config/languages.rs`: `shebangs` joins the implemented keys (a list of strings).
- `src/description.rs`: `shebangs` in the `languages` object.

## Deviations from the plan

- The language tables of the `get_config` examples of tasks 35–71 show `shebangs`, and E70.6 the key of the added
  and removed language (separate commits).

## Tests

- examples E72.1–E72.7 (fixture `shebangs`)
- unit: `request::tests::languages`

## Limitations and known gaps

None.

## Try it

`{ "query": "lang = deno", "languages": { "deno": { "shebangs": ["deno"] } } }`
