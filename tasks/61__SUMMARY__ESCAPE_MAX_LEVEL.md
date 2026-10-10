# Summary of task 61: `escape.max_level`

Server version: `0.61.0`.

## What was built

- `escape.max_level` (CONFIG.md#escape, QUERY.md#levels), default `3`: from the config file, shown by `get_config`
  (a new section `[escape]`) with its source, changed and reset by `update_config`, replaced for one request by the
  new request object `escape`; invalid values like every whole number (`'escape.max_level' must be a whole number`,
  the CONFIG.md text added in its own commit).
- It is the upper level of `/e(style, any)` and the highest level a query may ask for; `invalid_escape_level` names
  it in the hint: `max level is 1 (configuration escape.max_level)`.
- A level whose escaped forms of the value are too many for the regex engine (more than 1024 forms of a character, or
  a form longer than 4096 bytes — `'` doubles its forms with every level of `backslash`) is `invalid_escape_level` at
  the levels with the hint `the escaped forms of this value are too many on level 20, use a lower one` (spec added in
  its own commit), instead of a regex which can't be built.

## Architecture

- `src/config/keys.rs`: `Key::MaxLevel`; the request object `escape` is a section field of the request.
- `src/query/mod.rs`: `ParseOptions.max_escape_level` instead of `escape::MAX_LEVEL`; `content_pattern` returns
  `None` when the forms are too many, checked in the validation; levels stay `u8`.
- `src/query/escape.rs`: `forms` and `pattern` are fallible, bounded by `MAX_FORMS` and `MAX_FORM_LEN`.
- `src/description.rs`: the `escape` object in the input schema.

## Deviations from the plan

- The full `get_config` examples of tasks 35–43 show the new section (separate commit).

## Tests

- examples E61.1–E61.9 (fixture `levels`)
- unit: `query::tests::escape_levels_up_to_the_configured_maximum`, `query::escape::tests::too_many_forms`,
  `request::tests::escape`

## Limitations and known gaps

- E30.1 of task 30 shows matched lines with a common indentation; `dedent` (task 84) changes that output, the example
  is fixed there.

## Try it

`{ "query": "text has `\"role\": \"admin\"` /e(backslash, any)", "escape": { "max_level": 5 } }`
