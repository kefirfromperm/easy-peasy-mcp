# Summary of task 58: `traversal.archives`: `all`

Server version: `0.58.0`.

## What was built

- `traversal.archives = "all"` (CONFIG.md#archives): `compressed` plus the members of archives — `.zip`, `.jar`,
  `.tar`, `.tar.gz` / `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`, `.7z` (by the end of the name, any case).
- A member is an entry with the path `<archive path>!/<member path>` (the member path as stored, never resolved
  against the file system), its own `name`, type `file`, the uncompressed `size` and `modified` of the entry;
  `created` and `accessed` are unknown (`date_unsupported` when the query needs them), every `git` condition is false.
  Only the files of an archive are members.
- The content of a member is read from the archive in memory; `max_file_size` applies to it (`too_big`); nested
  archives and compressed members are not opened (searched as they are, usually binary).
- The archive file itself is still an entry — a binary one, so text conditions give it `binary_skipped`.
- An archive which can't be read (corrupted, encrypted, an unsupported method) gets `archive_error`; members read
  before the problem stay.
- Output: a member is listed under the header `<archive>!/<member dir>/`; the part after `!/` always uses `/`.

## Architecture

- `src/archive.rs` (new): `ArchiveKind`, `Member`, `for_each_member` over zip (the extended timestamp, else the DOS
  date in the local time zone), tar (optionally through `compressed::reader`) and 7z.
- `src/search/mod.rs`: `Archives::All`; `Walk::visit_members` / `visit_member` evaluate members with the same
  `evaluate` as files, which now reads the content through a closure and warns through another; `walk_builder` holds
  the walk options. `Found.member`; files are sorted by the archive path and then the member, duplicates are found by
  the identity of the archive and the member path.
- `src/warning.rs`: warning entries with a member (`Warnings::add_member`).
- `src/output/mod.rs`: `split_found` for the headers of members.
- `src/search/kinds.rs`: `KindRules::is_generated_path` for members (no markers).

## Dependencies

`zip` (pure Rust features only: deflate via `zlib-rs`, bzip2, deflate64, lzma, xz — zstd members are unsupported),
`tar`, `sevenz-rust2` (without AES — encrypted 7z archives get `archive_error`).

## Deviations from the plan

- The code was written before the examples of this task; it was set aside (stash) and committed after the examples,
  the plan and the failing tests, in the order of AGENTS.md.
- E58.6 was fixed in its own commit: nine binary files, not ten.

## Tests

- examples E58.1–E58.10 (fixture `archives`)
- unit: `archive::tests` (kinds, the members of every format with size, date and content, a broken archive),
  `config::tests::valid_files` / `update_config_changes_nothing_yet` and `request::tests::traversal` (`all` is valid)

## Limitations and known gaps

- `max_archive_size` comes with task 59.
- The members of archives are not reported as warnings of `permission_denied`; an archive which can't be opened is.
- On Windows the part of a path before `!/` uses `\` (task 120 checks the Windows paths).

## Try it

`{ "query": "path ~ '**/*!/**' and text has 'req-7f3a9c'", "traversal": { "archives": "all" } }`
