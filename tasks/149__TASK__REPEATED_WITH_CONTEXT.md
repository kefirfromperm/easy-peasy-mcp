# Task 149: A repeated line keeps the context which was asked for

Status: implemented, see [149__SUMMARY__REPEATED_WITH_CONTEXT.md](149__SUMMARY__REPEATED_WITH_CONTEXT.md)

Depends on: task 80, task 86, task 138

## Goal

A [repeated line](../OUTPUT.md#repeated-lines) is printed once in the `repeated` section, and "the context of a
repeated line goes with it": the context lines an agent asked for with `context`, `before` or `after` are not shown at
all. When the matched line is only an anchor (a fence, a heading, `#[test]`, `Status:`) the context is what the agent
wanted, and every file has it different.

Found in the experiment of task 144: a search for a fence line ```` ```text tree ```` with context, in 105 files, put
every match into `repeated:` and showed none of the context; the agent had to run it again with `group_repeated = 0`.

## Scope

A spec change first (its own commit; OUTPUT.md#repeated-lines), then the code.

- spec 2: a matched line with context lines is grouped only when its block (the line and its context) is the same in
  the files; otherwise it is written with its context under its file
- the spec step decides: "the same block" (ignoring leading and trailing whitespace like the line); whether a block
  which is the same is grouped with its context in the `repeated` section; `after = "block"` (task 144); the
  counts and `max_output` of such lines
- spec 1 is unchanged; a change of a spec 2 rule is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- lines without context; `repeated_locations`

## Examples

`149__EXAMPLES__REPEATED_WITH_CONTEXT.md` covers:

- a line in 4 files with different context: written under every file with its context
- a line in 4 files with the same context: grouped
- `before` only, `after` only; `context = 0`, unchanged
- `"version": 1`, unchanged

## Acceptance criteria

- the spec says when a line with context is grouped before the code is written
- every example in `149__EXAMPLES__REPEATED_WITH_CONTEXT.md` passes as a test; the examples of all earlier tasks
  still pass (a spec 2 example which changes is fixed with the reason in its commit)
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
