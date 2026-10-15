# Task 147: The indentation written once per change

Status: implemented, see [147__SUMMARY__INDENT_CARRIED.md](147__SUMMARY__INDENT_CARRIED.md)

Depends on: task 84, task 141

## Goal

[Dedent](../OUTPUT.md#dedent) writes `⇥N` at the start of every block. Without context lines (the default, `context
= 0`) every matched line which is not next to another one is a block of its own, so most matched lines pay a line
`⇥4` — about as much as the four spaces it removes, often more. The same indentation is written again and again in a
file:

```
lib.rs  lines: 2  matches: 2
⇥4
2:let raw = read().⟦unwrap(⟧);
⇥4
4:parse(&text).⟦unwrap(⟧)
main.rs  lines: 1  matches: 1
⇥4
2:let cfg = load().⟦unwrap(⟧);
```

Found while working on task 143: a search for definitions answered with `⇥4`, `⇥4`, `⇥12` before single lines.

With `⇥N` written only where it changes, the example above has one `⇥4` per file, or one for the whole response if
the spec step carries it across files.

## Scope

A spec change first (its own commit; OUTPUT.md#dedent), then the code.

- spec 2: `⇥N` holds for the following blocks until the next `⇥N`; it is written only when the removed indentation
  differs from the one in effect; `⇥0` is written where a block has no common whitespace after one which had
- the spec step decides: whether the indentation in effect starts at 0 in every file or carries across files (and
  directory headers, repeated lines, pages of a cursor); how it reads with `omit = ["gaps"]` and the repeated lines
- JSON keeps `"indent"` on every block (nothing to save there) unless the spec step finds otherwise
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- what `dedent` removes; the `indent` part of `omit` (it still leaves the markers out)

## Examples

`147__EXAMPLES__INDENT_CARRIED.md` covers:

- one-line blocks with the same indentation in one file and in two files
- blocks with different indentation, a block without common whitespace after an indented one (`⇥0`)
- context lines, `omit = ["indent"]`, a repeated line between blocks, a second page of a cursor
- `"version": 1`, unchanged

## Acceptance criteria

- the spec says where `⇥N` is written before the code is written
- every example in `147__EXAMPLES__INDENT_CARRIED.md` passes as a test; the examples of all earlier tasks still pass
  (a spec 2 example which changes is fixed with the reason in its commit)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
