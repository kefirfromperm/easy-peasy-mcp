# Plan for task 141: useful output by default, the default of `output.omit`

## Review

Every part of [OUTPUT.md#omit](../OUTPUT.md#omit). The costs are from five plain `find` calls on this repository
(about 140 example files and 70 fixtures, server 0.134.1) and from the searches of `EASY_PEASY_STATS.local.md`, where
the agent omitted `legend`, `conditions` and `result` by hand in almost every search and paid for `funnel` and the
warnings when it forgot.

| Part | What the agent does with it, in which search | Cost | How it gets it back, how bad to miss | Decision |
|------|----------------------------------------------|------|--------------------------------------|----------|
| `counts` | the size of the result, every search; the only answer of `count` | ~10 tokens | — | shown |
| `result` | `within`, `since`, the write tools; any search it refines or acts on | ~3 tokens | — | shown |
| `dir` | the paths of `path_style = in_dir` | one line | — | shown |
| `legend` | decodes the ids of `[#1 #2]` and the markers; with one condition it repeats the request | ~6 tokens a condition | `show = ["legend"]`; nothing lost with one condition, the agent wrote it | shown with more than one condition |
| `funnel` | fixes an empty or huge result; `funnel = auto` already writes it only then | 10–30 tokens, when shown | — | shown (`auto`) |
| `hints` | as the funnel | 10–40 tokens, when shown | — | shown |
| `conditions` | which branch of `or` matched; with one condition `[#1]` on every file | ~2 tokens a file: 157 of 894 tokens of a list of 100 files | `show = ["conditions"]`; nothing lost with one condition | shown with more than one condition |
| `file_counts` | how many matches in a file, `files` detail | ~6 tokens a file | — | shown |
| `match_ids` | which condition a match is of; with one condition `#1:` in every marker | ~2 tokens a match | `show = ["match_ids"]`; nothing lost with one condition | shown with more than one condition |
| `groups` | JSON only, the sub-groups of `=~`, for `replace` templates | only with capture groups | — | shown |
| `line_numbers` | to edit, to judge where the matches are | ~2 tokens a line | — | shown |
| `line_counts` | `count_by = line` | — | — | shown |
| `gaps` | `--` between blocks; without context lines the line numbers already show the gap | ~1 token a block | `show = ["gaps"]`; nothing lost while the line numbers are shown | shown with context lines, or without line numbers |
| `indent` | `⇥N` rebuilds the exact indentation an edit needs | ~2 tokens a block | — | shown |
| `locations` | the places of a repeated line, to edit them | ~3 tokens a location | — | shown |
| `warnings` | entries which were not searched: binary, undecodable, too big, unreadable; rarely acted on, but repeated on every search: 28 `binary_skipped` and 2 `decode_errors` of test fixtures on a search with no match, 69 of its 93 tokens | ~25 tokens a code | `show = ["warnings"]`; missing a `timeout` must never look complete: the header keeps `truncated: yes (timeout)`, and the mark names every code left out | omitted by default, with a mark |

The mark of the omitted warnings is a new part, `left_out`: one line `warnings left out: binary_skipped (28),
decode_errors (2)`, the codes with their counts, without paths (~12 tokens instead of ~70). It is the only mark: the
parts omitted by the rules lose no information.

The options which decide what a part looks like keep their defaults: `highlight = auto` (markers, plain lines where
they outgrow the line), `dedent = true` (with `⇥N`), `funnel = auto`, `group_repeated = 3` with
`repeated_locations = 3`, `max_line_length = 300`.

Measured with the new defaults simulated by `omit` (without the mark):

| Search | Old | New |
|--------|-----|-----|
| one condition, a hit, `lines` | 104 tokens | 26 |
| one condition, no hit | 93 | 24 |
| two conditions, a hit, `lines` | 75 | 75 |
| one condition, `files`, 100 files | 894 | 737 |
| one condition, a hit, JSON | 162 | 64 |

## Design

### Spec version

The new defaults change the text layout, a breaking change (TOOLS.md#versioning): they come with spec version 2,
the current one; `"version": 1` keeps the old defaults (`supported specs: 1, 2`). Arguing that a default of `omit` is
not a layout change would make every earlier example wrong at once, and an agent which parses `warning:` lines would
lose them silently.

- the JSON responses have `"version": 2`, or `1` for a request with `"version": 1`
- the descriptions and the server instructions say `Spec version 2`
- the earlier examples are spec 1 examples: an examples file states its spec version with a line `` Spec: `1` ``, and
  the test server runs with that current spec version (a test hook like `now`: `EngineOptions::spec`, for the startup
  examples `EASY_PEASY_TEST_SPEC`). So their requests and their cursors stay as they are; only `supported specs: 1, 2`
  of `get_config` and the descriptions change. AGENTS.md describes the line

### Getting a part back

- `output.show`: a list of parts or `"all"`, a request option and a configuration key (default `[]`): the parts it
  names are shown whatever `omit` and the rules say; `"all"` is the full output of spec 1
- spec 2: the `omit` of a request (and of a `queries` item) is added to the configured one instead of replacing it,
  so `omit = ["legend"]` keeps the warnings omitted; `show` takes parts back. Spec 1 keeps replacing
- rejected: `omit` entries like `"+funnel"` / `"-funnel"` (a second syntax inside one list, and `"-x"` in a config
  file means nothing); a request `omit` which replaces (the problem of the task)

### Defaults

- the built-in default of `[output] omit` is `["warnings"]` in spec 2, `[]` in spec 1; `get_config` shows the one of
  the current spec; the config file, the command line and `update_config` change it as today
- the rules of spec 2 (OUTPUT.md#omit): `legend`, `conditions`, `match_ids` only when the legend has more than one
  condition; `gaps` only with context lines or without line numbers; `show` overrides them
- `left_out`: written when the warnings are left out (by default or by `omit`) and there are some; not in spec 1

### Code

- `lib.rs`: `SPEC_VERSION = 2`; `request.rs`: `SPEC_VERSIONS = [1, 2]`, `Request::version`
- `config/keys.rs`: `Key::Show` after `omit`; the default of `omit` per spec (`Key::default_for(spec)`)
- `engine.rs`: `EngineOptions::spec` (the current spec version); the settings of a request through one function which
  applies the spec default of `omit` and the merge of spec 2; the effective `Omit` of a response with the rules and
  `show`; the spec of the JSON `version`
- `output/omit.rs`: `Part::LeftOut`; `output/mod.rs`, `json.rs`: the mark
- `config/mod.rs`: the `get_config` header with the current and the supported versions
- `description.rs`: `Spec version 2`, the line about the defaults and `show`

## Tests

- E141.*: the same searches with spec 1 and spec 2 (one condition and several, a hit and no hit, `files` and `lines`,
  text and JSON); a part back with `show` keeping the other defaults; the defaults from the config file, the command
  line and `update_config` with `get_config`; the mark and the mark omitted; a `timeout`; `queries` items; wrong values
- unit tests: the effective omit (rules, `show`, the merge), `Key::Show`, `Part::LeftOut`, the version check
- the earlier examples: `` Spec: `1` `` in every examples file before 141 (the files of later tasks are spec 2),
  `supported specs: 1, 2`, the descriptions

## Steps

1. this plan, status `in progress`
2. spec: OUTPUT.md, CONFIG.md, TOOLS.md (versioning, schema, description), AGENTS.md (the `Spec:` line)
3. examples
4. red tests
5. the spec version, `show`, the defaults, the rules, the mark
6. fix the earlier examples
7. summary with the measurement, status `implemented`, minor version bump `0.134.1` → `0.135.0`

## Risks

- the spec 1 examples run against a test server whose current spec is 1: the hook must not change anything else
- a request with `"version": 1` on a spec 2 server gets the defaults of spec 1, also for a configuration key left at
  its default; a configured `omit` applies to both
