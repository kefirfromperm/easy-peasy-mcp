# Task 118: CI on Linux, macOS and Windows

Status: implemented, see [118__SUMMARY__CROSS_PLATFORM_CI.md](118__SUMMARY__CROSS_PLATFORM_CI.md)

Depends on: task 1

## Goal

Every commit is built and tested on the three platforms the spec targets.

## Scope

- CI runs format, clippy, tests and the release build on Linux, macOS and Windows
- release binaries for the three platforms
- platform-specific tests are marked and skipped elsewhere

## Spec

- [CONFIG.md#platforms](../CONFIG.md#platforms)

## Examples

`118__EXAMPLES__CROSS_PLATFORM_CI.md` covers:

- no new examples; the existing examples pass on all three platforms, or are marked with the platforms they apply to

## Acceptance criteria

- every example in `118__EXAMPLES__CROSS_PLATFORM_CI.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
