# Plan for task 117: final tool descriptions and instructions

## Design

- `src/description.rs`: the `find` description and input schema become the texts of TOOLS.md#find, byte for byte:
  the description a raw string, the schema the JSON text of the spec parsed once (`find_schema`), so the long
  `json!` literal and the `recursion_limit` of `lib.rs` go away. The instructions, the `get_config` and
  `update_config` descriptions and schemas and the annotations of `src/mcp.rs` already are the spec texts.
- `tests/common/mod.rs`: the response to `initialize` is the first message of an exchange (AGENTS.md, in its own
  commit), so an example can check it.

Rejected: generating the texts from TOOLS.md at build time — the spec is documentation, the binary shouldn't depend
on its layout; a test compares them instead.

## Tests

- examples E117.1–E117.3 (fixture `mvp`)
- `tests/mcp_stdio.rs`: `tools_are_the_spec_text` — the instructions, descriptions, input schemas and annotations of
  `initialize` and `tools/list` equal the blocks and the table of TOOLS.md exactly (the examples compare subsets)

## Steps

1. failing tests
2. the description and the schema of `find`

## Risks

- the shorter description of the spec leaves out details the long one had (field descriptions of the schema); the
  agent learns them from `get_config` and the hints.
