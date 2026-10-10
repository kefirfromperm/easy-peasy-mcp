# Plan for task 61: `escape.max_level`

## Design

- `src/config/keys.rs`: `Key::MaxLevel` in the section `escape`, `Type::Whole`, default `3`; the request object
  `escape` is parsed by the section table (it becomes implemented with its first key).
- `src/query/mod.rs`: `ParseOptions.max_escape_level` replaces the constant `escape::MAX_LEVEL`: the levels of `any`
  and the limit of a level written in the query, named in the hint of `invalid_escape_level`. Levels are kept as
  `u8`; a configured value above 255 allows every level a query can write.
- A condition with `/e` whose regex can't be built (too many variants on a level) is `invalid_escape_level` at the
  levels, with the highest level in the hint (spec added in its own commit); the compile step builds the regex once
  in the check and doesn't fail anymore.
- `src/engine.rs`: the setting → the parse options.
- `src/description.rs`: the `escape` object in the input schema.

## Tests

- examples E61.1–E61.9 (fixture `levels`: one line per level 0–3, without indentation)
- unit: the levels with a configured maximum, a level with too many variants, the request field

## Steps

1. failing tests
2. key, parse options, the regex check

## Risks

- none
