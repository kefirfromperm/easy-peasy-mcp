# Plan for task 46: `traversal.follow_symlinks`

## Design

- `src/config.rs`: `traversal.follow_symlinks` (a boolean) in the file values, `Settings` with its source and as a
  `Mutable` key; `get_config` shows it.
- `src/request.rs`: `traversal.follow_symlinks` (`'traversal.follow_symlinks' must be true or false`).
- `src/search/mod.rs`:
  - `SearchOptions.follow_symlinks` → `WalkBuilder::follow_links`; the walker detects loops (its `Loop` errors are
    skipped).
  - a followed symlink has the type of its target and is a link too: `FileValues.is_link`, and `type = link` matches
    it (`Filter::Type`).
  - `SearchOptions.allowed_roots`: with `follow_symlinks` a symlink whose canonical target is outside of the roots
    is pruned (`filter_entry`) and gets a `permission_denied` warning.
  - the identity of a followed entry is the one of its target, so duplicates through symlinks are returned once.
- `src/engine.rs`: the request value, else the settings; the roots of the settings.

## Tests

- examples E46.1–E46.8 (fixture `links`, symlinks made by the setup)
- unit: `type` with links; the request field; the file value

## Steps

1. failing tests
2. key, request field, walker

## Risks

- symlinks on Windows need the developer mode; the examples assume a platform where the setup can create them
