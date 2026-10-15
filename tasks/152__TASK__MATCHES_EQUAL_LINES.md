# Task 152: The number of matches left out when it is the number of lines

Status: not implemented

Depends on: task 134, task 141

## Goal

Most searches find one match per line: a name, a call, a key. Then every file of the result pays `  matches: N` with
the same number as its `lines: N`, and the header pays it once more. For a list of files, the most common answer to
"where is X", that is about a third of every line of the listing, and it carries nothing.

Found while working on task 145: `line(text has 'group_dirs')` over `src` answered 10 files, all of them with the
same two numbers:

```
files: 10  lines: 29  matches: 29

src/
description.rs  lines: 4  matches: 4
engine.rs  lines: 1  matches: 1
write.rs  lines: 4  matches: 4
...
```

Without them it would be `description.rs  lines: 4`: 12 characters, about 3 tokens, less per file and the same
information, once the agent knows the rule.

## Scope

A spec change first (its own commit; OUTPUT.md#counts, OUTPUT.md#defaults-of-spec-2, the tool description). The spec
step decides:

- the rule in spec 2: `matches: N` of a file is left out when it is its `lines: N`; the header's `matches: N` when it
  is the header's `lines: N`; or only when it is true for every file of the result (all its files, so the pages of a
  cursor agree), so the agent never has to tell a left-out count from a missing one
- how the agent learns the rule: the tool description (one line under OUTPUT) and OUTPUT.md; whether the header says
  it (e.g. `files: 10  lines: 29` without `matches` means one match per line)
- the part: a new `omit` part (`equal_matches`?) in the spec 2 default of `omit`, so `show = ["equal_matches"]` (and
  `show = "all"`, spec 1) brings the numbers back; or a rule of `file_counts`, like the legend rule of task 143
- `count_by = line` (`  2: 1`): whether `: 1` of a line with one match is left out too (then `count_by = line` is the
  list of the line numbers, as with `line_counts` omitted)
- JSON: whether `"matches"` stays (a missing number in JSON costs more to explain than it saves; the text format is
  where the tokens are)
- spec 1 is unchanged; a change of the spec 2 defaults is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- the counts of the directory headers or of facets
- any other count of the header (`files:`, `lines:`)

## Spec

- [OUTPUT.md#counts](../OUTPUT.md#counts), [OUTPUT.md#omit](../OUTPUT.md#omit),
  [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2), [TOOLS.md#versioning](../TOOLS.md#versioning)

## Examples

`152__EXAMPLES__MATCHES_EQUAL_LINES.md` covers:

- a result with one match per line in every file: `files`, `lines` and `count` with `count_by = file` and `line`
- a result where one file has two matches in a line: what the decided rule writes for it and for the others
- a cursor whose first page has one match per line but the result doesn't (if the rule is over the result)
- `show` of the part, `"version": 1`, JSON

## Acceptance criteria

- the spec says when the number is left out and how the agent knows, before the code is written
- every example in `152__EXAMPLES__MATCHES_EQUAL_LINES.md` passes as a test; the examples of all earlier tasks still
  pass (a spec 2 example which changes is fixed with the reason in its commit, as the versioning rule allows)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
