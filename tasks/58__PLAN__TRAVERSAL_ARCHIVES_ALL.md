# Plan for task 58: `traversal.archives`: `all`

## Design

- `src/archive.rs` (new): `ArchiveKind` by the end of a name in any case — `.zip` / `.jar`, `.tar`, `.tar.gz` /
  `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`, `.7z`; `for_each_member` streams the file members of an archive in their
  order with their stored path, uncompressed size and modification date (zip: the extended timestamp, else the DOS
  date in the local time zone; tar: the header; 7z: the FILETIME) and a reader of the content. Directories, links and
  other entries are not members.
- `src/search/mod.rs`: `Archives::All`; after the archive file itself is visited, `visit_members` reads its members
  and evaluates each as an entry: path `<archive>!/<member>` (NFC), name its last part, type `file`, size and
  modified of the entry, created and accessed unknown (`date_unsupported` when the query needs them), no git status,
  the language and kind by its name and path (no markers), content read from the archive with `max_file_size`
  (`too_big`), never decompressed again (nested archives are not opened). An archive which can't be read gets
  `archive_error`. `evaluate` reads the content through a closure, so files and members share it.
- `Found.member`, sorted after the archive path; duplicates by the identity of the archive and the member path.
- `src/warning.rs`: entries with a member, written `<archive>!/<member>`.
- `src/output/mod.rs`: a member is under the header `<archive>!/<member dir>/`; the part after `!/` uses `/`.
- `src/config/keys.rs`: `all` is implemented (`Key::unimplemented` keeps an empty list for later values).

## Dependencies

- `zip` (features `deflate-flate2-zlib-rs`, `bzip2`, `deflate64`, `lzma`, `xz`: pure Rust; zstd members are
  unsupported, they get `archive_error`), `tar`, `sevenz-rust2` (without AES: encrypted archives get `archive_error`).

## Tests

- examples E58.1–E58.10 (fixture `archives`: one archive per format, a nested archive, a broken zip)
- unit: the archive kinds, the members of every format with size, date and content, a broken archive

## Steps

1. failing tests
2. the archive reader, the members in the walker, the output

## Risks

- the binary check of every archive file with `all` adds `binary_skipped` warnings to text searches; that is what the
  spec says (the archive file is a binary entry)
