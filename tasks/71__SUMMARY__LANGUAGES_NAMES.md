# Summary of task 71: `languages.<name>.names`

Server version: `0.71.0`.

## What was built

- The key `names` of a language (CONFIG.md#languages): full file names, compared in any case, the first rule of the
  language detection; from the config file, shown by `get_config`, set and reset by `update_config` with the
  language, replaced for one request by `languages.<name>.names`.
- A name listed in two languages is an error: `the name 'Dockerfile' is also in 'languages.dockerfile'` at start (at
  the line of `names`) and in `update_config`, `the name 'Gemfile' of 'languages.groovy.names' is also in
  'languages.ruby'` in a request.

## Architecture

- `src/config/languages.rs`: the keys are checked by their type (`KeyType`: a list of strings, a list of pairs, a
  boolean) from a TOML item or a JSON value (`Raw`), so the next keys only join the implemented ones; `value` writes
  every key.
- `src/description.rs`: `names` in the `languages` object.

## Deviations from the plan

- The language tables of the `get_config` examples of tasks 35–70 show `names` (separate commit).
- E70.6 was fixed in its own commit: a language added or removed by `update_config` shows all its implemented keys.

## Tests

- examples E71.1–E71.9 (fixture `filenames`)
- unit: `config::languages::tests`, `request::tests::languages`

## Limitations and known gaps

- E71.5 replaces `groovy`, whose comment keys come with tasks 73–76; their lines join its response then.

## Try it

`{ "query": "lang = groovy", "languages": { "groovy": { "extensions": ["groovy", "gradle"], "names": ["Jenkinsfile"] } } }`
