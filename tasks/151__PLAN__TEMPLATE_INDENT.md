# Plan for task 151: The indentation of the line in a replace template

## Decisions

The spec step ([TOOLS.md#templates](../TOOLS.md#templates)) decided:

- the name is `{indent}`: a placeholder without `@` or `#`, so it can't clash with a group reference; it works in the
  template of every group
- it is the whole run of spaces and tabs at the start of the line where the match starts, also when the match starts
  after it or inside it (`^\s*` in the match); other whitespace (a no-break space) is not indentation
- a multiline match takes the indentation of its first line
- a line break of the template is not followed by the indentation on its own. Rejected: indenting every new line of
  the template — a template which writes its own indentation (`\n    x`) would get it twice, and the agent can't opt
  out
- `/e` doesn't escape it (it is text of the file, like the placeholders of the match); `/n` restyles the whole new
  text into an identifier, so `{indent}` is empty there — the new text of a `/n` match keeps no spaces or tabs
- the hint of `unknown placeholder` and `empty placeholder` lists `{indent}` after the groups; the hint of
  `'{@x}' is not '@old' or its sub-group` stays the list of the group's placeholders — the agent meant a group there,
  and E129.12 (spec 1) keeps its response

## Design

`src/replace.rs`:

- `Part::Indent`, a new variant of the template part; `Template::parse` maps the name `indent` to it
- the hint closure takes whether to add `{indent}`: yes for the unknown and the empty placeholder, no for a wrong
  group reference
- `Template::expand` gets the indentation as a `&str` argument; the caller computes it once per match
- `fn indentation(text: &str, start: usize) -> &str`: from the start of the line of `start` (after the last `\n`
  before it) the run of `' '` and `'\t'`; the content text is normalized to `\n`, so `\r\n` files work too
- `replacements`: the indentation of `f.range.start`, or `""` when the match is restyled by `/n`

`src/description.rs`: the line of `{indent}` in `REPLACE_DESCRIPTION`, the same as TOOLS.md#replace (checked by
`tests/mcp_stdio.rs`).

No new modules, types beyond the variant, or dependencies.

## Tests

- example tests: E151.1-E151.9 (startup examples, `tests/startup.rs`)
- unit tests in `replace.rs`:
  - `indentation`: spaces, tabs, mixed, a line without indentation, a match inside the indentation, the first line of
    the text, a match which starts at the end of a line (its `\n`)
  - `templates_and_their_errors`: `{indent}` expands, the hints with and without `{indent}`
  - `/n` with `{indent}`: the indentation is left out; `/e`: the indentation is not escaped

## Steps

1. red: the unit tests (examples are committed already and fail)
2. green: `Part::Indent`, `indentation`, the hints, the description line
3. refactor, fmt, clippy, the whole test suite
4. summary, status, version `0.N+1.0`

## Risks

- the description of `replace` is compared with TOOLS.md byte for byte: both changed in the same way
- a fragment's range is in the normalized text; `indentation` must use the same text as `expand`
