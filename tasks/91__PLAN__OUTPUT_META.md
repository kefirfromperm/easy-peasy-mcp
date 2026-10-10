# Plan for task 91: `output.meta`

## Design

- `src/config/keys.rs`: `Key::Meta` after `Detail`, `Type::Bool`, default `false`.
- `src/search/mod.rs`: `SearchOptions.meta`; `Found.meta: Option<Meta>` — type (a symlink is `link`), size, lang
  and kind of a file, the modification date; computed only for a matched entry, reusing the metadata, language and
  kind the query already needed; a member of an archive from the archive.
- `src/output/meta.rs` (new): `human_size` (`512B`, `2.1K`, …) and the dates in the time zone of the server (text
  `2026-10-08 14:03`, JSON ISO 8601 with the offset).
- `src/output/mod.rs`, `src/output/json.rs`: `Response.time_zone`; the text values after the counts and the encoding,
  the JSON fields `type`, `size`, `lang`, `kind`, `modified` in the order of the fields table (OUTPUT.md#meta, in its
  own commit).
- `src/engine.rs`, `src/description.rs`.

## Tests

- examples E91.1–E91.11 (fixture `meta`)
- unit: the sizes, the dates, the request field

## Steps

1. failing tests
2. key, metadata of the matched entries, the output

## Risks

- the score comes with task 93
