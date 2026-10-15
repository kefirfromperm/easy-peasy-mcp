# Summary of task 147: The indentation written once per change

Server version: `0.144.0`.

## What was built

- In spec 2 a `⇥N` holds for the blocks after it until the next one ([OUTPUT.md#dedent](../OUTPUT.md#dedent)):
  it is written only where the removed indentation differs from the one in effect; `⇥0` where a block has no common
  whitespace after a dedented one.
- Every file starts with the indentation 0: nothing carries over to the next file, a directory header, the repeated
  section or a page of a cursor, so every file reads on its own.
- The gaps `--` stay as they were; `omit = ["indent"]` leaves every `⇥N` out; JSON keeps `"indent"` on every block.
- It is a rule of the defaults of spec 2 ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)):
  `show = ["indent"]` and `show = "all"` write every `⇥N`, as spec 1 does, which is unchanged.
- Spec: two commits on the branch — the rule, then `show` bringing every `⇥N` back. A change of a spec 2 rule,
  allowed since spec 2 has not been used outside this repository (TOOLS.md#versioning).

## Architecture

- `output::lines::render` takes `carry`: it keeps the indentation in effect over the blocks of one file and writes
  `⇥N` when a block differs from it; without `carry` every block above 0, as before.
- `Response::carry_indent`, set by `engine.rs` to spec 2 without `indent` in `show`.

## Deviations from the task and the plan

- The plan passed `spec >= 2` to `render`. The startup example E141.11 (`show = "all"` in the config) showed that
  `show = "all"` would no longer be the output of spec 1, as the spec promises; so the rule yields to `show` like the
  other rules of spec 2 (spec commit, `Response::carry_indent`, E147.8). E141.6 step 3 (`show = "all"`), changed by
  mistake in the first fix of the examples, was restored.

## Tests

- examples E147.1–E147.8 (new fixture `indent`): one file and two files with the same indentation, 4/8/4/0, context
  lines with a gap, `omit = ["indent"]`, a repeated line between blocks, a page of a cursor, spec 1, `show`
- unit `output::lines::tests::the_indentation_is_written_where_it_changes`
- earlier spec 2 examples which repeated `⇥4` in a file, fixed in their own commit (only `⇥N` lines removed, checked by
  a script): E141.1, E141.2, E141.6, E141.7, E141.9, E143.1, E143.3, E143.4, E143.5, E146.1, E146.3, E146.4

## Limitations and known gaps

- none known

## Try it

`{"query": "text has 'fn ' and lang = rust", "dirs": ["src/output"]}` — one `⇥4` per file instead of one per line.
