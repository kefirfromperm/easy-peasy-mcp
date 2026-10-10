# Summary of task 77: `languages.<name>.source`

Server version: `0.77.0`.

## What was built

- The key `source` of a language (CONFIG.md#languages), `true` for a new language: `true` — code, its files are
  `kind = source`; `false` — a data or document format, its files are `other` without a `config` or `doc` rule. From
  the config file, shown by `get_config`, set and reset by `update_config` with the language, replaced for one
  request by `languages.<name>.source`; a wrong value is `true or false is expected` / `must be true or false`.
- Every key of a language is implemented now; the languages section of the configuration is complete.

## Architecture

- `src/config/languages.rs`: every key of `KEYS` is accepted; the problem of a key not implemented yet is gone.
- `src/description.rs`: `source` in the `languages` object.

## Deviations from the plan

- The examples of tasks 35–76 show the key (separate commit).
- Two unit tests used `source` as the example of a language key not implemented yet; they check its value now.

## Tests

- examples E77.1–E77.8 (fixture `sourcekind`)
- unit: `config::languages::tests::languages_of_toml_and_json`, `request::tests::languages`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = source", "languages": { "json": { "extensions": ["json"], "source": true } } }`
