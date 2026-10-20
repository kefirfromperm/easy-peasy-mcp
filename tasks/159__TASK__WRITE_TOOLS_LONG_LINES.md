# Task 159: Long changed lines of `replace` cut around the change

Status: in progress

Depends on: task 129, task 155

## Goal

`replace` writes every changed line whole, twice (`-` and `+`), whatever its length. Found in run 4 of the experiment
(EXPERIMENT.md, task 152): arm A changed one sentence of the `find` description inside the one-line JSON of E117.2,
and the dry run and the real call each printed the whole line twice, ~10K characters for a change of ~20.
Reproduced on 0.150.0:

```
replace  query = "path = 'tasks/117__EXAMPLES__TOOL_DESCRIPTIONS.md' and text has 'leftovers in files with uncommitted changes\"' @x"
         replace = {"@x": "leftovers in changed files\""}, dry_run = true

dry run  would replace: 1  matches: 1

tasks/
117__EXAMPLES__TOOL_DESCRIPTIONS.md  matches: 1
-45:        "description": "Search files by name, path, ... (4.8K characters) ... leftovers in files with uncommitted changes",
+45:        "description": "Search files by name, path, ... (4.8K characters) ... leftovers in changed files",
```

`find` cuts a line longer than `max_line_length` (300) to windows around its matches (OUTPUT.md#long-lines); the
write tools have no such limit, and `output.max_lines` counts lines, not characters. With the windows of `find` the
response above is ~700 characters: ~9K characters, ~2.3K tokens saved per call, and the dry run is done before
every real call, so a change of a long line pays it twice. Long lines are common where agents edit text: minified
files, JSON on one line, Markdown paragraphs without line breaks, generated code.

## Scope

A spec change first (its own commit; TOOLS.md#response-of-a-write-tool, TOOLS.md#replace), then the code.

- the spec step decides:
  - the windows: around the matches in the `-` line and around the new text in the `+` line, with the rule of
    OUTPUT.md#long-lines and `…` at the cuts; or one window over the changed span of both lines
  - the limit: `output.max_line_length` of the configuration (300) and of the request, or a limit of its own
  - JSON: the lines cut the same way, or whole (its fields are read by code)
  - whether `rename`, `copy` and `delete` need it (they write paths, not lines)
- spec 1 is unchanged; a change of a spec 2 default is decided by [TOOLS.md#versioning](../TOOLS.md#versioning)

## Out of scope

- what is written to the files: only the response changes
- `find`'s long lines (OUTPUT.md#long-lines)

## Examples

`159__EXAMPLES__WRITE_TOOLS_LONG_LINES.md` covers:

- a dry run and a real call of `replace` on a line over the limit: one match, two matches far apart, a match near
  the start and near the end
- a line under the limit: unchanged
- `output.max_line_length` in the request, and `"version": 1`

## Acceptance criteria

- the spec states how a long changed line is written before the code is written
- every example in `159__EXAMPLES__WRITE_TOOLS_LONG_LINES.md` passes as a test; the examples of all earlier tasks
  still pass
- the server version gets a minor bump
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
