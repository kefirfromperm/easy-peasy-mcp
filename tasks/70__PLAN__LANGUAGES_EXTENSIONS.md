# Plan for task 70: `languages.<name>.extensions`

## Design

Languages are not single keys: a section `[languages.<name>]` is one value which replaces a language as a whole
(CONFIG.md#languages, clarified in its own commit).

- `src/config/languages.rs` (new):
  - the keys of a language in the order of the spec, the implemented ones (`extensions` now; tasks 71–77 add theirs);
  - a language from a TOML table (config file, `update_config`) or a JSON object (request): the written keys, the
    others as for a new language (empty lists, `nested_block_comments = false`, `source = true`); errors for a name
    which is not a word, a language which is not a table, an unknown key (did you mean), a key not implemented yet, a
    value of a wrong type;
  - `merge`: the changed languages replace the ones with the same name (compared in lower case) or are added; an
    extension (any case) in two languages is an error naming the changed language, its key and the other language;
  - `render`: the `get_config` tables, `changes`: the lines of `update_config`.
- `src/config/mod.rs`: `Settings.languages` — the built-in table, then the file; `get_config` shows them as the last
  section; `update_config` sets languages from `set` and resets `languages.<name>` (to the file or the built-in table,
  or removes it) and `all`.
- `src/request.rs`: `Request.languages`; the field `languages` is implemented.
- `src/engine.rs`: the settings with the request languages (a conflict is `invalid_option`); the language table of a
  request is built from them instead of once for the engine.
- `src/description.rs`: the `languages` object in the input schema.

## Tests

- examples E70.1–E70.12 (fixture `languages`)
- unit: parsing a language (TOML, JSON, errors), merging (replace, add, conflicts in any case), rendering, the
  changes of `update_config`, the request field

## Steps

1. failing tests
2. the languages module, settings, `get_config`, `update_config`
3. the request field and the engine

## Risks

- `get_config` without a section shows 41 language tables; the full examples of tasks 35–43 show them (separate
  commit)
