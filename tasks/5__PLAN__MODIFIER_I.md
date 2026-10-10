# Plan for task 5: `/i` modifier on `name` and `path`

## Design

- The parser already reads modifiers after any condition (`/i`, `/w`, `/m`, `/c`, `/n`, `/e(style, levels)`, any
  case, with or without spaces); an unknown one is `unexpected_token` with the list of modifiers.
- `query/mod.rs` validation, for every condition of the query (also outside of the implemented subset), with the
  scope (file or inside `line( ... )` / `near( ... )`):
  `modifier_not_allowed`, message `modifier '/x' is not allowed here`, position the modifier, hints

  | Modifier | Allowed                                        | Hint                                                         |
  |----------|------------------------------------------------|--------------------------------------------------------------|
  | `/i`     | text operators on `path`, `name`, `text`       | `'/i' works with the text operators on 'path', 'name' and 'text'` |
  | `/w`     | `=`, `has`, `starts`, `ends`, `=~` on `text`   | `'/w' works with '=', 'has', 'starts', 'ends', '=~' on 'text'` |
  | `/m`     | `=~` on `text` in file scope                   | `'/m' works with '=~' on 'text' in file scope`               |
  | `/c`     | all operators on `text`                        | `'/c' works with 'text'`                                     |
  | `/e`     | `=`, `has`, `starts`, `ends` on `text`, not with `/n` | `'/e' works with '=', 'has', 'starts', 'ends' on 'text'`; with `/n`: `'/e' can't be combined with '/n'` |
  | `/n`     | `=`, `has`, `starts`, `ends` on `path`, `name`, `text` | `'/n' works with '=', 'has', 'starts', 'ends'`        |

- errors are ordered by position; on the same position a real error wins over `not_implemented`.
- `query/subset.rs`: `/i` (any case) is allowed after the value; other modifiers are `not_implemented`.
- compile: `/i` makes the matcher case-insensitive (`(?i)`, Unicode simple case folding), whatever `path_case` is.
- `description.rs`: `/i ignore case`.

## Tests

- examples E5.1–E5.7
- unit: `/i` matchers (`=`, `has`, `~`, non-ASCII); `modifier_not_allowed` for each modifier with its hint, `/m`
  inside `line( ... )`, `/e` with `/n`; the subset accepts `/i` and rejects `/w`; the legend keeps `/i/w` and `/i /w`

## Steps

1. failing unit tests
2. validation, subset, compile
3. description

## Risks

None known.
