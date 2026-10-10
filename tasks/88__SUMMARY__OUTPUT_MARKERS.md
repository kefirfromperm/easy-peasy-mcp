# Summary of task 88: `output.markers`

Server version: `0.88.0`.

## What was built

- `output.markers` (CONFIG.md#output, OUTPUT.md#highlight), default `['⟦', '⟧']`: from the config file, shown by
  `get_config` with its source, changed and reset by `update_config`, replaced for one request by `output.markers`.
  The value is two non-empty strings: `two non-empty strings are expected` (config file, `update_config`),
  `'output.markers' must be two non-empty strings` (request).
- The markers wrap the fragments with `highlight = markers` (and `auto` in text), in the text and JSON formats and in
  the repeated lines.
- The warning `marker_in_content`: in `lines` detail with markers, the files with a matched line which contains the
  opening or the closing marker; ordered after `date_unsupported`.

## Architecture

- `src/config/keys.rs`: `Type::Markers`, `Key::Markers`.
- `src/output/mod.rs`: `Highlight::Markers(open, close)`, `Response::all_warnings` (the warnings of the search and the
  marker warnings, used by both formats); `src/output/lines.rs` writes the given markers.
- `src/warning.rs`: `WarningCode::MarkerInContent`; `src/engine.rs`, `src/description.rs`.

## Deviations from the plan

- The `get_config` examples of tasks 35–87 show the key and the spec got the errors and the rule of the warning
  (separate commits).

## Tests

- examples E88.1–E88.11 (fixture `marker-text`, fixture `highlight` of task 87)
- unit: `config::keys::tests::markers_are_two_strings`, `output::tests::a_marker_in_a_matched_line_is_a_warning`,
  `warning::tests::markers_in_content_after_dates`, `request::tests::output`

## Limitations and known gaps

None.

## Try it

`{ "query": "text has 'TODO'", "output": { "markers": ["«", "»"] } }`
