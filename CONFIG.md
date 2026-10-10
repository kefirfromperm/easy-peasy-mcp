# Configuration

The configuration holds the security boundaries of the server and the defaults of every `find` request. The query is
described in [QUERY.md](QUERY.md), the output in [OUTPUT.md](OUTPUT.md), the MCP tools in [TOOLS.md](TOOLS.md).

- [Where the values come from](#where-the-values-come-from)
- [Config file](#config-file)
- [Command line](#command-line)
- [Sections](#sections)
  - [server](#server)
  - [traversal](#traversal)
  - [Ignore files](#ignore-files)
  - [Encoding](#encoding)
  - [Archives](#archives)
  - [output](#output)
  - [escape](#escape)
  - [query](#query)
  - [kinds](#kinds)
  - [git](#git)
  - [Languages](#languages)
- [Platforms](#platforms)
- [get_config](#get_config)
- [update_config](#update_config)
- [Security](#security)
- [Examples](#examples)

## Where the values come from

From the lowest to the highest priority:

1. built-in defaults
2. config file
3. command line arguments
4. `update_config` calls — in memory, until the server stops
5. fields of the `find` request — for this request only

A request can replace any configuration value — `traversal`, `output`, `escape`, `query` (as the request field
`query_options`), `kinds`, `git`, `languages` — except the `server` section: the allowed roots, which bound the
working directories, can't be changed by a request; only `server.timeout` and `server.default_dirs` are replaced for
one request by its `timeout` and `dirs` fields. Values are replaced, not merged: a request `exclude` list replaces
the configured list. `languages` are replaced one language at a time: a request language replaces the configured
language with the same name, the others stay.

```json
{
  "dirs": ["."],
  "query": "text has 'deprecated' /i",
  "timeout": "5s",
  "traversal": { "exclude": ["**/target/**", "**/vendor/**"], "archives": "compressed" },
  "output": { "max_files": 10 }
}
```

Read-only values (`server.allowed_roots`, `server.allow_update`) can be set only by the config file and the command
line.

## Config file

TOML. The server looks for it in this order and takes the first one found:

1. `--config <path>`
2. `$EASY_PEASY_CONFIG`
3. the platform config directory:

| Platform | Path                                                                  |
|----------|-----------------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME/easy-peasy-mcp/config.toml`, usually `~/.config/easy-peasy-mcp/config.toml` |
| macOS    | `~/Library/Application Support/easy-peasy-mcp/config.toml`            |
| Windows  | `%APPDATA%\easy-peasy-mcp\config.toml`                                |

Without a file the built-in defaults are used. A file named by `--config` or by a non-empty `$EASY_PEASY_CONFIG`
must exist; the file in the platform directory is optional. An invalid file stops the server at start with an error
that names the key and the line.

Errors at start are one line on stderr, `easy-peasy-mcp: <path>:<line>: <message>` (the path as given or found, the
line 1-based), and the exit code `1`; the first error in the file is reported:

| Problem                                    | Message                                                            |
|--------------------------------------------|--------------------------------------------------------------------|
| not TOML                                   | `invalid TOML: <the parser's message>`                             |
| a section which doesn't exist              | `unknown section 'colors', expected one of: server, traversal, output, escape, query, kinds, git, languages` |
| a section written as a value               | `'output' must be a section`                                       |
| a key which doesn't exist                  | `unknown key 'output.colour'`, with `, did you mean 'output.color'?` for a close one |
| a wrong value                              | `invalid value for 'output.max_files': <what is expected>`, e.g. `a list of paths is expected` |
| a key this server version doesn't implement yet | `the key 'output.max_files' is not implemented yet`         |

A file which can't be read is `easy-peasy-mcp: <path>: the config file doesn't exist` or `... can't be read`, without a
line. An invalid command line prints the usage to stderr with the exit code `2`; an option this server version doesn't
implement yet prints `easy-peasy-mcp: the option '--root' is not implemented yet` with the exit code `2`.

There is no project level config file (e.g. in a repository): a cloned repository must not be able to change what the
server is allowed to search. Project specific exclusions are done with [ignore files](#ignore-files) like `.aiignore`.

### Full example

All keys with their default values; of the languages only one entry is shown:

```toml
[server]
allowed_roots = []              # empty: the directory the server is started in
default_dirs = ["."]            # working directories when the request has no `dirs`
timeout = "30s"
threads = 0                     # 0: number of CPU cores
allow_update = true             # whether `update_config` is available
result_cache = 50               # results kept for `within` / `since`

[traversal]
min_depth = 0
# max_depth = 10                # not set: unlimited
follow_symlinks = false
hidden = false
ignore_files = [".gitignore", ".ignore", ".aiignore"]
git_excludes = true             # .git/info/exclude and the global git excludes file
ignore_paths = []               # extra ignore files applied to every working directory
exclude = []                    # wildcards of paths which are never walked
path_case = "native"            # native | sensitive | insensitive
binary = "skip"                 # skip | search | exclude
max_file_size = "10M"
encoding = "auto"
# fallback_encoding = "windows-1251"   # not set: invalid bytes are replaced
archives = "none"               # none | compressed | all
max_archive_size = "100M"
skip_generated = true

[output]
format = "text"                 # text | json
detail = "lines"                # exists | count | facets | files | lines
# budget = 2000                 # not set: no token budget
chars_per_token = 4
funnel = "auto"                 # auto | always | never
facets = ["dir", "lang", "kind", "condition"]
facet_depth = 2
facet_limit = 8
meta = false
context = 0
# before = 0                    # not set: equal to `context`
# after = 0                     # not set: equal to `context`
path_fragments = false
highlight = "auto"              # auto | none | columns | markers
markers = ["⟦", "⟧"]
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
path_style = "relative"         # relative | absolute

[escape]
max_level = 3

[query]
fuzzy_min_score = 0.5

[kinds]
generated_markers = ["@generated", "DO NOT EDIT", "Code generated by", "auto-generated"]
generated_marker_lines = 5
generated = ["**/*.lock", "**/package-lock.json", "**/pnpm-lock.yaml", "**/*.min.js", "**/*.min.css", "**/*.pb.go", "**/*_pb2.py", "**/*.g.dart", "**/*.generated.*"]
test = ["**/test/**", "**/tests/**", "**/__tests__/**", "**/spec/**", "**/*_test.*", "**/test_*.py", "**/*.test.*", "**/*.spec.*", "**/*Test.java", "**/*Tests.cs"]
doc = ["**/*.md", "**/*.rst", "**/*.adoc", "**/README*", "**/LICENSE*", "**/CHANGELOG*", "**/docs/**"]
config = ["**/*.toml", "**/*.yaml", "**/*.yml", "**/*.ini", "**/*.cfg", "**/*.conf", "**/*.properties", "**/*.env", "**/package.json", "**/tsconfig*.json", "**/*.config.json", "**/*.config.js", "**/*.config.ts", "**/Dockerfile", "**/Makefile", "**/CMakeLists.txt", "**/requirements*.txt", "**/.github/**", "**/.gitlab-ci.yml"]

[git]
base = "auto"

# Languages: the built-in table is in "Languages" below, here one entry as an example.
# Built-in languages can be overridden, new ones added.
[languages.rust]
extensions = ["rs"]
line_comments = ["//"]
block_comments = [["/*", "*/"]]
nested_block_comments = true
strings = ['"']
source = true
```

## Command line

```
easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--log-level <level>]
               [--version]
```

| Argument         | Overrides                      | Example                                  |
|------------------|--------------------------------|------------------------------------------|
| `--config`       | config file location           | `--config ~/.config/ep/work.toml`        |
| `--root`         | `server.allowed_roots`, repeatable; replaces the list from the file | `--root ~/projects --root /srv/logs` |
| `--timeout`      | `server.timeout`               | `--timeout 10s`                          |
| `--no-update`    | `server.allow_update = false`  |                                          |
| `--log-level`    | log level to stderr: `error`, `warn` (default), `info`, `debug` | `--log-level debug` |
| `--version`      | prints the server and the spec version and exits |                         |

Registration in an MCP client, e.g. `.mcp.json` of Claude Code, see also [TOOLS.md](TOOLS.md#transport):

```json
{
  "mcpServers": {
    "easy-peasy": {
      "command": "easy-peasy-mcp",
      "args": ["--root", "/home/me/projects"]
    }
  }
}
```

## Sections

### server

Can't be changed by a request; only `timeout` and `default_dirs` are replaced for one request by its `timeout` and
`dirs` fields.

| Key             | Type             | Default  | Read-only | Meaning                                                      |
|-----------------|------------------|----------|-----------|--------------------------------------------------------------|
| `allowed_roots` | list of paths    | `[]`     | yes       | the server never searches outside of them; empty — the start directory |
| `default_dirs`  | list of paths    | `["."]`  | no        | working directories of a request without `dirs`              |
| `timeout`       | duration         | `"30s"`  | no        | max time of one search, units `ms`, `s`, `m`; `"0s"` — no limit |
| `threads`       | number           | `0`      | no        | search threads, `0` — number of CPU cores                    |
| `allow_update`  | bool             | `true`   | yes       | `false` disables `update_config`                             |
| `result_cache`  | number           | `50`     | no        | results kept in memory for `within` and `since`, see [OUTPUT.md](OUTPUT.md#result-ids); `0` — off |

Paths

- `~` is expanded to the home directory (`%USERPROFILE%` on Windows)
- relative paths are resolved against the directory the server is started in
- `allowed_roots` are canonicalized at start (symlinks resolved); a root which doesn't exist is an error

A request working directory must be inside an allowed root after canonicalization, otherwise the request fails with
`dir_not_allowed`: `error[dir_not_allowed]: the working directory '<dir>' is outside of the allowed roots`, hint
`get_config shows the allowed roots`.

Working directories (`dirs` of a request, else `server.default_dirs`) are resolved against the start directory and
checked in their order: one which doesn't exist is `dir_not_found` (`the working directory '<dir>' doesn't exist`, or
`... is not a directory`), one outside of the roots `dir_not_allowed`. `default_dirs` must be a non-empty list of
paths (`invalid value for 'server.default_dirs': a non-empty list of paths is expected`); its directories are checked
by every request, not at start.

Durations are a whole number and a unit, `ms`, `s` or `m`: `"500ms"`, `"30s"`, `"2m"`; zero (`"0s"`) is no limit.
An invalid `server.timeout` is `invalid value for 'server.timeout': a duration like "30s" is expected (units ms, s,
m)` at start and the same hint for `invalid_value` of `update_config`; an invalid `--timeout` prints the usage.
`get_config` shows the duration as written. An invalid `server.threads` is `invalid value for 'server.threads': a whole
number is expected (0: the number of CPU cores)`, an invalid `server.allow_update` `invalid value for
'server.allow_update': true or false is expected`, an invalid `server.result_cache` `invalid value for
'server.result_cache': a whole number is expected (0: off)`.

A root which doesn't exist stops the server with `the allowed root '<root>' doesn't exist` — after `<path>:<line>: `
when it comes from the config file, see [Config file](#config-file).

### traversal

| Key                 | Type              | Default    | Meaning                                                            |
|---------------------|-------------------|------------|--------------------------------------------------------------------|
| `min_depth`         | number            | `0`        | skip entries closer to the working directory; its direct children are depth 1 |
| `max_depth`         | number            | not set    | don't descend deeper; not set — unlimited                          |
| `follow_symlinks`   | bool              | `false`    | descend into symlinked directories, read symlinked files           |
| `hidden`            | bool              | `false`    | include hidden entries, see [Platforms](#platforms)                |
| `ignore_files`      | list of names     | `[".gitignore", ".ignore", ".aiignore"]` | ignore files to respect, see [Ignore files](#ignore-files) |
| `git_excludes`      | bool              | `true`     | respect `.git/info/exclude` and the global git excludes file       |
| `ignore_paths`      | list of paths     | `[]`       | extra ignore files applied to every working directory              |
| `exclude`           | list of wildcards | `[]`       | paths which are never walked, matched like `path ~`                |
| `path_case`         | string            | `"native"` | case sensitivity of `path` and `name`: `native`, `sensitive`, `insensitive` |
| `binary`            | string            | `"skip"`   | binary files: `skip` — not searched by content, content conditions are unknown for them, but they are returned by other conditions (see [QUERY.md](QUERY.md#files-whose-content-is-not-searched)); `search` — searched as text; `exclude` — never returned, like `grep -I` |
| `max_file_size`     | size              | `"10M"`    | bigger files are not searched by content, content conditions are unknown for them |
| `encoding`          | string            | `"auto"`   | see [Encoding](#encoding)                                          |
| `fallback_encoding` | string            | not set    | see [Encoding](#encoding)                                          |
| `archives`          | string            | `"none"`   | see [Archives](#archives)                                          |
| `max_archive_size`  | size              | `"100M"`   | see [Archives](#archives)                                          |
| `skip_generated`    | bool              | `true`     | skip files of [kind](#kinds) `generated`, unless the query has a `kind` condition |

An invalid `traversal.min_depth` or `traversal.max_depth` is `invalid value for 'traversal.min_depth': a whole number
is expected`. In a request (`"traversal": { "min_depth": 2 }`) a wrong value is `error[invalid_option]:
'traversal.min_depth' must be a whole number`, a key which isn't one of the section `unknown field 'traversal.<key>'`
with the keys of the section in the hint. A boolean key (`follow_symlinks`, `hidden`, ...) is `invalid value for
'traversal.follow_symlinks': true or false is expected`, in a request `'traversal.follow_symlinks' must be true or
false`. A list of names or paths (`ignore_files`, `ignore_paths`) is `invalid value for 'traversal.ignore_files': a
list of file names is expected` (`a list of paths` for `ignore_paths`), in a request `'traversal.ignore_files' must be
a list of strings`. A list of wildcards (`exclude`) is `invalid value for 'traversal.exclude': a list of wildcards is
expected`, with an invalid wildcard `invalid value for 'traversal.exclude': invalid wildcard '[a-': unclosed '['`; in
a request `'traversal.exclude' must be a list of strings`, with an invalid wildcard `invalid wildcard '[a-' in
'traversal.exclude'` and the hint `unclosed '['`. A key with a fixed set of values (`path_case`, `binary`, `archives`) is `invalid value
for 'traversal.path_case': expected one of: native, sensitive, insensitive`, in a request `'traversal.path_case' must
be one of: native, sensitive, insensitive`. A size (`max_file_size`, `max_archive_size`) is a string like the values of the `size` target
(QUERY.md#sizes): `invalid value for 'traversal.max_file_size': a size like "10M" is expected (units B, K, M, G)`, in
a request `'traversal.max_file_size' must be a size like "10M" (units B, K, M, G)`. An encoding which is neither `auto` nor a
[WHATWG label](#encoding) is `invalid value for 'traversal.encoding': "auto" or an encoding name like
"windows-1252" is expected`, in a request `'traversal.encoding' must be "auto" or an encoding name like
"windows-1252"`. `fallback_encoding` is not `auto`: `invalid value for 'traversal.fallback_encoding': an
encoding name like "windows-1251" is expected`, in a request `'traversal.fallback_encoding' must be an encoding name
like "windows-1251"`; it is unset by `reset`.

Notes

- `exclude` prunes directories, they are not even read, so it is much faster than `not path ~ ...` in the query.
  Wildcards are relative to the working directory: `**/target/**` excludes every `target` directory, `target/**` only
  the top level one. An entry is excluded when a wildcard matches its path, a directory also when a wildcard matches
  its path followed by `/`: `target/**` excludes the directory `target` itself, so `type = dir` doesn't return it.
- With `follow_symlinks` loops are detected and skipped, and a symlink which points outside of the allowed roots is
  skipped with a `permission_denied` warning.
- Files reachable several times (overlapping working directories, symlinks, hard links) are returned once, see
  [OUTPUT.md](OUTPUT.md#duplicates).
- `path_case = "native"`: case-sensitive on Linux, case-insensitive on macOS and Windows. `path_case` applies to `=`,
  `has`, `starts`, `ends`, `~`, `=~` on `path` and `name` and to `exclude`; `/i`, `~~` and `/n` are always
  case-insensitive.
- A file is binary if its first 8K contain a zero byte; the BOM and UTF-16 checks of [Encoding](#encoding) come
  first, so UTF-16 text is not binary.

### Ignore files

Files in the `.gitignore` format which exclude paths from the search. A project can have an `.aiignore` (or any other
name from `ignore_files`) to hide generated code, fixtures or secrets from the agent without touching `.gitignore`.

- every file name from `ignore_files` is looked up in every directory; its rules apply to that directory and below,
  exactly like nested `.gitignore` files
- the rules work in any directory, also outside of a git repository
- when several files apply, a deeper file wins over an upper one, and at the same level a later name in
  `ignore_files` wins over an earlier one; so `!` in `.aiignore` can re-include what `.gitignore` excludes
- `ignore_paths` are ignore files outside of the tree, e.g. a personal `~/.config/easy-peasy-mcp/ignore`; their rules
  are applied as if the file were in the working directory, with the lowest priority, a later file winning over an
  earlier one. `~` is expanded to the home directory, relative paths are resolved against the directory the server is
  started in, like the [paths of the server section](#server); a file which doesn't exist is skipped
- `git_excludes` adds `.git/info/exclude` and the global excludes file of git (`core.excludesFile`)
- `ignore_files = []`, `ignore_paths = []` and `git_excludes = false` turn ignoring off completely

```gitignore
# .aiignore
fixtures/large/
*.min.js
secrets/**
!secrets/README.md
```

### Encoding

Text conditions, columns and the output always work on decoded text; the output is UTF-8.

| `encoding`          | Behavior                                                                     |
|---------------------|------------------------------------------------------------------------------|
| `auto`              | by the BOM: UTF-8, UTF-16 LE / BE, UTF-32 LE / BE; without a BOM — UTF-16 if the zero bytes show it (see below), otherwise UTF-8, and `fallback_encoding` if the file is not valid UTF-8 |
| an encoding name    | every file without a BOM is decoded with it, e.g. `utf-16le`, `windows-1252`  |

- encoding names are the [WHATWG labels](https://encoding.spec.whatwg.org/#names-and-labels): `utf-8`, `utf-16le`,
  `utf-16be`, `windows-1251`, `windows-1252`, `koi8-r`, `iso-8859-2`, `shift_jis`, `gb18030`, ... in any case;
  the output names an encoding by its WHATWG name in lower case (`latin1` → `windows-1252`)
- with an encoding name a file without a BOM is not checked for the UTF-16 pattern of zero bytes; it is binary by a
  zero byte in its first 8K unless the encoding is `utf-16le` or `utf-16be`
- `fallback_encoding` is used only by `auto`, only for files which are not valid UTF-8. Without it the invalid bytes
  are replaced with `�` and the response has a `decode_errors` warning.
- binary detection runs after the BOM check: a UTF-16 file with a BOM is text even though it contains zero bytes.
  A UTF-16 file without a BOM is detected by its pattern of zero bytes (every second byte in ASCII text) and decoded
  as UTF-16, not skipped as binary: in its first 8K more than half of the bytes at odd positions are zero and none at
  even positions (UTF-16 LE), or the other way round (UTF-16 BE).
- the output shows the encoding of every file which is not UTF-8, see [OUTPUT.md](OUTPUT.md#fields)
- line breaks `\n` and `\r\n` are supported in every encoding

### Archives

Searching inside compressed files and archives, like `rg -z`. Off by default.

| `archives`   | Searched                                                                              |
|--------------|---------------------------------------------------------------------------------------|
| `none`       | nothing; archives are binary files                                                    |
| `compressed` | single compressed files: `.gz`, `.bz2`, `.xz`, `.zst`, `.lz4` — as their decompressed content |
| `all`        | `compressed` + archives: `.zip`, `.jar`, `.tar`, `.tar.gz` / `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`, `.7z` |

- a compressed single file keeps its path and name: `logs/app.log.gz`; `size` is the compressed size on disk; a compressed
  file is recognized by its extension, in any case; one which can't be decompressed is not searched by content (its
  content conditions are unknown) and the response has an `archive_error` warning
- with `compressed`, a `.tar.gz` / `.tgz` is a single compressed file too: its decompressed content is a tar archive,
  which is binary; its members are searched only with `all`
- an archive member has the path `<archive path>!/<member path>`, e.g. `logs/2026-10.zip!/app/server.log`; `name`,
  `size` (uncompressed) and `modified` come from the archive entry; `created` and `accessed` are unknown
- the archive itself is also an entry: `name ~ '*.zip'` returns the archive file; it is a binary file, so with
  `binary = exclude` it is not returned
- archives inside archives are not opened
- `max_file_size` applies to the decompressed content of every compressed file and every member
- `max_archive_size` limits the total decompressed content of one archive (protection against zip bombs); the rest is
  skipped with an `archive_too_big` warning: the members count with their uncompressed sizes in the order of the
  archive, the member which would exceed the limit and the ones after it are not entries
- encrypted, corrupted and unsupported archives are not opened: their members are not searched, the archive file
  itself is still an entry; the response has an `archive_error` warning
- members are decompressed in memory, nothing is written to disk; member paths with `..` or absolute paths are
  reported as they are, they are never resolved against the file system

### output

Defaults of the `output` field of the request. The meaning of every key is in [OUTPUT.md](OUTPUT.md#options).

| Key                    | Type        | Default        |
|------------------------|-------------|----------------|
| `format`               | string      | `"text"`       |
| `detail`               | string      | `"lines"`      |
| `budget`               | number      | not set        |
| `chars_per_token`      | number      | `4`            |
| `funnel`               | string      | `"auto"`       |
| `facets`               | list        | `["dir", "lang", "kind", "condition"]` |
| `facet_depth`          | number      | `2`            |
| `facet_limit`          | number      | `8`            |
| `meta`                 | bool        | `false`        |
| `context`              | number      | `0`            |
| `before`               | number      | `context`      |
| `after`                | number      | `context`      |
| `path_fragments`       | bool        | `false`        |
| `highlight`            | string      | `"auto"`       |
| `markers`              | two strings | `["⟦", "⟧"]`   |
| `max_files`            | number      | `100`          |
| `max_lines`            | number      | `500`          |
| `max_matches`          | number      | `1000`         |
| `max_matches_per_file` | number      | `50`           |
| `max_line_length`      | number      | `300`          |
| `max_output_per_file`  | number      | `4000`         |
| `max_output`           | number      | `40000`        |
| `dedent`               | bool        | `true`         |
| `blank_context`        | bool        | `false`        |
| `group_dirs`           | bool        | `true`         |
| `group_repeated`       | number      | `3`            |
| `sort`                 | string      | `"relevance"`  |
| `path_style`           | string      | `"relative"`   |

`markers`: choose characters which don't appear in your files, e.g. `["«", "»"]` or `["[[", "]]"]`.

`max_output` and `max_output_per_file` are in characters; a token is about 3–4 characters of code, so the default
`max_output` is about 10–13 thousand tokens. `budget` is in tokens, converted with `chars_per_token`; tune it for the
tokenizer of your model.

### escape

| Key         | Type   | Default | Meaning                                                                   |
|-------------|--------|---------|---------------------------------------------------------------------------|
| `max_level` | number | `3`     | the upper level of `/e(style, any)` and the max level a query may ask for |

Every level multiplies the number of variants to match, keep it small. An invalid value is `invalid value for
'escape.max_level': a whole number is expected`, in a request `'escape.max_level' must be a whole number`.

### query

| Key               | Type   | Default | Meaning                                                                     |
|-------------------|--------|---------|-----------------------------------------------------------------------------|
| `fuzzy_min_score` | number | `0.5`   | `~~` matches with a lower score (0–1) are dropped, see [QUERY.md](QUERY.md#identifiers-and-fuzzy-names) |

In a request the section is the object `query_options`, because `query` is the query itself. An invalid value is
`invalid value for 'query.fuzzy_min_score': a number from 0 to 1 is expected`, in a request
`'query_options.fuzzy_min_score' must be a number from 0 to 1`. `get_config` writes the number as it is, `1` for
`1.0`.

### kinds

Rules of the `kind` target (see [QUERY.md](QUERY.md#kind)). A file gets the first kind which matches, in the order
`generated`, `test`, `doc`, `config`, `source`, `other`; `source` is a file of a [language](#languages) with
`source = true`.

| Key                      | Type              | Meaning                                                           |
|--------------------------|-------------------|-------------------------------------------------------------------|
| `generated_markers`      | list of strings   | a file with one of them in the first lines is `generated`         |
| `generated_marker_lines` | number            | how many first lines are checked for the markers                  |
| `generated`              | list of wildcards | paths of generated files                                          |
| `test`                   | list of wildcards | paths of tests                                                    |
| `doc`                    | list of wildcards | paths of documentation                                            |
| `config`                 | list of wildcards | paths of configuration                                            |

The defaults are in the [full example](#full-example). Wildcards are matched like `path ~`, relative to the working
directory; the defaults start with `**/`, so the kind of a file doesn't depend on the working directory. Markers are
compared case-sensitively with the first lines as they are. A list in the config file replaces the built-in list:

```toml
[kinds]
test = ["**/tests/**", "**/*_test.go", "**/e2e/**"]
generated = ["**/*.lock", "**/src/gen/**", "**/*.pb.go"]
```

Errors: `invalid value for 'kinds.generated_markers': a list of strings is expected`; in a request (the object
`kinds`) `'kinds.generated_markers' must be a list of strings`. `generated_marker_lines` is a whole number of at least 1: `invalid
value for 'kinds.generated_marker_lines': a whole number of at least 1 is expected`, in a request
`'kinds.generated_marker_lines' must be a whole number of at least 1`; every key with a minimum (see the input schema
in [TOOLS.md](TOOLS.md#input-schema)) says it the same way. The wildcard lists `generated`, `test`, `doc`, `config` are checked
like [`traversal.exclude`](#traversal): `invalid value for 'kinds.test': a list of wildcards is expected`, `invalid
wildcard '[a-' in 'kinds.test'`.

### git

| Key    | Type   | Default  | Meaning                                                                         |
|--------|--------|----------|---------------------------------------------------------------------------------|
| `base` | string | `"auto"` | the base of `git = branch`: a branch, tag or commit; `auto` — see below         |

`auto` takes the first one which exists: the upstream of the remote default branch (`origin/HEAD`), `main`, `master`.
`git = branch` is the files changed by the commits between the merge base of the base and `HEAD`, plus the
uncommitted changes. On the base branch itself it is the same as `changed`.

The git status is read only if the query has a `git` condition or the `git` facet is requested, so searches without
it don't pay for it. It is read by the `git` program (`git status` and `git ls-files` once per repository), which
must be installed; without it every file is outside of a repository. The server runs it with `core.fsmonitor`
off, so the configuration of a cloned repository can't start a program through it.

### Languages

The language table is used by the `lang` target (see [QUERY.md](QUERY.md#lang)), by `kind = source` and by the `/c`
modifier.

| Key                     | Type                      | Meaning                                                       |
|-------------------------|---------------------------|---------------------------------------------------------------|
| `extensions`            | list of strings           | file extensions without the dot                               |
| `names`                 | list of strings           | full file names, e.g. `Dockerfile`, `Makefile`                |
| `shebangs`              | list of strings           | interpreters in the shebang of files without an extension, e.g. `python3` |
| `line_comments`         | list of strings           | start a comment until the end of the line                     |
| `block_comments`        | list of `[start, end]`    | start and end of a block comment                              |
| `nested_block_comments` | bool                      | block comments can be nested (Rust, Haskell)                  |
| `strings`               | list of strings           | string delimiters; comment markers inside strings are ignored |
| `source`                | bool                      | `true` (default) — code (programming languages, also HTML and CSS), its files are `kind = source`; `false` — a data or document format |

How a file gets its language, the first rule which matches:

1. the full file name is in `names` (`Dockerfile`, `.env`)
2. the extension, the part after the last `.` of a name which doesn't start with it, is in `extensions`
3. a file without an extension whose first line is a shebang: the interpreter — the program, or the first argument
   of `env` — is in `shebangs`, as it is or without a trailing version (`python3.11` → `python`)

Names and extensions are compared case-insensitively. Directories and other files have the language `unknown`.

#### Built-in languages

| Language     | Extensions, names, shebangs                                      | Line comment  | Block comment | Strings         |
|--------------|------------------------------------------------------------------|---------------|--------------------|-----------------|
| `c`          | `c`, `h`                                                         | `//`          | `/* */` | `"`, `'`        |
| `cpp`        | `cpp`, `cc`, `cxx`, `hpp`, `hh`, `hxx`                           | `//`          | `/* */` | `"`, `'`        |
| `csharp`     | `cs`                                                             | `//`          | `/* */` | `"`, `'`        |
| `java`       | `java`                                                           | `//`          | `/* */` | `"`, `'`        |
| `kotlin`     | `kt`, `kts`                                                      | `//`          | `/* */`, nested | `"`, `'`        |
| `scala`      | `scala`, `sc`                                                    | `//`          | `/* */`, nested | `"`, `'`        |
| `go`         | `go`                                                             | `//`          | `/* */` | `"`, `` ` ``    |
| `swift`      | `swift`                                                          | `//`          | `/* */`, nested | `"`, `'`        |
| `dart`       | `dart`                                                           | `//`          | `/* */`, nested | `"`, `'`        |
| `groovy`     | `groovy`, `gradle`                                               | `//`          | `/* */` | `"`, `'`        |
| `javascript` | `js`, `jsx`, `mjs`, `cjs`; shebang `node`                        | `//`          | `/* */` | `"`, `'`, `` ` `` |
| `typescript` | `ts`, `tsx`, `mts`, `cts`                                        | `//`          | `/* */` | `"`, `'`, `` ` `` |
| `rust`       | `rs`                                                             | `//`          | `/* */`, nested | `"`             |
| `php`        | `php`                                                            | `//`, `#`     | `/* */` | `"`, `'`        |
| `css`        | `css`                                                            | —             | `/* */` | `"`, `'`        |
| `scss`       | `scss`, `less`                                                   | `//`          | `/* */` | `"`, `'`        |
| `python`     | `py`, `pyi`; shebang `python`, `python3`                         | `#`           | — | `"`, `'`        |
| `ruby`       | `rb`; `Gemfile`, `Rakefile`; shebang `ruby`                      | `#`           | `=begin =end` | `"`, `'`        |
| `shell`      | `sh`, `bash`, `zsh`; shebang `sh`, `bash`, `zsh`                 | `#`           | — | `"`, `'`        |
| `perl`       | `pl`, `pm`; shebang `perl`                                       | `#`           | — | `"`, `'`        |
| `r`          | `r`                                                              | `#`           | — | `"`, `'`        |
| `powershell` | `ps1`, `psm1`                                                    | `#`           | `<# #>` | `"`, `'`        |
| `batch`      | `bat`, `cmd`                                                     | `REM`, `::`   | — | `"`             |
| `sql`        | `sql`                                                            | `--`          | `/* */` | `"`, `'`        |
| `lua`        | `lua`                                                            | `--`          | `--[[ ]]` | `"`, `'`        |
| `haskell`    | `hs`                                                             | `--`          | `{- -}`, nested | `"`             |
| `clojure`    | `clj`, `cljs`, `cljc`                                            | `;`           | — | `"`             |
| `lisp`       | `lisp`, `el`, `scm`                                              | `;`           | — | `"`             |
| `html`       | `html`, `htm`                                                    | —             | `<!-- -->` | `"`             |
| `xml`        | `xml`, `svg`, `xsd`, `xsl`                                       | —             | `<!-- -->` | `"`             |
| `vue`        | `vue`                                                            | `//`          | `<!-- -->`, `/* */` | `"`, `'`, `` ` `` |
| `markdown`   | `md`, `markdown`                                                 | —             | `<!-- -->` | —               |
| `yaml`       | `yml`, `yaml`                                                    | `#`           | — | `"`             |
| `toml`       | `toml`                                                           | `#`           | — | `"`             |
| `ini`        | `ini`, `cfg`, `conf`                                             | `#`, `;`      | — | `"`             |
| `properties` | `properties`                                                     | `#`, `!`      | — | `"`             |
| `dotenv`     | `env`; `.env`                                                    | `#`           | — | `"`             |
| `json`       | `json`                                                           | —             | — | `"`             |
| `jsonc`      | `jsonc`, `json5`                                                 | `//`          | `/* */` | `"`             |
| `dockerfile` | `dockerfile`; `Dockerfile`                                       | `#`           | — | `"`             |
| `makefile`   | `mk`; `Makefile`, `GNUmakefile`                                  | `#`           | — | `"`             |

Data and document formats have `source = false`: `json`, `jsonc`, `yaml`, `toml`, `ini`, `properties`, `dotenv`, `xml`,
`markdown`. They are found by `lang`, but their files are not `kind = source`; without a matching `config` or `doc`
rule they are `other`.

#### Comment syntax

The `/c` modifier (see [QUERY.md](QUERY.md#comments)) uses the comment keys of the language of the file. Files of an
unknown language are searched as is.

- the file is read from the start: outside of strings and comments, the longest marker at a position starts a string,
  a line comment or a block comment; a line comment ends at the end of the line, a block comment at its end marker
  (with `nested_block_comments` after as many end markers as start markers), the markers belong to the comment
- a string ends at the same delimiter which is not escaped with `\`; a string of `"` or `'` also ends at the end of
  the line, other delimiters (`` ` ``) may span lines; comment markers inside a string are text
- a match is inside a comment when its first character is

Override or add a language:

```toml
# Terraform
[languages.terraform]
extensions = ["tf", "tfvars"]
line_comments = ["#", "//"]
block_comments = [["/*", "*/"]]
strings = ['"']

# treat .tpl files as HTML
[languages.html]
extensions = ["html", "htm", "tpl"]
block_comments = [["<!--", "-->"]]
```

A language section with the same name as a built-in one replaces it completely. An extension or a name listed in two
languages is an error at start.

## Platforms

The server is built natively for Linux, macOS and Windows and follows the conventions of each platform.

| Topic                  | Linux                        | macOS                                | Windows                                    |
|------------------------|------------------------------|--------------------------------------|--------------------------------------------|
| Output paths           | `/`                          | `/`                                  | `\`                                        |
| Query paths            | `/`                          | `/`                                  | `/` and `\`                                |
| `path_case = native`   | case-sensitive               | case-insensitive                     | case-insensitive                           |
| Hidden entries         | name starts with `.`         | name starts with `.` or the `hidden` flag | name starts with `.` or the hidden attribute |
| `created`              | if the file system supports it (`statx`) | yes                      | yes                                        |
| `accessed`             | yes, may be stale with `noatime` / `relatime` | yes                 | yes, may be disabled on NTFS               |
| `type = link`          | symlinks                     | symlinks                             | symlinks and junctions                     |
| Duplicate detection    | device + inode               | device + inode                       | volume serial + file index                 |
| Absolute paths         | `/home/me/p`                 | `/Users/me/p`                        | `C:\Users\me\p`, `\\server\share\p`; the `\\?\` prefix is never shown |
| Unicode names          | compared in NFC form         | compared in NFC form                 | compared in NFC form                       |
| Config directory       | `~/.config/easy-peasy-mcp`   | `~/Library/Application Support/easy-peasy-mcp` | `%APPDATA%\easy-peasy-mcp`       |

Windows paths in TOML are written in single quotes, where `\` is not an escape: `allowed_roots = ['C:\Users\me\work']`.

## get_config

Returns the effective configuration: all values after merging built-in defaults, the config file, command line and
`update_config` calls. The tool definition is in [TOOLS.md](TOOLS.md#get_config).

The result is TOML, the same format as the config file, so it can be copied into `config.toml` as is.

Request — all fields are optional:

```json
{ "section": "traversal" }
```

| Field     | Meaning                                                               |
|-----------|-----------------------------------------------------------------------|
| `section` | `server`, `traversal`, `output`, `escape`, `query`, `kinds`, `git`, `languages`; not set — all |
| `sources` | `true` — add a comment with the source of every value                 |

Result:

```toml
# easy-peasy-mcp 0.1.0, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 0
# max_depth =  (not set)
follow_symlinks = false
hidden = false
ignore_files = [".gitignore", ".ignore", ".aiignore"]
git_excludes = true
ignore_paths = []
exclude = ["**/target/**", "**/node_modules/**"]
path_case = "native"
binary = "skip"
max_file_size = "50M"
encoding = "auto"
# fallback_encoding =  (not set)
archives = "none"
max_archive_size = "100M"
skip_generated = true
```

With `"sources": true` every value gets a comment:

```toml
[traversal]
exclude = ["**/target/**", "**/node_modules/**"]    # file: /home/me/.config/easy-peasy-mcp/config.toml
hidden = false    # default
max_file_size = "50M"    # update
```

A comment follows its line after four spaces; comments are not aligned, so the result doesn't depend on the length of
the paths.

`server.allowed_roots` shows the canonical roots in effect, so an empty list shows the start directory. Paths are
written as TOML literal strings (`'...'`), so Windows paths need no escaping; a path with a `'` is a basic string.

Sources: `default`, `file: <path>`, `cli`, `update`. `cli` appears only for the values the command line can set:
`server.allowed_roots`, `server.timeout`, `server.allow_update`. Values which are not set (e.g. `max_depth` without a
limit) are written as comments: `# max_depth =  (not set)`.

An agent should call `get_config` once at the start to learn the allowed roots and limits instead of guessing.

## update_config

Changes the configuration in memory until the server stops. The config file is never written. The tool definition is
in [TOOLS.md](TOOLS.md#update_config).

The new values are a TOML fragment in the `set` field, written like the config file. The tool arguments themselves
are JSON, as MCP requires, so the TOML is a JSON string:

```json
{
  "set": "[output]\nmax_files = 30\ncontext = 2\n\n[traversal]\nexclude = [\"**/target/**\"]",
  "reset": ["output.format"]
}
```

The `set` value is this TOML:

```toml
[output]
max_files = 30
context = 2

[traversal]
exclude = ["**/target/**"]
```

Dotted keys are shorter for one or two values and need no line breaks:

```json
{ "set": "output.context = 2" }
```

| Field   | Meaning                                                              |
|---------|----------------------------------------------------------------------|
| `set`   | TOML with the values to change                                       |
| `reset` | keys to return to the value from the file / command line / default; `["all"]` resets everything |

- lists are replaced, not merged: to add an exclude, send the whole new list
- the update is atomic: if any value is invalid, nothing is changed
- `reset` is applied before `set`
- a search which is already running keeps the configuration it started with

Result — TOML with the keys whose value changed (in the order of the spec), the old value in a comment four spaces
after the line, like in `get_config`; `, reset` when the value comes from `reset`. A value which is not set is
written like in `get_config` and as `(not set)` in the comment: `max_depth = 3    # was (not set)`,
`# max_depth =  (not set)    # was 3, reset`. A key which
is both reset and set gets the value of `set`. A changed value has the source `update` in `get_config`; a reset value
gets its source back.

```toml
[output]
format = "text"    # was "json", reset
context = 2    # was 0
max_files = 30    # was 100

[traversal]
exclude = ["**/target/**"]    # was []
```

A call which changes nothing (no `set`, an empty `reset`) returns `# nothing changed`.

Errors — the first one in the order `reset`, then `set` in the order of the TOML:

| Code             | Example                                             | Message                                    | Hint                                    |
|------------------|-----------------------------------------------------|--------------------------------------------|-----------------------------------------|
| `invalid_toml`   | `set`: `output.context = `                          | invalid TOML in 'set'                      | line 1, col 18: string values must be quoted, expected literal string |
| `read_only`      | `set`: `server.allowed_roots = ["/"]`               | 'server.allowed_roots' is read-only        | allowed roots are set only at start     |
| `read_only`      | `reset`: `["server.allow_update"]`                  | 'server.allow_update' is read-only         | allow_update is set only at start       |
| `update_disabled`| any call with `allow_update = false`                | update_config is disabled                  | the configuration has server.allow_update = false |
| `unknown_key`    | `set`: `output.max_file = 10`                       | unknown key 'output.max_file'              | did you mean 'output.max_files'?        |
| `unknown_key`    | `set`: `[colors]`                                   | unknown key 'colors'                       | expected one of: server, traversal, output, escape, query, kinds, git, languages |
| `invalid_value`  | `set`: `output.detail = "all"`                      | invalid value for 'output.detail'          | expected one of: exists, count, facets, files, lines |

The hint of `invalid_toml` is the line and the column (1-based, in characters) and the message of the TOML parser.
A section written as a value (`output = 1`) is `unknown_key` with the message `'output' must be a section`; a section
name in `reset` is `unknown_key` with the hint `expected a key like 'output.max_files', or 'all'`.
Arguments which don't match the input schema are `invalid_option`, like in `find`.

## Security

- The server only reads. There is no command which writes, moves or deletes files, and `update_config` never writes
  the config file.
- `allowed_roots` can't be changed by the agent: neither `update_config` nor a request can widen them.
- Every working directory, every symlink target and every file of a `within` result is canonicalized and checked
  against the allowed roots; a cursor holds no paths, only the request hash, a position and the totals.
- `..` in `dirs` is allowed as long as the result is inside an allowed root.
- Files the server process can't read are skipped with a warning; the server never tries to change permissions.
- Archives are decompressed in memory with `max_file_size` and `max_archive_size` limits, nested archives are not
  opened, member paths are never resolved against the file system.
- `timeout`, `max_file_size` and the output limits protect against huge results and slow searches on big trees. The
  `regex` crate itself guarantees linear time, so a single regex can't hang the server.

## Examples

### One project, defaults tuned for a Rust workspace

```toml
[server]
allowed_roots = ["~/projects/my-app"]

[traversal]
exclude = ["**/target/**", "**/*.lock"]

[output]
context = 1
max_files = 50
```

### Several roots, read-only configuration, logs with archives

The agent can search the code and the logs, including rotated `.gz` and `.zip` logs, but can't change the
configuration:

```toml
[server]
allowed_roots = ["~/projects", "/var/log/my-app"]
default_dirs = ["~/projects"]
timeout = "60s"
allow_update = false

[traversal]
max_file_size = "200M"          # logs are big
archives = "all"
max_archive_size = "1G"
```

### Frontend monorepo

```toml
[server]
allowed_roots = ["~/work/web"]

[traversal]
exclude = ["**/node_modules/**", "**/dist/**", "**/.next/**", "**/coverage/**"]
max_file_size = "2M"            # don't search the content of big bundles and snapshots

[output]
max_line_length = 200           # minified code has huge lines
```

### Windows, legacy encoding

Old projects with sources in Windows-1251 next to UTF-8 ones:

```toml
[server]
allowed_roots = ['C:\work\legacy', 'D:\src']

[traversal]
encoding = "auto"
fallback_encoding = "windows-1251"
```

### Tune the session from the agent

The agent wants more context for the rest of the session:

```json
{ "set": "output.context = 3" }
```

And once searches inside `node_modules`, which is in `.gitignore`, with a short timeout:

```json
{
  "dirs": ["node_modules/some-lib"],
  "query": "text has 'deprecated' /i",
  "timeout": "5s",
  "traversal": { "ignore_files": [] }
}
```

The second request changes `ignore_files` and `timeout` for this request only.
