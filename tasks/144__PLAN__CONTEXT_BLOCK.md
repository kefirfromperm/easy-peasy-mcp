# Plan for task 144: Context to the end of the block: `output.after = "block"`

## Design

- `config/keys.rs`: a new value type `Type::After`, a whole number or `"block"` (`Value::Whole` or
  `Value::Text("block")`), with the messages `a whole number or "block"`; the key `output.after` takes it. A new omit
  part `more_lines` after `indent` (`output/omit.rs`)
- `config/mod.rs`: `Settings::after()`: `After::Lines(n)` / `After::Block` / not set
- `output/blocks.rs`: `BlockOptions::after_block: bool`; `block_end(content, m)` finds the end of the block of a matched
  line by the rule of OUTPUT.md#block-context (indentation in leading spaces and tabs, closing lines `}` `)` `]` `</`
  `end` `fi` `done` `esac`, a closing line followed by a more indented line goes on); `blocks()` takes it as the end
  of the after context of every matched line, the ranges merge as before. `Block::more`: the lines of the file left
  out of a block by `until` in block mode, set by `before()`
- `output/lines.rs` (text) writes `… N more lines of the block` after the last line of a block with `more > 0`,
  unless `more_lines` is omitted; `output/json.rs` writes `"more_lines": N`
- `output/budget.rs`: level 2 turns `after_block` off with the other context
- `engine.rs`: `block_options` reads the new setting; the `gaps` rule counts the block as context
- `description.rs` / TOOLS.md: the schema of `after`

Decisions (in the spec commit): a value of `after`, not a new option (the task title, one place for the context after a
line; the error text of an invalid `after` changes, the earlier examples are fixed as with task 136); `context` stays a
number; a line which opens no block gets no context; not a breaking change (a new value of an option).

Rejected: a separate cap like `max_block_lines`: the limits per file and of the response already cut, and the line
`… N more lines of the block` tells the agent exactly what to read.

## Tests

- unit: `blocks` — the end of a Rust, a Python and a Markdown block, a closing line which opens again, a match on a
  closing line, merging, `more` after a cut; `keys` — the type `After` in TOML and JSON
- examples: E144.1–E144.16; the earlier examples with the changed messages (E82.5–E82.7, the omit lists)

## Steps

1. red: the unit tests; the example tests run from the examples file
2. green: the type, the setting, the block end, the cut line in text and JSON, the budget, the schema
3. fill in the cursor of E144.11 and the budget response of E144.14 from the run, checked by hand
4. summary, status `implemented`, version 0.141.0

## Risks

- tabs and spaces mixed in one file: a tab counts one, so a block may end early; the spec says so
- a long block in a long file: the end is found by one pass over the lines after the match, merged blocks may scan the
  same lines again; fine for files within `max_file_size`
