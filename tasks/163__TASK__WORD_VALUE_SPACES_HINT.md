# Task 163: A hint for `/w` with a value that starts or ends with a space

Status: not implemented

Depends on: task 23, task 103

## Goal

`/w` matches a value only between word boundaries, so a value that starts or ends with a space (or another non-word
character) next to a word character can never match: `'fn ' /w` needs a non-word character after the space, but
`fn main` has `m` there. The agent meant "the word `fn`" and wrote the space to say so. `grep -w 'fn '` behaves the
same, but grep has no funnel to explain it. Found in run 8 of the experiment (EXPERIMENT.md, task 158, the
easy-peasy arm), reproduced on 0.154.0:

```
find  query = "path ~ 'src/main.rs' and line(text has 'fn ' /w)"
      output = {"detail": "count", "highlight": "none"}

files: 0  lines: 0  result: r31
funnel:
  path ~ 'src/main.rs'                   1
  and line(text has 'fn ' /w)            0
```

The funnel shows where the files were lost but not why; the agent rewrote the query blindly. A hint with the value
trimmed (`text has 'fn' /w`) saves that call and the guessing (~100–200 tokens per occurrence).

## Scope

A spec change first (its own commit; QUERY.md, the `/w` modifier, and OUTPUT.md, the hints of empty results), then
the code.

- an empty result whose condition has `/w` and a value with leading or trailing whitespace: a hint with the value
  trimmed; with task 103's variations, the trimmed condition is tried like the other variations and its count shown
- the spec step decides: other non-word characters at the edge (`'foo(' /w`, `'.bar' /w`), which match only before
  or after another non-word character; probably whitespace only
- a non-empty result is unchanged: no hint

## Out of scope

- changing what `/w` matches

## Examples

`163__EXAMPLES__WORD_VALUE_SPACES_HINT.md` covers:

- `text has 'fn ' /w` with no match: the hint and the count of `text has 'fn' /w`
- a leading space, a space on both sides, inside `line(...)`
- the same value without `/w`: no hint; a value with spaces inside (`'pub fn' /w`): no hint
- a non-empty result: no hint

## Acceptance criteria

- the spec states the hint before the code is written
- every example in `163__EXAMPLES__WORD_VALUE_SPACES_HINT.md` passes as a test; the examples of all earlier tasks
  still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
