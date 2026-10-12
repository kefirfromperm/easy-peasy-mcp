# Task 132: `highlight = auto` without markers which outgrow the line

Status: not implemented

Depends on: task 87, task 131

## Goal

The default text output doesn't blow up a line with many small matches. An agent gets the lines it asked for at
about their own size, without having to know `highlight = "none"`.

Found by trying the server: `line(text starts '|' and not text has '✅')` over a Markdown table marked every `|` as
`⟦#2,#3:|⟧` — a table separator line of 34 characters became 74, and a response of a few table rows cost
several times the tokens of the rows. With `highlight = "none"` the same response was a fraction of the size. Short,
frequent fragments (a delimiter, a single character, `/w` words in a dense line) are where markers cost the most and
say the least: the line is already shown as a matched line (`42:`) and the file lists its conditions (`[#1 #2]`).

## Scope

A spec change first (its own commit), then the code:

- [OUTPUT.md#highlight](../OUTPUT.md#highlight): in the text format `auto` is `markers`, except for a matched line
  whose markers would add more characters than the line has: it is written as with `none`, the line as it is
- the rule is per line: other lines of the same file keep their markers; a repeated line
  ([OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines)) follows the rule of its text
- the characters of the line are counted after [dedent](../OUTPUT.md#dedent) and the long line cut, the characters
  of the markers with their ids and escape levels
- `highlight = "markers"` set explicitly keeps all the markers; `none` and `columns` are unchanged
- the `marker_in_content` warning is given only for lines written with markers
- the tool description (TOOLS.md#description) says `⟦ids:text⟧ a match` still; one more word if needed, e.g.
  "unless the markers would outgrow the line"

## Out of scope

- JSON (`auto` is `columns` there: `fragments` are data, not markup)
- a configurable threshold
- marking fewer fragments, e.g. leaving out the inner conditions of `line()` or merging adjacent fragments

## Spec

- [OUTPUT.md#highlight](../OUTPUT.md#highlight)
- [CONFIG.md#output](../CONFIG.md#output) (`highlight`)
- [TOOLS.md#description](../TOOLS.md#description)

## Examples

`132__EXAMPLES__AUTO_HIGHLIGHT.md` covers:

- a table: separator and header lines written plain with `auto`, rows with few matches keep their markers
- the boundary: markers which add exactly as many characters as the line has keep the markers, one more drops them
- the same request with `highlight = "markers"`: every fragment marked
- a repeated line and a dedented line
- a line with a marker character in it, written plain: no `marker_in_content` warning for it

Earlier examples whose responses change by the rule are fixed, with the reason in the commit message
(AGENTS.md#2-examples).

## Acceptance criteria

- every example in `132__EXAMPLES__AUTO_HIGHLIGHT.md` passes as a test; the examples of all earlier tasks still pass
- a feature change: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
