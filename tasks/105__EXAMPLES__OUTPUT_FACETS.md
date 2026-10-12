# Examples for task 105

Examples with a command line are startup examples (AGENTS.md). A JSON response is compared as a JSON value. The
examples use the fixture `huge` of task 104: eleven files with `error` — six in `vendor/lib/`, two in `src/`, two
tests in `tests/` (`.rs`), `docs/d1.md`; and the fixture `gitrepo` of task 17.

## E105.1 Where the matches are

Fixture: `huge`

1. `find`

```json request
{"query": "text has 'error'", "output": {"detail": "facets"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
dir:        vendor/lib/ 6 · src/ 2 · tests/ 2 · docs/ 1
lang:       rust 10 · markdown 1
kind:       source 8 · test 2 · doc 1
condition:  #1 11
```

2. `get_config`

```json request get_config
{ "section": "output" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[output]
format = "text"
detail = "lines"
# budget =  (not set)
chars_per_token = 4
funnel = "auto"
facets = ['dir', 'lang', 'kind', 'condition']
facet_depth = 2
facet_limit = 8
meta = false
context = 0
# before =  (not set)
# after =  (not set)
path_fragments = false
highlight = "auto"
markers = ['⟦', '⟧']
max_files = 100
max_lines = 500
max_matches = 1000
max_matches_per_file = 50
max_line_length = 300
max_output_per_file = 4000
max_output = 40000
dedent = true
blank_context = false
group_dirs = true
group_repeated = 3
sort = "relevance"
path_style = "relative"
```

## E105.2 Other facets

Fixture: `huge`

```json request
{"query": "text has 'error' or name ~ '*.md' @md", "output": {"detail": "facets", "facets": ["ext", "condition"]}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
@md name ~ '*.md'
ext:        rs 10 · md 1
condition:  #1 11 · @md 1
```

## E105.3 The git facet

The status is read without a `git` condition; `outside/loose.txt` is in no repository.

Fixture: `gitrepo`

```json request
{"query": "name ~ '*.txt'", "output": {"detail": "facets", "facets": ["git"]}}
```

```text response
files: 7  lines: 0  matches: 0  result: r1
#1 name ~ '*.txt'
git:        modified 2 · staged 2 · clean 1 · conflicted 1 · untracked 1
```

## E105.4 In JSON

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"detail": "facets", "facets": ["kind", "lang"], "format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "facets": {
    "kind": { "source": 8, "test": 2, "doc": 1 },
    "lang": { "rust": 10, "markdown": 1 }
  },
  "counts": { "files": 11, "lines": 11, "matches": 11 },
  "legend": { "#1": "text has 'error'" }
}
```

## E105.5 For the session

Fixture: `huge`

1. `update_config`

```json request update_config
{"set": "output.facets = ['lang']"}
```

```text response
[output]
facets = ['lang']    # was ['dir', 'lang', 'kind', 'condition']
```

2. `find`

```json request
{"query": "text has 'error'", "output": {"detail": "facets"}}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
lang:       rust 10 · markdown 1
```

3. `get_config`

```json request get_config
{ "section": "output", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.output_dir, server.max_output_file_size

[output]
format = "text"    # default
detail = "lines"    # default
# budget =  (not set)    # default
chars_per_token = 4    # default
funnel = "auto"    # default
facets = ['lang']    # update
facet_depth = 2    # default
facet_limit = 8    # default
meta = false    # default
context = 0    # default
# before =  (not set)    # default
# after =  (not set)    # default
path_fragments = false    # default
highlight = "auto"    # default
markers = ['⟦', '⟧']    # default
max_files = 100    # default
max_lines = 500    # default
max_matches = 1000    # default
max_matches_per_file = 50    # default
max_line_length = 300    # default
max_output_per_file = 4000    # default
max_output = 40000    # default
dedent = true    # default
blank_context = false    # default
group_dirs = true    # default
group_repeated = 3    # default
sort = "relevance"    # default
path_style = "relative"    # default
```

## E105.6 An invalid value in a request

Fixture: `huge`

```json request
{"query": "text has 'error'", "output": {"facets": ["dir", "size"]}}
```

```text error
error[invalid_option]: 'output.facets' must be a list of facets (dir, ext, lang, kind, git, condition)
```

## E105.7 An invalid value for `update_config`

Fixture: `huge`

```json request update_config
{"set": "output.facets = 'dir'"}
```

```text error
error[invalid_value]: invalid value for 'output.facets'
  hint: a list of facets (dir, ext, lang, kind, git, condition) is expected
```

## E105.8 From the config file

Fixture: `huge`

```toml conf/o.toml
[output]
detail = "facets"
facets = ["kind"]
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```json request
{"query": "text has 'error'"}
```

```text response
files: 11  lines: 11  matches: 11  result: r1
#1 text has 'error'
kind:       source 8 · test 2 · doc 1
```

## E105.9 An invalid value in the config file

Fixture: `huge`

```toml conf/o.toml
[output]
facets = ["dirs"]
```

```text command
easy-peasy-mcp --config conf/o.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/o.toml:2: invalid value for 'output.facets': a list of facets (dir, ext, lang, kind, git, condition) is expected
```
