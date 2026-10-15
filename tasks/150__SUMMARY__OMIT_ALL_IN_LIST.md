# Summary of task 150: The error of "all" in a list of parts says what to write

Server version: `0.147.0`.

## What was built

- `output.omit = ["all"]` and `output.show = ["all"]` no longer fail: a list with `"all"` among its items is the list
  form of `"all"` ([OUTPUT.md#omit](../OUTPUT.md#omit),
  [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2), [CONFIG.md#output](../CONFIG.md#output)).
  `["all", "legend"]` is `"all"` too; `["all", "colour"]` is still `invalid_option` (`invalid_value` in
  `update_config`) with the same message as before.
- The same in a request, a query of a [batch](../OUTPUT.md#batch) (the item's `["all"]` added to the shared list is
  `"all"`), the config file and `update_config`, which answers `omit = "all"    # was [...]`; `get_config` shows
  `omit = "all"`.
- Every spec version: accepting a value which was an error doesn't break spec 1.

## Architecture

`src/config/keys.rs`: `Type::Omit` (the type of both `output.omit` and `output.show`) parses its list in
`parse_json` (requests) and `parse_toml` (config file, `update_config`) with one function, `omit_list`, which returns
`Value::Text("all")` when `"all"` is among the items, `Value::List` of the parts otherwise and `None` for an unknown
item. It replaces `are_parts`. The rest of the server only ever sees the normalized value.

## Deviations from the plan

None.

## Tests

- `config::keys::tests::a_list_with_all_is_all`: JSON and TOML lists with `"all"`, and `["all", "colour"]` as an error
- examples E150.1–E150.8 in [150__EXAMPLES__OMIT_ALL_IN_LIST.md](150__EXAMPLES__OMIT_ALL_IN_LIST.md): `omit` and
  `show` lists, `"all"` among other parts, a batch, `update_config`, the config file (a startup example with
  `get_config`), `"version": 1`, wrong values
- all earlier examples pass unchanged

## Limitations

- `update_config` and `get_config` show `"all"`, not the list as it was written.

## How to try it

```json
{ "query": "text has 'unwrap('", "output": { "omit": ["all"] } }
```

answers with the bare paths and lines, like `"omit": "all"`.
