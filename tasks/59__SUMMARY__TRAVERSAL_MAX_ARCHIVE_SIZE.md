# Summary of task 59: `traversal.max_archive_size`

Server version: `0.59.0`.

## What was built

- `traversal.max_archive_size` (CONFIG.md#traversal, CONFIG.md#archives), default `"100M"`: a size like
  `max_file_size`; from the config file, shown by `get_config` with its source, changed and reset by
  `update_config`, replaced for one request by `traversal.max_archive_size`; invalid values like every size.
- With `archives = all` the members of one archive count with their uncompressed sizes in the order of the archive;
  the member which would exceed the limit and the ones after it are not entries, and the archive gets the new
  `archive_too_big` warning (spec clarified in its own commit). A protection against zip bombs on top of
  `max_file_size` per member.

## Architecture

- `src/config/keys.rs`: `Key::MaxArchiveSize`.
- `src/warning.rs`: `WarningCode::ArchiveTooBig`, after `archive_error`.
- `src/search/mod.rs`: `SearchOptions.max_archive_size`; `Walk::visit_members` adds up the sizes.

## Deviations from the plan

- The `get_config` examples of tasks 35–58 show the key (separate commit).

## Tests

- examples E59.1–E59.7 (fixture `archives` of task 58)
- unit: `warning::tests::archive_warnings_in_the_order_of_the_table`, `request::tests::traversal`

## Limitations and known gaps

- A member whose header claims a smaller size than its content is cut by `max_file_size` when it is read.

## Try it

`{ "query": "path ~ '**/*!/**' and text has 'ERROR'", "traversal": { "archives": "all", "max_archive_size": "1G" } }`
