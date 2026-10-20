# Summary of task 159: Long changed lines of `replace` cut around the change

Server version: `0.156.0`.

## What was built

- In spec 2 a changed line of `replace` longer than `output.max_line_length` (the configuration, 300) is cut like a
  [long line](../OUTPUT.md#long-lines) of `find` ([TOOLS.md#result-6](../TOOLS.md#result-6)): the old line around
  the parts of the replaced matches in it, the new line around the parts of their new texts. Every part stays whole
  with `(max_line_length − characters of the parts) / (2 × parts)` characters on both sides, touching windows are
  one, `…` marks the cuts. A deleted match is a part of no characters at its place; a line without a part keeps its
  first `max_line_length` characters; a short line is written whole. The dry run and the real call write the same.
- `output.max_line_length` is a new field of `replace` (at least 20, checked like in `find`); it is in the input
  schema without a description, so the schema stays under its 1900 characters.
- Spec 1 writes the lines whole unless the request sets `output.max_line_length`; JSON writes them whole (its `old`
  and `new` are read by code). `rename`, `copy` and `delete` write paths and are unchanged.
- The one-line JSON of the finding (a 10K line, a change of 20 characters) now costs ~700 characters per call instead
  of ~10K, in the dry run and again in the real call.

## Architecture

- `replace::Change` has `old_parts` and `new_parts`: per line, the byte ranges of the replaced matches and of the new
  texts in it. `replace::changes` computes them from the replacements and their shifted ranges in the new text;
  `parts_in` clips a range to a line and keeps an empty range in it. `Change::lines` iterates the `-` and `+` lines with
  their parts.
- `output::lines::windows` takes plain byte ranges; `cut_line` of `find` filters its fragments to the line itself, and
  the new `cut_plain(line, parts, max)` writes the windows of a plain line. One rule, one implementation.
- `write::Outcome::max_line_length` (`None`: whole) is set by `engine` from the settings of the request: in spec 2
  always, in spec 1 only when the request has `output.max_line_length`. `to_text` cuts, `to_json` doesn't.

## Deviations from the plan

- the description of `max_line_length` in the input schema was dropped (its own spec commit): with it the schema of
  `replace` had 1933 characters, over the limit of `description::tests::schemas_stay_small`
- the loop over the changed lines moved to `Change::lines`, so `Outcome::to_text` stays under clippy's line limit

## Tests

- examples E159.1–E159.8 (startup examples): one match in a one-line JSON (dry run, real call, JSON whole), a match
  near the start with a short line written whole, a match near the end, `max_line_length` of the request with a match
  at the end of the line, two matches far apart, a deleted match, spec 1 with and without `max_line_length`
- unit tests: `replace::tests::runs_of_changed_lines` (the parts of one and two matches, a deletion, an insertion, a
  match over a line break), `output::lines::tests::plain_lines_cut_around_their_parts` (multibyte characters, an
  empty part, windows at the start and merged), `write::tests::long_changed_lines_cut_in_text_only`
- the schema test of `tests/mcp_stdio.rs` compares the new schema with TOOLS.md; all earlier examples pass unchanged

## Limitations and known gaps

- a line of a run touched only by the line break at the end of a match (a match ending with `\n`) has no part and
  keeps its first `max_line_length` characters
- `output.max_lines` still counts lines, not characters: many long lines are bounded by both limits together

## Try it

`easy-peasy-mcp --allow-write`, then `replace` with `{"query": "path = 'data/tools.json' and text has 'uncommitted
changes' @old", "replace": {"@old": "changes"}, "dry_run": true}` on a one-line JSON: the `-1:` and `+1:` lines are
about 300 characters each, `…` at both ends.
