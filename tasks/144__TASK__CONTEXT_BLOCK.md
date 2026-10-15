# Task 144: Context to the end of the block: `output.after = "block"`

Status: in progress

Depends on: task 80, task 134

## Goal

An agent which finds a definition (`text =~ `fn\s+page\(``) usually wants its body. Today it has two ways, both
costly:

- a second call: `Read` of the file with an offset and a guessed limit, or `sed -n '/fn page(/,/^}/p'`; one more
  round trip, and the guessed limit reads too much or too little
- a big `after = 40` "to be safe": every match gets 40 lines, most of them after the end of the body, and a short
  function next to a long one gets the same 40

Found while working on task 135: reading `fn page`, `fn request_hash`, `struct Run` and `search_all` of the
engine took a `sed -n '/start/,/^}/p'` call each, after the `find` which had already found the line.

With `output.after = "block"` the context after a matched line goes on to the end of its block: the lines more
indented than the matched line, and the first line after them at its own indentation which closes the block
(`}`, `)`, `]`, `end`, ...). One call, exactly the body.

## Scope

A spec change first (its own commit; OUTPUT.md#options, a new section under OUTPUT.md#compact-output or near the
context options). The spec step decides:

- the value: `after = "block"` (and `context = "block"`? `before` has no block), or a new `output.block = true`
- the end of a block by indentation, the language-free rule: the lines after the matched line until the first
  non-blank line indented at most as much as the matched line; that line is included when it is a closing line
  (only closing brackets, `end`, `fi`, `done`, ...: a list in the spec or in the language table of CONFIG.md)
- a matched line which opens no block (the next non-blank line is not more indented): no context after it, or one
  line, as the spec says
- a cap so a huge block can't flood the response: `max_lines` and `max_output_per_file` apply to the context lines
  as they do now; a cut block says so (`… N more lines of the block`, its own [`omit`](../OUTPUT.md#omit) part:
  **every part of the output is optional**)
- blocks of several matches in one block merge like overlapping context now; `--` between blocks, `⇥N` dedent as now
- the budget levels (OUTPUT.md#budget): a lower level drops the block context first
- JSON: the context lines of the block as the context lines now
- a breaking change? A new value of an option is not one (TOOLS.md#versioning); decided in the spec step

## Out of scope

- blocks by the syntax of a language (a parser, brace matching across strings and comments); indentation only
- `before = "block"` (the head of an enclosing block: the function a match is in) — a separate task if wanted
- the write tools

## Spec

- [OUTPUT.md#options](../OUTPUT.md#options), [OUTPUT.md#compact-output](../OUTPUT.md#compact-output),
  [OUTPUT.md#limits-and-truncation](../OUTPUT.md#limits-and-truncation), [OUTPUT.md#budget](../OUTPUT.md#budget),
  [OUTPUT.md#omit](../OUTPUT.md#omit)

## Examples

`144__EXAMPLES__CONTEXT_BLOCK.md` covers:

- a Rust function, a Python function (no closing line), a Markdown section is not a block (no indentation)
- two matches in one block, a match inside a nested block, a match on the closing line
- a block longer than `max_lines`, with `max_output_per_file`, with a budget
- `after = "block"` with `before = 2`, with `dedent = false`, in JSON, with `omit` of the new part

## Acceptance criteria

- the spec describes the block context before the code is written
- every example in `144__EXAMPLES__CONTEXT_BLOCK.md` passes as a test; the examples of all earlier tasks still pass
- a new feature: the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
