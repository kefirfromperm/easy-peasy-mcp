# Plan for task 132: `highlight = auto` without markers which outgrow the line

## Design

The spec change is committed already ([OUTPUT.md#highlight](../OUTPUT.md#highlight)): in the text format `auto`
writes a matched line plain when its markers would add more characters than the line has, the lines of a multiline
fragment counted together.

Today `engine::highlight` resolves `auto` to `Highlight::Markers` in text, so the output can't tell an explicit
`markers` from `auto`. A new variant keeps the difference:

```rust
pub enum Highlight<'a> {
    None,
    Columns,
    Markers(&'a str, &'a str),
    /// `auto` in text: markers, except for the lines they outgrow.
    Auto(&'a str, &'a str),
}
```

- `Highlight::markers()` returns the two markers of `Markers` and `Auto`, so the code which writes markers
  (`write_styled`, `write_cut`, the path fragments) treats both alike
- `lines::render` decides per line before writing it: a new `lines::plain_lines(content, parts, block, max)` returns
  the lines of a block written plain. It renders every line as shown (after the dedent `skip` and the long line cut,
  with its `…`) once without and once with markers and compares the characters; lines joined by a part which goes on
  in the next line form one group with their sums. A plain line is written with `Highlight::None`
- `lines::shown_line` (the repeated lines) applies the same rule to its one line
- `Response::all_warnings`: with `Auto` a matched line counts for `marker_in_content` only when it is written with
  markers, judged in the blocks of the file (`blocks::blocks` with the response's options)
- `json.rs` never gets `Auto` (`auto` is `columns` in JSON); its matches compare with `None` and `Columns` and stay
- `engine::highlight` returns `Auto` for `auto` in text

## Rejected

- a flag next to `Highlight::Markers`: every place which builds a `Highlight` (the tests too) would have to set it;
  a variant is the place where the difference is
- deciding by fragment counts instead of characters: the spec counts characters, ids like `#2,#3` and escape levels
  make the markers of the same count differ
- deciding the multiline lines one by one: a marker would open on a written line and close on a plain one

## Tests

- E132.1–E132.7 (`tests/examples.rs`)
- unit tests in `src/output/lines.rs`: a cut long line counts its `…` and its windows only; a multiline group is
  decided together; `Markers` keeps every marker
- earlier examples whose responses change by the rule are fixed in their own commit, with the reason
- the examples of every earlier task still pass

## Steps

1. red: the examples (committed) and the unit tests
2. `Highlight::Auto`, `markers()`, `plain_lines`, render and `shown_line`, the warning
3. fix the earlier examples which change
4. summary, status `implemented`, minor version bump `0.123.0` → `0.124.0`

## Risks

- every matched line is rendered twice more with `auto`: a cost proportional to the output, which is limited by
  `max_output`
- the warning judges the lines in the full blocks of the file, while limits may leave some of them out: such a line
  is not written at all, as before the warning counts it
