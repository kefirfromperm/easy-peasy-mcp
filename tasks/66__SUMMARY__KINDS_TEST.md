# Summary of task 66: `kinds.test`

Server version: `0.66.0`.

## What was built

- `kinds.test` (CONFIG.md#kinds, QUERY.md#kind), default the built-in list of the full example of CONFIG.md:
  wildcards of the paths of kind `test`, matched like `path ~` with the case sensitivity of `path_case`; from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `kinds.test`. A list replaces the built-in one.
- Errors like `traversal.exclude` (spec added with task 65): `invalid value for 'kinds.test': a list of wildcards
  is expected`, `... invalid wildcard '[z-a]': invalid range 'z-a'`; in a request `'kinds.test' must be a list
  of strings`, `invalid wildcard '**/{a' in 'kinds.test'` with the hint.

## Architecture

- `src/config/keys.rs`: `Key::KindsTest`, `Type::Wildcards`.
- `src/engine.rs`: `KindOptions.test` from the settings.
- `src/description.rs`: the key in the `kinds` object.

## Deviations from the plan

- The `get_config` examples of tasks 35–65 show the key (separate commit).

## Tests

- examples E66.1–E66.8 (fixture `test-paths`)
- unit: `request::tests::kinds_test`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = test", "kinds": { "test": ["**/tests/**", "**/e2e/**"] } }`
