# Plan for task 63: `kinds.generated_markers`

## Design

- `src/config/keys.rs`: `Key::GeneratedMarkers` in the section `kinds`, `Type::List("strings")`, default the four
  markers of the spec; the request object `kinds` holds the keys of the section.
- `src/search/kinds.rs`: `KindOptions` — the markers, the marker lines and the four wildcard lists, the built-in
  ones by default; `KindRules::new(&KindOptions, case_insensitive)` instead of `builtin`. The next tasks make the
  other fields configurable.
- `src/search/mod.rs`: `SearchOptions.kinds: KindOptions`; the walker builds the rules from it.
- `src/engine.rs`: the markers from the settings.
- `src/description.rs`: the `kinds` object in the input schema.

## Tests

- examples E63.1–E63.9 (fixture `markers`: two files with different markers, one without)
- unit: the rules with other markers, without markers; the request field

## Steps

1. failing tests
2. key, kind options, walker

## Risks

- none
