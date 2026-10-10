# Plan for task 6: Logical operators `not`, `and`, `xor`, `or` and brackets

## Design

- The parser already builds `Or`, `Xor`, `And`, `Not`, `Group` with the spec precedence and the grammar errors.
- `query/subset.rs` becomes a check of the syntax tree: the first unsupported node by position — a target other
  than `name` / `path`, an operator other than `=`, `has`, `starts`, `ends`, `~`, a modifier other than `/i`, a
  label, `line( ... )`, `near( ... )` — or a comment. The token scan of task 1 can't follow nested expressions.
- `query/mod.rs`: `Filter` becomes a tree: `And`, `Or`, `Xor`, `Not`, `Text` (brackets disappear). Every `Text` keeps
  its order of appearance for sorting the condition list.
- `search`: `Filter::eval(&FileInfo) -> bool`; for a matching file `contributors` walks the tree from the top
  (OUTPUT.md#file-conditions): a true `And` — all parts, a true `Or` / `Xor` — only the true parts, `Not` — nothing.
  Ids are listed in the order of appearance. `Xor` is true for an odd number of true parts.
- three-valued logic comes with the first unknown value (task 13 / 20); now every value is known.
- `description.rs`: `not`, `and`, `xor`, `or` and brackets.

## Tests

- examples E6.1–E6.12
- unit: evaluation and contributions for `and`, `or`, `xor` (chained), `not`, `not not`, nesting; subset check of
  the tree (positions of the first unsupported node in nested expressions)

## Steps

1. failing unit tests
2. subset check on the tree, filter tree, evaluation, contributions
3. description

## Risks

None known.
