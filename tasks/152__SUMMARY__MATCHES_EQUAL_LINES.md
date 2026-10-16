# Summary of task 152: The number of matches left out when it is the number of lines

Server version: `0.151.0`.

## What was built

- A new `omit` part `match_counts` ([OUTPUT.md#omit](../OUTPUT.md#omit)): `matches: N` of the header and of a file
  and, in the text format, the `: N` of `count_by = line`. JSON has no column: `omit` of the part doesn't touch
  JSON, which keeps `matches` everywhere.
- The spec 2 rule ([OUTPUT.md#one-match-per-line](../OUTPUT.md#one-match-per-line), listed in
  [OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)): the text format leaves `match_counts` out when
  every matched line of the whole result has exactly one match. So:
  - `text has 'unwrap('` answers `files: 2  lines: 3  result: r1` and `lib.rs  lines: 2`; `count_by = line` is the
    list of the matched line numbers (`  2`)
  - a `matches: N` anywhere in a response means that a line of the result has two matches or `0` (a multiline match
    continuing into it); then every count is written, also of the files with one match per line, and also when
    `0` and `2` add up like `1` and `1`
  - a result without matched lines (names only) answers `files: 4  lines: 0`
  - the decision is made over all files of the result, so the pages of a cursor, `max_files` and the budget don't
    change it
- `show = ["match_counts"]`, `show = "all"` and `"version": 1` bring the numbers back; `omit = ["match_counts"]`
  leaves them out whatever the matches. The `: 1` of `count_by = line` belongs to both `line_counts` and
  `match_counts` in the text format, so `show = ["line_counts"]` alone doesn't bring it back (in the spec).
- The tool description has one more line under OUTPUT: `No "matches: N" in a text response: every matched line has
  one match.`; the `omit` / `show` schemas, the error lists of the parts (OUTPUT.md, CONFIG.md) name the part.
- Spec: one commit on the branch (the part, the rule, the subsection, the description, the spec 2 examples of
  OUTPUT.md which have one match per line). A change of the spec 2 defaults, allowed since spec 2 has not been used
  outside this repository (TOOLS.md#versioning); spec 1 is unchanged.

## Architecture

- `output::omit::Part::MatchCounts` after `FileCounts`; `Part::ALL` has 19 parts, so the errors, the schemas and
  the parsing follow.
- `output::omit::Differ` has a third field `match_counts`: `Differ::of(&[Found])` sets it when a line of
  `lines::line_counts` of a file has a count other than 1; `Differ::and(many)` (the counted-conditions rule) keeps it.
- `Omit::effective` adds `MatchCounts` in spec 2 when `!differ.match_counts`; `show` takes it back as every part.
- `output/mod.rs`: the header and the file line write `  matches: N` only without `MatchCounts`;
  `push_line_counts` writes `: N` only when neither `LineCounts` nor `MatchCounts` is left out.
- `engine.rs` is unchanged: `Differ::of(&files)` already runs over the whole result before the limits.

## Deviations from the plan

- E145.5 (a file list cut by the budget) would have changed its meaning: without the numbers the 14 files fit the
  budget of 115 in `lines` detail, and the smallest budget is 100. The example brings the numbers back with
  `show = ["match_counts"]` and keeps showing a cut list.
- E155.6: the cursor of the second query of a batch changed, since its share of `max_output` (part of its request)
  grows when the first query is shorter. E155.7: the bytes of the files.
- `tests/mcp_stdio.rs` checked `files: 1  lines: 0  matches: 0` of a name search; now `files: 1  lines: 0`.

## Tests

- examples E152.1–E152.8 (fixtures `omit` of task 134 and the new `match-counts`): one match in every line in
  `lines`, `files`, `count`, `count_by = file` and `line`; no matched lines; a line with two matches; `0` and `2`
  adding up like `1` and `1`; a cursor whose first page alone has one match per line; `show` of the part and of
  `line_counts`, `show = "all"`, `"version": 1`; `omit` of the part with two matches in a line; JSON
- unit `output::omit::tests::one_match_per_line`, `the_rule_of_the_match_counts`; `output::tests::the_counts_of_the_lines`
  with `MatchCounts`; the earlier `omit` tests adapted to the new field
- the earlier examples fixed in their own commit: the lists of the parts (E134, E136–E138, E141, E150, E117.2,
  E131.1) and the spec 2 responses with one match per line (tasks 130, 135, 141–149, 151, 155); each changed line
  was checked by a script to differ only by the rule

## Limitations and known gaps

- the counts of the directory headers and of facets, and the `files:` / `lines:` of the header, are out of scope
- the decision needs the line counts of every matched file: one more linear pass over the fragments of the result

## Try it

`{"query": "line(text has 'group_dirs')", "dirs": ["src"], "output": {"detail": "files"}}` — the files with
`lines: N` only, unless a line has `group_dirs` twice.
