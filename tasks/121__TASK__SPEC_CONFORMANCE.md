# Task 121: Spec examples as tests

Status: implemented, see [121__SUMMARY__SPEC_CONFORMANCE.md](121__SUMMARY__SPEC_CONFORMANCE.md)

Depends on: task 117, task 119, task 120

## Goal

Every request / response example of the spec is verified against the finished server.

## Scope

- the examples of README.md, QUERY.md, OUTPUT.md, CONFIG.md and TOOLS.md become example tests with fixtures that reproduce them
- a spec example that can't be reproduced exactly is fixed in the spec
- the Known gaps sections of all task files are checked: none is left open

## Spec

- [README.md](../README.md)
- [QUERY.md#recipes](../QUERY.md#recipes)
- [OUTPUT.md#examples](../OUTPUT.md#examples)
- [CONFIG.md#examples](../CONFIG.md#examples)
- [TOOLS.md#session-example](../TOOLS.md#session-example)

## Examples

`121__EXAMPLES__SPEC_CONFORMANCE.md` covers:

- every spec example, by file and section

## Acceptance criteria

- every example in `121__EXAMPLES__SPEC_CONFORMANCE.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
