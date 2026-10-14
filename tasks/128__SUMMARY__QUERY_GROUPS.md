# Summary of task 128: Query groups

An agent can name the parts of a query whose matched text it wants: `(text has 'serde' or text has 'serde_json') @ser
and text has 'apple'` makes `@ser` the matched `serde` / `serde_json` text, not `apple`. The capture groups of a
`text =~` condition are its sub-groups (`@def.1`, `@def.fname`), listed per match in the JSON format. The reference
syntax and its error are defined for `output.groups` (task 130) and `replace` (task 129).

## What was built

- **Bracket groups** ([QUERY.md#groups](../QUERY.md#groups)): a label after `)` gives the bracket an id; a bracket
  without a label stays a plain bracket and is not numbered. The grammar has
  `primary = ( "(" or_expr ")" | line | near | condition ) [ label ]`.
- **The id of a group** ([OUTPUT.md#condition-ids](../OUTPUT.md#condition-ids)): in the legend as written with its
  brackets, without the labels inside, in the order of its `(`; in the conditions of a file and the `condition` facet
  when it made the file match at the file level (inside `line( ... )` / `near( ... )` it is not listed per file, like
  the conditions there); in the ids of every fragment of a condition inside it, before their ids: `⟦@ser,#1:serde⟧`,
  path fragments included.
- **A group without fragments**, e.g. of `name` conditions, is a valid condition id without matched text.
- **Labels** are unique across conditions and groups; the duplicate is the later one as written, so in
  `(text has 'a' @x or text has 'b') @x` the group label is marked.
- **Regex sub-groups** ([QUERY.md#regex-groups](../QUERY.md#regex-groups),
  [OUTPUT.md#sub-groups](../OUTPUT.md#sub-groups)): the capture groups of `text =~` which took part in a match are the
  `groups` of its JSON fragment, by number, each `{ "n"?, "col", "text", "ids", "to"? }` with the ids `@def.2` and
  `@def.fname`; `n` when the group starts in another line than the fragment (`/m`), `to` when it ends in another one.
  The text format is unchanged. The fragments of the repeated lines keep their groups.
- **Omit** ([OUTPUT.md#omit](../OUTPUT.md#omit)): the new part `groups` leaves the sub-groups out; `match_ids` leaves
  out their ids too. The part is in the error messages of `output.omit`, in CONFIG.md and in the schema.
- **Group references** ([QUERY.md#group-references](../QUERY.md#group-references)): `@name`, `#3`, `@name.1`,
  `@name.fname`; `unknown_group` with the position in the field and the groups of the query as the hint:
  `the groups of the query: @def, @def.1, @def.2, @def.fname, #1`. No request field takes a reference yet.
- **The `find` description**: the `Labels:` line names bracket groups.

## Architecture

- `query::ast`: `NodeKind::Group(Box<Expr>)`, a node like `line( ... )` whose tokens span the bracket;
  `SubGroup` and `GroupRef` for the references. The parser makes the node when a label follows `)`.
- `query::eval`: `Filter::Group { id, order, inner, members }` evaluates as its inside; `collect` lists the group and
  goes on inside, `collect_line` only goes inside; `group_ids(orders)` gives the groups which hold any of the
  conditions of a fragment. `TextPattern::captures` (set for `=~` with capture groups) and `sub_groups(hay, start)`.
- `query::Query`: `sub_groups` of the `text =~` conditions and `group(reference)`, which checks a reference.
- `search`: `file_matches`, `line_fragments` and `PathFragments::of` add the sub-groups (`SubMatch`) of every regex
  match and then the ids of the groups around the conditions (`with_group_ids`); `merge_identical` joins the groups
  of merged fragments.
- `output::json`: `Group` of a fragment, its columns by the line where it starts; `engine::without_ids` clears the
  groups for `omit groups` and their ids for `omit match_ids`.
- `relevance`: a group counts as its inside, so a label doesn't change the ranking.

## Deviations from the plan

- the task sketch showed `⟦#1,@ser:serde⟧`; the spec orders the ids by appearance, so the group id comes first
  (`⟦@ser,#1:serde⟧`), as the id of `line( ... )` does
- `Query::group`, `GroupRef` and `SubGroup` are not reachable from a request yet: they carry a local
  `#[allow(dead_code)]` naming tasks 129 and 130, which use them; the error `unknown_group` is unit tested only
- `TextPattern` got a fifth flag: `clippy::struct_excessive_bools` is allowed locally

## Tests

- examples E128.1–E128.11 (`tasks/128__EXAMPLES__QUERY_GROUPS.md`, fixture `query-groups`): a group around `or`
  branches, the `condition` facet, nested groups, a group inside `line( ... )`, a group of metadata conditions, regex
  sub-groups numbered and named in JSON (a group which didn't take part), the same query in the text format, a
  numbered regex inside `line( ... )`, `omit groups`, duplicate labels between a group and a condition, a second label
- E117.1, E131.1 (the description), E117 (the schema) and E134 (the omit parts) updated for the spec change
- unit tests: `query::tests::a_labelled_bracket_is_a_group`, `a_duplicate_label_is_the_later_one`,
  `group_references`; `eval::tests::a_group_contributes_with_its_inside`, `sub_groups_of_a_regex`;
  `omit::tests::parts_by_name`

## Limitations

- groups of `name` / `path` conditions have no content text, and `name =~` / `path =~` have no sub-groups
- the references are used by `replace` ([task 129](129__TASK__REPLACE_TOOL.md)) and `output.groups`
  ([task 130](130__TASK__OUTPUT_GROUPS.md))

## How to try it

```sh
cargo build --release
```

```json
{ "query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'" }
{ "query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": { "format": "json" } }
```

## Version

`0.130.0`
