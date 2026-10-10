# Summary of task 62: `query.fuzzy_min_score`

Server version: `0.62.0`.

## What was built

- `query.fuzzy_min_score` (CONFIG.md#query, QUERY.md#--fuzzy), default `0.5`: a number from 0 to 1 (a float or a
  whole number); from the config file, shown by `get_config` (a new section `[query]`, the number as it is: `0.65`,
  `1`) with its source, changed and reset by `update_config`, replaced for one request by the new request object
  `query_options` (`query` is the query itself).
- `~~` matches with a lower score are dropped.
- Errors (spec added in its own commit): `invalid value for 'query.fuzzy_min_score': a number from 0 to 1 is
  expected` (file, `update_config` hint), in a request `'query_options.fuzzy_min_score' must be a number from 0 to 1`.

## Architecture

- `src/config/keys.rs`: `Key::FuzzyMinScore`, `Type::Score`, `Value::Number`; `Settings::number`.
- `src/query/mod.rs`: `ParseOptions.fuzzy_min_score` instead of `fuzzy::MIN_SCORE`.
- `src/description.rs`: the `query_options` object in the input schema.

## Deviations from the plan

- The full `get_config` examples of tasks 35–43 show the new section (separate commit).
- E62.7 says that even a whole name scores below 1 (0.84), so a minimum of 1 drops every `~~` match.

## Tests

- examples E62.1–E62.8 (fixture `fuzzy` of task 32)
- unit: `request::tests::query_options`

## Limitations and known gaps

None.

## Try it

`{ "query": "name ~~ 'cfgldr'", "query_options": { "fuzzy_min_score": 0.3 } }`
