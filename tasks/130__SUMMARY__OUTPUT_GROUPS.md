# Summary of task 130: Output limited to groups

An agent can see only the matches it cares about: with `output.groups = ["@ser"]` the query
`(text has 'serde' or text has 'serde_json') @ser and text has 'apple'` still needs `apple` to choose the files, but
only the `serde` matches are shown and counted. `output.only_match` writes the matches instead of their lines, like
`grep -o`, so `["@def.fname"]` of `` text =~ `fn\s+(?<fname>\w+)\(` @def `` lists the names of the functions.

## What was built

- **`output.groups`** ([OUTPUT.md#shown-groups](../OUTPUT.md#shown-groups)): a list of
  [group references](../QUERY.md#group-references). The shown matches of a file are its fragments of the whole
  groups of the list, with only those ids, and the sub-groups of the list as matches of their own, with their
  reference (`@def.fname`) as their id. The other conditions only select the files.
- **Counts and limits**: `lines` and `matches` of the header and of every file count the shown matches, in every
  detail; `files` stays the number of matched files, and a file without a shown match is listed without counts and
  lines. `max_lines`, `max_matches`, `max_matches_per_file`, the budget, the cursor (its totals too), the repeated
  lines and the result id work on the shown matches; the order of the files is the one without `groups`.
- **The ids of the matches** ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)): with `groups` the
  spec 2 rule writes them when the list has more than one reference, also when the legend has one condition.
- **`output.only_match`** ([OUTPUT.md#only-the-matches](../OUTPUT.md#only-the-matches)): every match on its own
  line(s) with its line number, in the order of the start, no context, gaps, `⇥N` or cut of long lines, no repeated
  lines; `markers` wraps a match with its ids, `auto` too unless the markers outgrow it or it has no ids, `columns`
  adds its columns. In JSON a file has `fragments` (`{ n, col, text, ids, ... }`, or `{ n, text }` with `none` and
  `markers`) instead of `blocks`. `max_output_per_file` and `max_output` count the characters of the matches.
- **Errors**: `'output.groups' must be a list of group references`; `unknown_group` with the position in the
  reference, in JSON the field `output.groups`; `'output.only_match' must be true or false`; in the config file and
  `update_config` `a list of group references is expected`.
- **Configuration** ([CONFIG.md#output](../CONFIG.md#output)): both keys after `path_fragments`, in `get_config`,
  `update_config` and the config file; a configured list applies to every `find` query, the list of a request
  replaces it.
- **The `find` description and schema** ([TOOLS.md#find](../TOOLS.md#find)) name the two options.

Spec commits: the options; the rule of the ids with references; the JSON field of the error; the order of the
matches at the same start.

## Architecture

- `shown::Shown` (new module): `of(query, list)` checks the references with `Query::group` in their order;
  `apply(files)` replaces the fragments (`merge_identical`, `matched_lines` again) and the path fragments of every
  file, a file without shown matches gets `matches: None`.
- `query::ast`: `CondId::Sub(String)`, the id of a match of a sub-group, so the markers, the columns, JSON and
  `omit match_ids` handle it unchanged; `GroupRef` displays as its canonical reference (`#1.01` → `#1.1`).
- `engine::run` takes `groups`: `find` (also every query of a batch) checks the references right after parsing the
  query and applies them after the search, before the check of the totals of a cursor; the write tools pass `false`
  and keep all the matches. `Run::references` goes to `Omit::effective`, which got the argument `references`.
- `only_match`: `BlockOptions::only_match` (the blocks without context and dedent, `group_repeated = 0`);
  `lines::render_matches` writes the text format, `json::matches` the JSON one; `listing::shown_lines` counts the
  characters of the matches.
- `config::keys`: `Key::Groups` (`Type::GroupRefs`) and `Key::OnlyMatch` (`Type::Bool`).

## Deviations from the plan

- the references are checked in `run`, right after the query is parsed, rather than in `try_find`: the query is
  parsed there and an unknown group still costs no search; a flag keeps the write tools out
- the examples E130.3–E130.5 first left out `fn main` of `src/a.rs`; they were fixed in their own commit

## Tests

- examples E130.1–E130.13 (`tasks/130__EXAMPLES__OUTPUT_GROUPS.md`, fixture `query-groups` of task 128): the goal,
  two references, a named sub-group with `only_match`, a group and its sub-group with markers, JSON, a group inside
  `line( ... )` and `near( ... )`, a file without a shown match in `files` and `count` detail, `max_matches` with the
  cursor over two pages, the budget, an unknown group in text and JSON, invalid values, the configuration with
  `get_config`, `update_config`, a configured list and its replacement
- earlier examples: the `[output]` section of `get_config` in 39 files, the description in E117.2 and E131.1, the
  schema in E117.2, E121.103 with the full example of CONFIG.md
- unit tests: `shown::tests` (an empty list, the references in order with the JSON field, whole groups, sub-groups,
  a numbered sub-group, a file without a shown match, path fragments), `omit::tests::the_ids_of_the_matches_follow_the_references_of_groups`,
  `lines::tests::only_the_matches`, `only_the_matches_with_auto`, `listing::tests::only_the_characters_of_the_matches`,
  the `GroupRefs` type and the defaults in `config::keys::tests`

## Limitations

- a configured `output.groups` applies to every query: a query without the group fails with `unknown_group` until
  the request gives its own list
- `only_match` doesn't group repeated matches (no `uniq -c` of the matched text)

## How to try it

```sh
cargo build --release
```

```json
{ "query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": { "groups": ["@ser"] } }
{ "query": "text =~ `fn\\s+(?<fname>\\w+)\\(` @def", "output": { "groups": ["@def.fname"], "only_match": true } }
```

## Version

`0.137.0`
