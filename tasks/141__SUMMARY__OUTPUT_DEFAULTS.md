# Summary of task 141: useful output by default, the default of `output.omit`

Server version: `0.135.0`. Spec version: `2` (current), `1` supported.

## What was built

The review of every part of the output is in [141__PLAN__OUTPUT_DEFAULTS.md](141__PLAN__OUTPUT_DEFAULTS.md). Its
decisions are the defaults of spec version 2 ([OUTPUT.md#defaults-of-spec-2](../OUTPUT.md#defaults-of-spec-2)):

- `omit = ["warnings"]` by default. The warnings are replaced by one line which names what was left out, without
  paths: `warnings left out: binary_skipped (28), decode_errors (2)` (JSON `left_out`, the new omit part `left_out`).
  A timeout stays visible: the header has `truncated: yes (timeout)`, and the line names `timeout`
- rules of parts which carry nothing: the legend, the conditions of the files and the ids of the matches only with
  more than one condition; the gaps `--` only with context lines (or without line numbers)
- `output.show` (a request option and a configuration key, a list of parts or `"all"`) brings parts back over `omit`
  and the rules; `show = "all"` is the output of spec 1
- the `omit` of a request, and of a batch item, is added to the configured one in spec 2, so asking for one more part
  doesn't bring the warnings back
- `"version": 1` keeps the defaults of spec 1; the JSON responses carry the version of the request; `get_config`
  writes `spec 2, supported specs: 1, 2`; the descriptions say `Spec version 2` and describe the defaults and `show`

A plain `find` on this repository, the searches of the review, spec 1 against spec 2:

| Search | Spec 1 | Spec 2 |
|--------|--------|--------|
| one condition, a hit, `lines` | 104 tokens | 41 |
| one condition, no hit | 93 | 39 |
| two conditions, a hit, `lines` | 75 | 75 |
| one condition, `files` | 924 | 779 |
| one condition, a hit, JSON | 162 | 80 |

### Spec version

The new defaults change the text layout, so they came with spec version 2 (TOOLS.md#versioning, which now has a table
of the versions). The earlier examples are spec 1 examples: a line `` Spec: `1` `` in every examples file before 141
runs them against a test server whose current spec version is 1 (`EngineOptions::spec`, for the binary
`EASY_PEASY_TEST_SPEC`; AGENTS.md). So their requests, responses and cursors stay; what changed in them is listed
below.

## Architecture

- `lib.rs`: `SPEC_VERSION = 2`; `request.rs`: `SPEC_VERSIONS = [1, 2]`, `Request::version`
- `config/keys.rs`: `Key::Show` after `omit`; `Key::default_for(spec)` — `omit` is `[]` in spec 1
- `engine.rs`: `EngineOptions::spec`; `Engine::new` takes the built-in defaults of the current spec;
  `request_settings` applies the defaults of the spec of a request and adds its `omit` to the configured one;
  `respond` computes the effective omit (`Omit::effective`: the rules, `show`) and passes the spec to the response;
  the write tools write their JSON version
- `output/omit.rs`: `Part::LeftOut`, `Omit::effective`, `union`, `all`; `output/mod.rs`: `left_out()` and the line;
  `output/json.rs`: `left_out`, the version of the request
- `batch.rs`: the item `omit` added to the shared one in spec 2; the batch JSON with the version of the request
- `config/mod.rs`: `get_config` with the current and the supported versions
- tests: the `Spec:` line in `tests/common`, the hook in `tests/examples.rs`, `tests/startup.rs` and `main.rs`

## Deviations from the plan

- the command line has no option for `[output]` keys, so the defaults are changed by the config file and
  `update_config` in the examples, not on the command line
- the JSON number of a warning which is not about entries in `left_out` is `0`, added to the spec while coding
- the spec examples of OUTPUT.md stay spec 1 examples (E121 reproduces them), with a note at its top; the Omit section
  has a spec 2 example

## Tests

- E141.1–E141.14 (fixtures `omit`, `timeout`, the new `gaps`)
- unit tests: `Omit::effective`, `Key::Show` and `default_for`, the version of a request
- earlier examples changed: `` Spec: `1` `` in every examples file before 141; `supported specs: 1, 2` and
  `show = []` of `get_config`; the descriptions and the schema (E117, E121.8, E131.1); E110.3 asks for the version
  3, since 2 exists; the end-to-end tests of `tests/mcp_stdio.rs` expect spec 2, `tests/search.rs` runs as spec 1

## Limitations

- the responses of the write tools have no `omit` (out of scope)
- task 139 (the dry run header) is the next change of spec 2

## How to try it

```json
{ "query": "text has 'TODO'" }
{ "query": "text has 'TODO'", "output": { "show": ["warnings"] } }
{ "version": 1, "query": "text has 'TODO'" }
```
