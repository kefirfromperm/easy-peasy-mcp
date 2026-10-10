# Plan for task 111: `within`

## Design

- `src/request.rs`: `within` — a result id `rN` (`'within' must be a result id like 'r12'`), `Request.within`.
- `src/cache.rs`: a kept result keeps its working directories (the canonical directory and the prefix) and every
  file its working directory, its path inside it and its archive member, besides the fingerprint.
- `src/search/mod.rs`: `Entry` — what `visit` needs of an entry (path, depth, type, name, metadata, symlink),
  implemented by the walk's entries and by `FileEntry`, a file of a kept result; `search_files` visits the files of
  one working directory without a walk (deleted files are skipped; members of archives are read from their
  archive); `Found.root` — the index of the working directory.
- `src/engine.rs`: with `within` the files of the result are searched instead of a walk: in their working
  directories (prefixes as in the result), skipped with `permission_denied` outside of the allowed roots; an unknown
  id is `result_expired` (OUTPUT.md#refine-within, clarified in its own commit); the rest of the response as usual.
- `src/error.rs`: `ErrorCode::ResultExpired`; `src/description.rs`.

## Tests

- examples E111.1–E111.5 (fixture `funnel` of task 102)
- unit: the request field, the files of a kept result (a deleted one, a member)

## Steps

1. failing tests
2. the entries, the search of files, the cache, the engine

## Risks

- `visit` takes a trait object instead of the walk's entry
