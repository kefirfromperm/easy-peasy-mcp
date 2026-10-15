# Summary of task 144: context to the end of the block, `output.after = "block"`

Server version: `0.141.0`.

## What was built

- `output.after` takes `"block"` besides a number ([OUTPUT.md#block-context](../OUTPUT.md#block-context)): the context
  after a matched line goes on to the end of the block it opens, found by indentation, the same rule for every
  language. One `find` gives the body of a definition: `{"query": "text has 'fn load('", "output": {"after": "block"}}`.
- The rule: a line opens a block when the next non-blank line is indented more (leading spaces and tabs, a tab counts
  one); the block runs to the first non-blank line indented at most as much, without the blank lines at its end; that
  line is included when it is a closing line indented as much (`}`, `)`, `]`, `</`, the words `end`, `fi`, `done`,
  `esac`), and a closing line followed by a more indented line goes on (`} else {`, `) -> Vec<String> {`). A line
  which opens no block (a closing line, a Markdown heading) gets no context after it.
- The block contexts merge like any context; `before`, `blank_context`, `dedent`, `--` work as with a number;
  `context` stays a number.
- Limits: `max_output_per_file` and `max_output` cut the block context like any context; a cut block ends with
  `… N more lines of the block` (JSON `"more_lines": N` of the block), the new omit part `more_lines`. `max_lines`
  counts matched lines only. The budget drops the block context at its level 2.
- Config: `after = "block"` in the config file and `update_config`; the errors name the new value
  (`'output.after' must be a whole number or "block"`, hint `a whole number or "block" is expected`). The tool schema
  of `after` is `anyOf` a number or `"block"` (TOOLS.md#find).
- Not a breaking change: a new value of an option (TOOLS.md#versioning).
- Spec (own commits): the section `Block context`, the options table, the omit table and its error list, the gaps rule
  of spec 2, CONFIG.md, TOOLS.md.

## Architecture

- `config/keys.rs`: `Type::After` (a whole number or `"block"`), the const `BLOCK`; `output.after` has this type.
- `output/blocks.rs`: `BlockOptions::after` is the enum `After { Lines(n), Block }` (clippy forbids a fourth bool and
  the enum can't be half set); `block_end` finds the end of a block, `is_closing` the closing lines; `Block::end` (the
  last line of a block context) and `Block::more`, set by `before()` when `until` cuts inside it.
- `output/lines.rs` writes the `… N more lines of the block` line, `output/json.rs` the field `more_lines`; both
  respect the omit part `MoreLines` (`output/omit.rs`).
- `output/budget.rs`: level 2 sets `after` to `Lines(0)`. `engine.rs`: `block_options` reads the value, the `gaps`
  rule counts the block as context.

## Deviations from the plan

- The planned `after_block: bool` became the enum `After`: clippy's `struct_excessive_bools`.
- Earlier examples fixed in their own commits: E82.5–E82.7 (the messages of an invalid `after`), the omit part lists
  of the tasks 134, 136, 137, 138, 141, E117.2 (the schema of `after`).
- E144.6 first showed markers on lines of one `}`; `highlight = auto` writes such a line plain (the markers outgrow
  it), as the spec says; the example was fixed.
- clippy 1.99 (the machine's toolchain, newer than the pinned 1.98) denies `assert!(x.is_empty())`; the 18 places in
  the tests compare the length instead, in a commit of their own.

## Tests

- unit: `output::blocks` — a Rust function, a nested block, a line which opens no block, merged blocks, closing lines
  (the list, tabs), a closing line which opens again, a Python function, a Markdown heading, the rest of a cut block;
  `config::keys` — the type `After` in TOML and JSON; `output::omit` — the part `more_lines`.
- examples: E144.1–E144.16 (fixture `block`: `src/lib.rs`, `app.py`, `README.md`).

## Limitations

- Indentation only: a block whose closing line is indented differently from its opening line, or code with mixed tabs
  and spaces, ends where the indentation says. Blocks by syntax are out of scope.
- `before = "block"` (the head of the enclosing block) is not implemented; a separate task if wanted.

## How to try it

```sh
cargo build --release
```

Register `target/release/easy-peasy-mcp` in an MCP client and call `find` with:

```json
{ "query": "text has 'fn load('", "output": { "after": "block" } }
```
