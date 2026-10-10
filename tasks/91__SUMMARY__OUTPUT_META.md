# Summary of task 91: `output.meta`

Server version: `0.91.0`.

## What was built

- `output.meta` (CONFIG.md#output, OUTPUT.md#meta), default `false`: from the config file, shown by `get_config` with
  its source, changed and reset by `update_config`, replaced for one request by `output.meta`; invalid values like
  every boolean key.
- With it every listed file gets its type (`file`, `dir`, `link` for a symlink), the size, language and kind of a
  file, and the modification date in the time zone of the server. Text: `file 2.1K 2026-10-08 14:03 rust source`
  after the counts and the encoding (sizes `512B`, `2.1K`, `1.5M` in units of 1024); JSON: `type`, `size`, `lang`,
  `kind`, `modified` (ISO 8601 with the offset, whole seconds). A directory and an unfollowed symlink have only the
  type and the date; a member of an archive is a file with the size and date stored in the archive.

## Architecture

- `src/search/mod.rs`: `SearchOptions.meta`, `Found.meta: Option<Meta>`, `Walk::meta` / `Walk::member_meta` —
  computed only for matched entries, reusing the metadata, language and kind the query already needed.
- `src/output/meta.rs` (new): `human_size`, `date_text`, `date_json`, the text values.
- `src/output/mod.rs`, `src/output/json.rs`: `Response.time_zone`, the values in both formats.
- `src/config/keys.rs`: `Key::Meta`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–90 show the key and OUTPUT.md got the section Meta (separate commits).

## Tests

- examples E91.1–E91.11 (fixture `meta`; the startup example sets `TZ=UTC`)
- unit: `output::meta::tests::*` (sizes, dates), `request::tests::output`

## Limitations and known gaps

- The relevance score comes with task 93.

## Try it

`{ "query": "name ~ '*.rs' and size > 20K", "output": { "detail": "files", "meta": true } }`
