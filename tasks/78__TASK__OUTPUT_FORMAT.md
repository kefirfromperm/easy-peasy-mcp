# Task 78: `output.format`

Status: not implemented

Depends on: task 36

## Goal

Get the response as JSON: `"output": { "format": "json" }`.

## Scope

- the key `format` in the `[output]` section of the config file, default `"text"`
- shown by `get_config` (with its source), changed and reset by `update_config`
- the request field `output.format` replaces it for one request
- the request object `output` is added to the `find` input schema by this task, with this key only; the other keys of the spec are added by their tasks
- an invalid value: an error at start naming the key and the line (file), `invalid_value` (`update_config`), `invalid_option` (request)
- the JSON document of the spec in the text content item: `version`, `exists`, `result`, `counts`, `legend`, `files` with `path`, `lines`, `matches`, `conditions`, `blocks` of lines with `n`, `text`, `fragments` (`col`, `text`, `ids`, `escape`, `to`), `encoding`, `warnings`
- empty and default values are omitted
- JSON paths are always full (no directory groups)
- errors in JSON: `error` with `code`, `message`, `query`, `position`, `hint`
- `highlight = auto` is `columns` in JSON

## Spec

- [CONFIG.md#output](../CONFIG.md#output)
- [OUTPUT.md#options](../OUTPUT.md#options)
- [OUTPUT.md#json-format](../OUTPUT.md#json-format)
- [OUTPUT.md#errors](../OUTPUT.md#errors)
- [OUTPUT.md#fields](../OUTPUT.md#fields)

## Known gaps

The finished server answers differently here; the examples of this task must not cover these cases.

- default-on behavior of the tasks not implemented yet: relevance (93), `max_files` truncation (94), output limits (95–100), cursor (101), funnel and hints (102–104), dedent (84), repeated lines (86), long line cuts (98) — examples avoid these cases or set the output options which are already implemented (e.g. `sort = "path asc"` after task 92)

## Examples

`78__EXAMPLES__OUTPUT_FORMAT.md` covers:

- the default, a value from the config file, a value from the request, an invalid value

## Acceptance criteria

- every example in `78__EXAMPLES__OUTPUT_FORMAT.md` passes as a test; the examples of all earlier tasks still pass
- the `find` tool description (and `get_config` / `update_config` where the feature is configurable) describes the feature as far as it is implemented
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
