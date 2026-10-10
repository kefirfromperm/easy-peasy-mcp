# Summary of task 65: `kinds.generated`

Server version: `0.65.0`.

## What was built

- `kinds.generated` (CONFIG.md#kinds, QUERY.md#kind), default the built-in list of the full example of CONFIG.md:
  wildcards of the paths of kind `generated`, matched like `path ~` with the case sensitivity of `path_case`; from
  the config file, shown by `get_config` with its source, changed and reset by `update_config`, replaced for one
  request by `kinds.generated`. A list replaces the built-in one.
- Errors like `traversal.exclude` (spec added with task 65): `invalid value for 'kinds.generated': a list of wildcards
  is expected`, `... invalid wildcard '[z-a]': invalid range 'z-a'`; in a request `'kinds.generated' must be a list
  of strings`, `invalid wildcard '**/{a' in 'kinds.generated'` with the hint.

## Architecture

- `src/config/keys.rs`: `Key::KindsGenerated`, `Type::Wildcards`.
- `src/engine.rs`: `KindOptions.generated` from the settings.
- `src/description.rs`: the key in the `kinds` object.

## Deviations from the plan

- The `get_config` examples of tasks 35–64 show the key (separate commit).

## Tests

- examples E65.1–E65.8 (fixture `gen-paths`)
- unit: `request::tests::kinds_generated`

## Limitations and known gaps

None.

## Try it

`{ "query": "kind = generated", "kinds": { "generated": ["**/*.lock", "**/src/gen/**"] } }`
