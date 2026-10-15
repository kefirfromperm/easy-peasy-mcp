# Plan for task 147: The indentation written once per change

## Design

- `output/lines.rs`: `render` takes `carry: bool` (spec 2). It keeps the indentation in effect, 0 at the start of
  the call, which is one file: with `carry` a block writes `⇥N` only when its indentation differs from the one in
  effect (`⇥0` after a dedented block), without it every block with an indentation above 0 writes it, as before.
  `omit = ["indent"]` writes none, as before
- `output/mod.rs`: passes `self.spec >= 2`
- JSON, the repeated section and `only_match` are unchanged: they have no `⇥N` lines or keep `indent` per block

Rejected: carrying the indentation across files — a file would not read on its own and a page of a cursor would
depend on the page before it.

## Tests

- unit: `lines` — the carried indentation over blocks of 4, 4, 8, 0; without `carry` the old output
- examples: E147.1–E147.7; earlier spec 2 examples with several dedented blocks of the same indentation in a file
  change and are fixed in their own commit with the reason

## Steps

1. red: the unit test; the examples already fail on E147.*
2. green: `render` with `carry`
3. fix the earlier spec 2 examples
4. summary, status `implemented`, version 0.144.0

## Risks

- many earlier spec 2 examples show `⇥4` twice in `lib.rs` of the fixture `omit`: they change, mechanically
