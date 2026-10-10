# Summary of task 118: CI on Linux, macOS and Windows

Server version: `0.118.0`.

## What was built

- CI (`.github/workflows/ci.yml`): every push and pull request runs the definition of done of AGENTS.md on
  `ubuntu-latest`, `macos-latest` and `windows-latest` — `cargo fmt` (on Linux), `cargo clippy`, `cargo test
  --all-targets`, `cargo build --release` — and keeps the release binary of each platform as an artifact
- releases (`.github/workflows/release.yml`): a tag `v*` builds the binaries for `x86_64-unknown-linux-gnu`,
  `aarch64-apple-darwin`, `x86_64-apple-darwin` and `x86_64-pc-windows-msvc`, packs them with README.md and LICENSE
  (`.tar.gz`, `.zip` on Windows) and attaches them to the GitHub release of the tag
- the checkout keeps the line breaks on Windows (`core.autocrlf` off), the toolchain is the one of
  `rust-toolchain.toml`

## Architecture

- `.github/workflows/ci.yml`, `.github/workflows/release.yml`
- `tests/ci.rs`: the workflow runs on the three platforms and runs every command of the definition of done (read from
  AGENTS.md); every tracked path can be checked out on Windows and macOS (no reserved characters or device names, no
  paths which differ only in case)

## Deviations from the plan

None.

## Tests

- `tests/ci.rs`: `ci_runs_the_definition_of_done`, `tracked_names_are_portable`, `windows_names`
- no new examples (`118__EXAMPLES__CROSS_PLATFORM_CI.md`); the platform-specific examples already carry `Platform:`,
  the tests of permissions, hard links and symlinks `#[cfg(unix)]` / `#[cfg(windows)]`

## Limitations and known gaps

- the workflows were not run: nothing was pushed, and the macOS and Windows jobs can't run on this machine (the
  toolchain here has no other targets). The workflow files were checked with a YAML parser only.
- the examples assume the Linux conventions: the case-insensitive `native` path case of macOS and Windows and the
  `\` of Windows paths are tasks 119 and 120, until then those jobs may fail on such examples.

## Try it

Push a branch: the CI workflow runs on the three platforms. Push a tag `v0.118.0`: the release gets the four
archives.
