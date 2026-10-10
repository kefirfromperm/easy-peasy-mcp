# Summary of task 67: `kinds.doc`

Server version: `0.67.0`.

## What was built

- `kinds.doc` (CONFIG.md#kinds, QUERY.md#kind), default the built-in list of the full example of CONFIG.md:
  wildcards of the paths of kind `doc`, matched like `path ~` with the case sensitivity of `path_case`; from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `kinds.doc`. A list replaces the built-in one.
- Errors like `traversal.exclude` (spec added with task 65): `invalid value for 'kinds.doc': a list of wildcards
  is expected`, `... invalid wildcard '[z-a]': invalid range 'z-a'`; in a request `'kinds.doc' must be a list
  of strings`, `invalid wildcard '**/{a' in 'kinds.doc'` with the hint.

## Architecture

- `src/config/keys.rs`: `Key::KindsDoc`, `Type::Wildcards`.
- `src/engine.rs`: `KindOptions.doc` from the settings.
- `src/description.rs`: the key in the `kinds` object.

## Deviations from the plan

- The `get_config` examples of tasks 35–66 show the key (separate commit).

## Tests

- examples E67.1–E67.8 (fixture `doc-paths`)
- unit: `request::tests::kinds_doc`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = doc", "kinds": { "doc": ["**/*.md", "**/guide/**"] } }`
