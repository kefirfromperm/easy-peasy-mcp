# Summary of task 46: `traversal.follow_symlinks`

Server version: `0.46.0`.

## What was built

- `traversal.follow_symlinks` (CONFIG.md#traversal), default `false`: from the config file, shown by `get_config`
  with its source, changed and reset by `update_config`, replaced for one request by `traversal.follow_symlinks`.
  Invalid (spec clarified first): `invalid value for 'traversal.follow_symlinks': true or false is expected`,
  `'traversal.follow_symlinks' must be true or false` in a request.
- Following: the walker descends into symlinked directories and reads symlinked files; loops are detected and
  skipped; a symlink whose target is outside of the allowed roots is pruned with a `permission_denied` warning.
- A followed symlink has the type of its target and is a link too: `type = file and type = link` matches a followed
  symlink to a file; without a `type` condition it is returned like a regular file.
- Duplicates through symlinks are returned once, under the first path in path order (the identity of a followed
  entry is the one of its target).

## Architecture

- `src/query/eval.rs`: `FileValues.is_link`; `Filter::Type` matches `link` for every symlink.
- `src/search/mod.rs`: `SearchOptions.follow_symlinks` / `allowed_roots`; `WalkBuilder::follow_links`; a
  `filter_entry` prunes symlinks out of the roots and collects their warnings.
- `src/config.rs`: `Settings.follow_symlinks` with its source, `Mutable::FollowSymlinks`, `BOOLEAN`.
- `src/request.rs`: `Traversal.follow_symlinks`, `boolean`.

## Deviations from the plan

- The `get_config` examples of tasks 35–45 show the key (separate commit).

## Tests

- examples E46.1–E46.8 (fixture `links`: a file symlink, a directory symlink, a loop, a symlink out of the root)
- unit: `query::tests::followed_links_are_links`, `request::tests::traversal`, the file value

## Limitations and known gaps

- Junctions on Windows (task 120).

## Try it

`{ "query": "name ~ '*.rs'", "traversal": { "follow_symlinks": true } }`
