# Plan for task 128: Query groups

## Design

### Bracket groups

- `ast::NodeKind::Group(Box<Expr>)`: a bracket with a label is a node with an id, like `line( ... )`; a bracket
  without a label stays `Expr::Group`. `Node::tokens` of a group spans `(` to `)`, so the legend text is the bracket
  as written without the labels inside (`legend_text` already skips labels), and its order is the index of `(`: the
  group comes before the conditions inside it in the legend and in the ids of a fragment.
- parser: after `( or_expr )` an optional label makes the node.
- `assign_ids`: a group always has a label, so it never takes a number.
- validation: the inside of a group is validated in the scope of the group (file, `line`, `near`).
- `duplicate_labels`: the labels are sorted by their position, a later use is the duplicate (the group label is
  visited before the labels inside it, but written after them).
- `eval::Filter::Group { id, order, inner, members }`: `eval` and `eval_line` are those of `inner`; `collect`
  (file conditions) lists the group and goes on inside it; `collect_line` goes on inside it. `members` are the orders
  of every condition inside it, computed when compiling.
- fragments: after the fragments of a file are made (`search::file_matches`, `PathFragments::of`), every group whose
  members include an id of a fragment adds its id to the fragment (`Filter::group_ids`). A condition which produced a
  fragment contributed, so the groups around it contributed too.
- the other `match`es on `NodeKind` / `Filter` (narrowing, needs, has_content, relevance, funnel) treat a group as its
  inside.

### Regex sub-groups

- `lines::Fragment` gets `groups: Vec<SubGroup>`: `SubGroup { range, ids: Vec<String> }`, the byte range in the
  content and the references (`@def.1`, `@def.fname`).
- `TextPattern::sub_groups(hay, start)` returns the capture groups (number, name, range) of the match at `start`; only
  for `=~` (`TextPattern::captures` set when compiling `text =~` with capture groups). `file_matches` and
  `line_fragments` add them with the id of the condition; `merge_identical` joins the groups of merged fragments.
- `output::json`: a fragment gets `groups`, every group with `col` (in the line where it starts), `n` when that line is
  not the line of the fragment, `text`, `ids`, `to`. Left out with `omit groups`; the `ids` with `omit match_ids`.
- the text format is unchanged.

### Group references

- `Query::group(reference) -> Result<GroupRef, QueryError>`: parses `@name`, `#3`, `@name.1`, `@name.fname`, checks
  it against the legend and the sub-groups of the `text =~` conditions; `unknown_group` with the span in the reference
  and the groups of the query as the hint. `Query` keeps `sub_groups: Vec<(CondId, Vec<(usize, Option<String>)>)>`.
  Used by tasks 129 and 130; unit tested here.

### Omit and description

- `omit::Part::Groups` after `MatchIds`; the lists of the parts in the error messages and the schema.
- the `find` description: bracket labels on the `Labels:` line.

## Decisions

- the group id before the inner ids (`⟦@ser,#1:serde⟧`), not after as in the sketch of the task: the order of
  appearance of `(`, like `line( ... )` whose id comes before the ids inside it; the legend and the markers agree.
- the id of a group is added to every fragment of a condition inside it, path fragments included: one rule, no
  exceptions; metadata conditions have no content fragments anyway.
- sub-groups only for `text =~`: `name =~` / `path =~` are out of scope.
- rejected: a separate `Expr::Labeled` wrapper — the node already carries id, label, tokens and the ordering.

## Tests

- unit: parser (a labelled bracket is a node, without a label a plain group; a second label is an error), ids and
  legend, duplicate labels by position, `Filter::Group` eval / contributors / line eval, group ids of fragments,
  `sub_groups` of a regex (participating and non-participating groups, names), `Query::group` (valid references,
  unknown id, unknown sub-group, sub-group of a non-regex), JSON of a fragment with groups and `omit groups`
- examples E128.1–E128.11; E117, E131, E134 updated for the description and the omit list

## Steps

1. AST, parser, ids, validation, duplicate labels
2. `Filter::Group`, compile, eval, contributors, fragments with group ids
3. sub-groups: pattern, fragments, merge, JSON, omit
4. references: `Query::group`, `unknown_group`
5. description, schema, docs; fmt, clippy, tests, release build

## Risks

- `Fragment` is built in many places: the new field touches all of them (`groups: Vec::new()`)
- the order of ids in the fragments relies on the order of `(`, which must be smaller than the order of the conditions
  inside: it is a token index, so it is
