# Summary of task 68: `kinds.config`

Server version: `0.68.0`.

## What was built

- `kinds.config` (CONFIG.md#kinds, QUERY.md#kind), default the built-in list of the full example of CONFIG.md:
  wildcards of the paths of kind `config`, matched like `path ~` with the case sensitivity of `path_case`; from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `kinds.config`. A list replaces the built-in one.
- Errors like `traversal.exclude` (spec added with task 65): `invalid value for 'kinds.config': a list of wildcards
  is expected`, `... invalid wildcard '[z-a]': invalid range 'z-a'`; in a request `'kinds.config' must be a list
  of strings`, `invalid wildcard '**/{a' in 'kinds.config'` with the hint.

## Architecture

- `src/config/keys.rs`: `Key::KindsConfig`, `Type::Wildcards`.
- `src/engine.rs`: `KindOptions.config` from the settings.
- `src/description.rs`: the key in the `kinds` object.

## Deviations from the plan

- The `get_config` examples of tasks 35–67 show the key (separate commit).

## Tests

- examples E68.1–E68.8 (fixture `config-paths`)
- unit: `request::tests::kinds_config`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = config", "kinds": { "config": ["**/*.toml", "**/*.hcl"] } }`
