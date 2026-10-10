# Summary of task 105: `output.facets`

Server version: `0.105.0`.

## What was built

- `detail = facets` (OUTPUT.md#facets): the totals, the legend, the funnel when it applies, and per facet the
  number of matched files per value — counted over all matched files, no files listed.
- `output.facets` (CONFIG.md#output), default `['dir', 'lang', 'kind', 'condition']`, a list of `dir`, `ext`,
  `lang`, `kind`, `git`, `condition`: from the config file, shown by `get_config` with its source, changed and reset by
  `update_config`, replaced for one request by `output.facets`; an invalid value is `a list of facets (dir, ext,
  lang, kind, git, condition) is expected` / `must be a list of facets (...)`.
- Values: `dir` two levels below the working directory (`facet_depth` comes with task 106), `ext` without the dot or
  `(none)`, `lang`, `kind`, `git` (`modified`, `staged`, `untracked`, `conflicted`, `clean`; read without a `git`
  condition and without warnings, files outside of a repository not counted), `condition` (made the file match or
  produced a fragment in it). More files first, then in the order of the values (of the legend for conditions); more
  than 8 values (`facet_limit` comes with task 107) end with `… N more (M)`.
- Text: the name and `:` in a column of 12, `value count` joined with ` · `; JSON: `facets` after `hints`, an object per
  facet, the rest as `"… N more": M`.

## Architecture

- `src/output/facets.rs`: `Facet` (`line`, `json`), `facets`, `ext_value`, `git_values`, `conditions`; `LIMIT`.
- `src/output/mod.rs`: `Detail::Facets`, `Response.facets`; `src/output/json.rs`: `Facets`, the legend from `facets`
  detail on.
- `src/search/mod.rs`: `SearchOptions.git_status`, `Found.git`.
- `src/config/keys.rs`: `Type::Facets`, `FACETS`, `Key::Facets`; no value of the spec is unimplemented any more.
- `src/engine.rs`: `facet_names`, `parse_options` split from `try_find`; `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–104 show the key and OUTPUT.md got the details of the values and their layout
  (separate commits).

## Tests

- examples E105.1–E105.9 (fixtures `huge` of task 104, `gitrepo` of task 17)
- unit: `output::facets::tests::*`, `config::keys::tests::facets_are_known_names`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'unwrap('", "output": { "detail": "facets" } }`
