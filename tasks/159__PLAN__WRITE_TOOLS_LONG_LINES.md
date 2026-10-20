# Plan of task 159: Long changed lines of `replace` cut around the change

## Design

The spec ([TOOLS.md#result-6](../TOOLS.md#result-6)) cuts a changed line longer than `output.max_line_length` like a
long line of `find`: the old line around the parts of the replaced matches in it, the new line around the parts of
their new texts.

- `replace::Change` gets `old_parts` and `new_parts`: per line, the byte ranges (relative to the line) of the
  replaced matches in an old line and of their new texts in a new line. `replace::changes` computes them from the
  replacements: a replacement's range in the new text is its start shifted by the replacements before it, with the
  length of its new text. A part is clipped to the line; an empty one (a deletion, a new text without characters)
  counts when it lies in the line.
- `output::lines::windows` takes plain byte ranges instead of `Part`s; a new `pub(crate) fn cut_plain(line, ranges,
  max) -> Cow<str>` writes the windows with `…` — the same rule as `find`, one implementation.
- `write::Outcome` gets `max_line_length: Option<usize>`; `to_text` cuts every `-` and `+` line with it, `to_json`
  doesn't (the lines stay whole for code).
- `engine::replace` sets it: spec 2 → `settings.whole(Key::MaxLineLength)` (the request's `output.max_line_length`
  over the configuration); spec 1 → only when the request sets `output.max_line_length`.
- `replace::OUTPUT_FIELDS` gets `max_line_length`; the request is checked by `request::parse` as in `find` (at least
  20). The input schema in `description.rs` gets the field, as in TOOLS.md.

Rejected: one window over the changed span of both lines — it differs from the rule of `find` and the agent would
learn a second one; cutting JSON — its `old` and `new` are read by code that may compare or apply them.

`rename`, `copy` and `delete` write paths, not lines: unchanged.

## Tests

- examples E159.1–E159.8 (startup examples, `tests/startup.rs` reads them)
- unit `replace::tests::parts_of_changed_lines`: old and new parts of one and two matches, a deletion, a multiline
  replacement
- unit `write::tests::long_changed_lines_cut_in_text_only`: `to_text` cuts with `max_line_length`, `to_json` doesn't
- the schema test of `mcp_stdio.rs` compares the new input schema with TOOLS.md

## Steps

1. red: the unit tests, the examples
2. `windows` over ranges, `cut_plain`
3. the parts in `Change`, `max_line_length` in `Outcome`, the engine and the output fields, the schema
4. clippy, fmt, the summary, the version `0.156.0`

## Risks

- byte ranges vs characters: the parts are byte ranges of the line, the windows count characters as in `find`
- an old line of a run without a part of its own (the end of a multiline match at the start of the next line) keeps
  its first `max_line_length` characters, as the spec says
