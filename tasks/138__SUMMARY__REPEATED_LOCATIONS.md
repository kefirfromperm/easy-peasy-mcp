# Summary of task 138: all the locations of a repeated line

Server version: `0.134.0`.

## What was built

[OUTPUT.md#repeated-lines](../OUTPUT.md#repeated-lines): `output.repeated_locations` is the number of locations
written for a repeated line, default `3` (the output as before), `0` all of them. An agent which has to edit a line
repeated in many files gets every place in one response, in one line, instead of writing the line under every file
with `group_repeated = 0`:

```
repeated:
  ×13 use ⟦#1:std::⟧io;    src/a.rs:1, src/b.rs:1, …, src/l.rs:1, src/main.rs:1
```

- `, … M more` only when some locations are left out; JSON `locations` has the written ones, `count` all the files
- the locations count in `max_output` on their own: the location which would go over and the next ones are left out,
  `truncated_by` names `max_output`; they are not paged, the cursor stays a position in the files
- the budget estimates the whole response with the locations, so all of them can lower the detail (E138.3)
- `omit = ["locations"]` leaves them out, the line and `×N` stay; `"all"` too
- a request option and a configuration key after `group_repeated`, `update_config`, `get_config`, the items of
  `queries`; the schema has its field

## Architecture

- `config/keys.rs`: `Key::RepeatedLocations`, `Type::Whole(Some("0: all"))`
- `output/omit.rs`: `Part::Locations` after `Indent`
- `output/mod.rs`: `Response::repeated_locations` (`usize::MAX` for `0`); `written_locations` decides how many
  locations of every line are written and whether `max_output` cut them; `locations(line, count)`; `truncated_by`
  adds `max_output` for a cut of the locations
- `output/json.rs`: the written locations, left out when there are none

## Deviations from the plan

None. E138.3 was fixed before the code was green: a budget is at least 100 tokens, so the example uses three
characters a token to tell three locations from all of them.

## Tests

- E138.1–E138.5 (fixture `repeated-locations`)
- unit test `the_locations_of_a_repeated_line` (a number, all, left out, cut by `max_output`); `Part::Locations`
- earlier examples changed: `repeated_locations = 3` in every `get_config` of `[output]`, the schema (E117.2), the
  list of the omit parts

## Limitations

- the locations are not paged: `line(text = '<the line>')` with `detail = files` lists all the files of a line when
  `max_output` cuts them

## How to try it

```json
{ "query": "line(text = 'allow_update = true')", "output": { "repeated_locations": 0 } }
```
