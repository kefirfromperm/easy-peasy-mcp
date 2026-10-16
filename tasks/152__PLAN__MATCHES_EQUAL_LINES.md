# Plan for task 152: The number of matches left out when it is the number of lines

## Design

- `output/omit.rs`: a new `Part::MatchCounts` (`match_counts`) after `FileCounts`, so the list of the parts in the
  errors of `omit` / `show`, the schemas and `get_config` follow from `Part::ALL`. `Differ` gets a third field,
  `match_counts`: whether a matched line of the result has another number of matches than one;
  `Differ::of(&[Found])` computes it from `lines::line_counts` of every file with content matches (`0` or `2` in a
  line), `Differ::and(many)` keeps it (it is not about the conditions). `Omit::effective` adds `MatchCounts` in spec 2
  when `!differ.match_counts`; `show` takes it back as every part.
- `output/mod.rs` (text format): the header writes `  matches: N` and a file `  matches: N` only without
  `MatchCounts`; `push_line_counts` writes `: N` only when neither `LineCounts` nor `MatchCounts` is left out.
- `output/json.rs`: unchanged — it doesn't read `MatchCounts`, so `matches` stays in every JSON response.
- `engine.rs`: unchanged — `Differ::of(&files)` is already computed over the whole result before the cursor, the
  limits and the budget.
- `description.rs`: the line of the tool description and the `omit` / `show` enums of the schema (TOOLS.md).
- `config/keys.rs`: the expected parts lists in its tests.

## Decisions

- a new part rather than a rule of `file_counts` / `counts`: the rule removes a number from inside two parts, so it
  needs its own bit; `omit = ["match_counts"]` is useful on its own (the agent who reads only `lines:`)
- the rule is over the whole result and per line, not "`matches` = `lines` per file": a file of `0` and `2` adds up
  like `1` and `1`, and a left-out number must always mean one match per line, on every page of a cursor
- JSON keeps `matches`: fields are read by name, and the tokens are in the text format
- the `: 1` of `count_by = line` belongs to both `line_counts` and `match_counts` in the text format; `show =
  ["line_counts"]` alone doesn't bring it back (written in the spec)

## Tests

- examples E152.1–E152.8 (fixtures `omit`, `match-counts`)
- unit `output::omit::tests`: `Differ::of` with one match per line, with two matches in a line, with `0` and `2`;
  `and(false)` keeps `match_counts`; `effective` leaves out `MatchCounts` in spec 2 only, and not with `show`
- the earlier examples which change: the spec 2 examples files (tasks 130, 135, 139, 141–151, 155) where every line
  of the result has one match lose `matches: N`; the lists of the parts in the errors of examples 134, 136, 137, 138,
  141, 150 get `match_counts`; fixed in their own commit with the reason

## Steps

1. tests (red): the unit tests, the examples of task 152
2. `Part::MatchCounts`, `Differ::match_counts`, the rule, the text format, the description
3. fix the earlier spec 2 examples and the lists of the parts
4. summary, version bump `0.149.0` → `0.150.0`

## Risks

- `line_counts` of every matched file costs a pass over the fragments of the whole result: linear, as `Differ::of`
  already is
- many earlier spec 2 examples change; they are checked one by one against the rule (every changed response must
  have one match in every line of its result)
