# Plan for task 138: all the locations of a repeated line

## Design

The spec change is committed already ([OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines)):
`output.repeated_locations` (default `3`, `0` all), the locations counted in `max_output` on their own, the omit part
`locations`.

- `config/keys.rs`: `Key::RepeatedLocations` after `group_repeated`, `Type::Whole(Some("0: all"))`, default `3`;
  `config/mod.rs` names it in `[output]`
- `output/omit.rs`: `Part::Locations` after `Indent`
- `output/mod.rs`: `Response::repeated_locations` (`usize::MAX` for `0`); `Response::written_locations` — per repeated
  line how many locations are written: at most `repeated_locations`, and while their characters and the `, ` between
  them stay within `max_output`; whether `max_output` cut them. `locations` writes those and `… M more`;
  `truncated_by` adds `max_output` when the locations were cut and the listing didn't name it
- `output/json.rs`: `locations` are the written ones, left out with `locations` (and when none is written)

## Rejected

- paging the locations with the cursor: the cursor is a position in the files; a line's locations are in many files
  before and after it. The spec points to `line(text = ...)` with `detail = files` for the rest
- counting the locations in the `max_output` of the files: a cut there would leave no files and a cursor which starts
  where it is, a page which never moves

## Tests

- E138.1–E138.5
- unit tests: `written_locations` with a number, all, and cut by `max_output`; `Part::Locations`
- earlier examples which change: `get_config` of `[output]` (`repeated_locations = 3`), the schema, the list of the
  omit parts

## Steps

1. red: the unit tests
2. key, part, response
3. schema
4. fix the earlier examples
5. summary, status `implemented`, minor version bump `0.133.0` → `0.134.0`
