# Plan for task 150: `"all"` in a list of parts

## Decision

The spec step chose the first option of the task: a list with `"all"` among its items is the list form of `"all"`
(OUTPUT.md#omit, OUTPUT.md#defaults-of-spec-2, CONFIG.md#output). It is friendlier than an error with a hint: the agent
meant `"all"`, and the request works at the first try. Rejected: the error naming the item with the hint `write "all"
without the brackets` — one more round trip for a value whose meaning is clear.

- `["all", "legend"]` is `"all"` too: `"all"` covers every part
- `["all", "colour"]` stays an error: an unknown part is wrong whatever else the list has
- every spec version: accepting a value which was an error doesn't break spec 1 (TOOLS.md#versioning)
- the value is normalized where it is parsed, to `Value::Text("all")`, so `get_config` and `update_config` show
  `omit = "all"` and nothing after the parse knows about the list form

## Design

`src/config/keys.rs`, `Type::Omit` (used by `output.omit` and `output.show`):

- `parse_json` (requests, batch items) and `parse_toml` (the config file, `update_config`) share one function
  `omit_list(items: Vec<String>) -> Option<Value>`: `None` when an item is neither a part nor `"all"`,
  `Value::Text("all")` when `"all"` is among them, `Value::List(items)` otherwise
- the batch merges the `omit` of an item with the shared one before the parse (`batch::omit_union`): `["result"]` and
  `["all"]` become `["result", "all"]`, which the parse turns into `"all"`; no change there

No new types, modules or dependencies.

## Tests

- unit tests in `config/keys.rs`: `["all"]`, `["legend", "all"]` are `"all"` in JSON and TOML, `["all", "colour"]` is
  the same error as before, for `omit` and `show`
- example tests E150.1–E150.8 (`tests/examples.rs` reads the examples file)
- every earlier example still passes

## Steps

1. red: the unit tests and the examples fail
2. green: `omit_list` in `parse_json` and `parse_toml`
3. summary, status, version `0.147.0` (a minor bump: `["all"]` becomes valid)

## Risks

- a list with `"all"` written by `update_config` shows `"all"` afterwards, not the list as written; this is the
  normalization the spec asks for
