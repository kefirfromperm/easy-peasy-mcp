# Plan for task 118: CI on Linux, macOS and Windows

## Design

- `.github/workflows/ci.yml`: on every push and pull request, a job per platform (`ubuntu-latest`, `macos-latest`,
  `windows-latest`) runs the definition of done of AGENTS.md — `cargo fmt` (once, on Linux: formatting doesn't
  depend on the platform), `cargo clippy`, `cargo test --all-targets`, `cargo build --release` — and keeps the
  release binary as an artifact. The toolchain is the one of `rust-toolchain.toml` (`rustup toolchain install`);
  `core.autocrlf` is off before the checkout, so the example files and fixtures keep their line breaks on Windows
  (`.gitattributes` already marks the fixtures `-text`).
- `.github/workflows/release.yml`: on a tag `v*`, release binaries for `x86_64-unknown-linux-gnu`,
  `aarch64-apple-darwin`, `x86_64-apple-darwin` and `x86_64-pc-windows-msvc` (a `.tar.gz`, a `.zip` on Windows, with
  README.md and LICENSE), attached to a GitHub release.
- platform-specific tests: the examples already have `Platform:`; the unit and integration tests use `#[cfg(unix)]`
  / `#[cfg(windows)]` where they touch permissions and symlinks. The tracked file names are valid on every platform
  (no characters reserved on Windows, no names which differ only in case).

Rejected: `cross` or a single Linux runner cross-compiling for every target — the tests have to run on each platform.

## Tests

- `tests/ci.rs`: `ci_runs_the_definition_of_done` — the workflow runs on the three platforms and runs every command of
  the definition of done of AGENTS.md; `tracked_names_are_portable` — no tracked path has a character Windows
  reserves or differs from another one only in case
- the existing examples and tests

## Steps

1. failing tests
2. the workflows

## Risks

- this repository can't run the workflows locally: the macOS and Windows jobs are first run when the branch is
  pushed. The platform conventions (case-insensitive `native` path case, `\` in Windows paths) are tasks 119 and
  120; until they are done the examples which assume the Linux conventions may fail there.
