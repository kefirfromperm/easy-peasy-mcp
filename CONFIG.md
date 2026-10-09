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

A request can replace any configuration value — `traversal`, `output`, `escape`, `query` (as the request
field `query_options`), `kinds`, `git`, `languages` and the `timeout` —
except the `server` section: the allowed roots, which bound the working directories, can't be changed by a request.
Values are replaced, not merged: a request `exclude` list replaces the configured list.

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

Without a file the built-in defaults are used. An invalid file stops the server at start with an error that names the
key and the line.

There is no project level config file (e.g. in a repository): a cloned repository must not be able to change what the
server is allowed to read. Project specific exclusions are done with [ignore files](#ignore-files) like `.aiignore`.

### Full example

All keys with their default values:

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
binary = "skip"                 # skip | search | list
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
doc = ["**/*.md", "**/*.rst", "**/*.adoc", "**/*.txt", "docs/**", "**/LICENSE*", "**/CHANGELOG*"]
config = ["*.json", "**/*.toml", "**/*.yaml", "**/*.yml", "**/*.ini", "**/*.conf", "**/*.properties", "**/*.env", "config/**", "**/Dockerfile", "**/Makefile", ".github/**", ".gitlab-ci.yml"]

[git]
base = "auto"

# Languages, see "Languages" below. Built-in languages can be overridden, new ones added.
[languages.rust]
extensions = ["rs"]
line_comments = ["//"]
block_comments = [["/*", "*/"]]
nested_block_comments = true
strings = ['"']
```

## Command line

```
easy-peasy-mcp [--config <path>] [--root <dir>]... [--timeout <duration>] [--no-update] [--log-level <level>]
```

| Argument         | Overrides                      | Example                                  |
|------------------|--------------------------------|------------------------------------------|
| `--config`       | config file location           | `--config ~/.config/ep/work.toml`        |
| `--root`         | `server.allowed_roots`, repeatable; replaces the list from the file | `--root ~/projects --root /srv/logs` |
| `--timeout`      | `server.timeout`               | `--timeout 10s`                          |
| `--no-update`    | `server.allow_update = false`  |                                          |
| `--log-level`    | log level to stderr: `error`, `warn`, `info`, `debug` | `--log-level debug` |
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

Can't be changed by a request, except `timeout` through the request `timeout` field.

| Key             | Type             | Default  | Read-only | Meaning                                                      |
|-----------------|------------------|----------|-----------|--------------------------------------------------------------|
| `allowed_roots` | list of paths    | `[]`     | yes       | the server never reads outside of them; empty — the start directory |
| `default_dirs`  | list of paths    | `["."]`  | no        | working directories of a request without `dirs`              |
| `timeout`       | duration         | `"30s"`  | no        | max time of one search, units `ms`, `s`, `m`; `0` — no limit |
| `threads`       | number           | `0`      | no        | search threads, `0` — number of CPU cores                    |
| `allow_update`  | bool             | `true`   | yes       | `false` disables `update_config`                             |
| `result_cache`  | number           | `50`     | no        | results kept in memory for `within` and `since`, see [OUTPUT.md](OUTPUT.md#result-ids); `0` — off |

Paths

- `~` is expanded to the home directory (`%USERPROFILE%` on Windows)
- relative paths are resolved against the directory the server is started in
- `allowed_roots` are canonicalized at start (symlinks resolved); a root which doesn't exist is an error

A request working directory must be inside an allowed root after canonicalization, otherwise the request fails with
`dir_not_allowed`.

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
| `binary`            | string            | `"skip"`   | binary files: `skip` — content conditions are false, `search` — search them as text, `list` — content conditions are false, but files are still returned by metadata conditions |
| `max_file_size`     | size              | `"10M"`    | bigger files are not searched by content                           |
| `encoding`          | string            | `"auto"`   | see [Encoding](#encoding)                                          |
| `fallback_encoding` | string            | not set    | see [Encoding](#encoding)                                          |
| `archives`          | string            | `"none"`   | see [Archives](#archives)                                          |
| `max_archive_size`  | size              | `"100M"`   | see [Archives](#archives)                                          |
| `skip_generated`    | bool              | `true`     | skip files of [kind](#kinds) `generated`, unless the query has a `kind` condition |

Notes

- `exclude` prunes directories, they are not even read, so it is much faster than `not path ~ ...` in the query.
  Wildcards are relative to the working directory: `**/target/**` excludes every `target` directory, `target/**` only
  the top level one.
- With `follow_symlinks` loops are detected and skipped, and a symlink which points outside of the allowed roots is
  skipped with a `permission_denied` warning.
- Files reachable several times (overlapping working directories, symlinks, hard links) are returned once, see
  [OUTPUT.md](OUTPUT.md#paths).
- `path_case = "native"`: case-sensitive on Linux, case-insensitive on macOS and Windows.

### Ignore files

Files in the `.gitignore` format which exclude paths from the search. A project can have an `.aiignore` (or any other
name from `ignore_files`) to hide generated code, fixtures or secrets from the agent without touching `.gitignore`.

- every file name from `ignore_files` is looked up in every directory; its rules apply to that directory and below,
  exactly like nested `.gitignore` files
- the rules work in any directory, also outside of a git repository
- when several files apply, a deeper file wins over an upper one, and at the same level a later name in
  `ignore_files` wins over an earlier one; so `!` in `.aiignore` can re-include what `.gitignore` excludes
- `ignore_paths` are ignore files outside of the tree, e.g. a personal `~/.config/easy-peasy-mcp/ignore`; their rules
  are applied as if the file were in the working directory, with the lowest priority
- `git_excludes` adds `.git/info/exclude` and the global excludes file of git (`core.excludesFile`)
- `ignore_files = []` and `git_excludes = false` turn ignoring off completely

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
| `auto`              | by the BOM: UTF-8, UTF-16 LE / BE, UTF-32 LE / BE; without a BOM — UTF-8, and `fallback_encoding` if the file is not valid UTF-8 |
| an encoding name    | every file without a BOM is decoded with it, e.g. `utf-16le`, `windows-1252`  |

- encoding names are the [WHATWG labels](https://encoding.spec.whatwg.org/#names-and-labels): `utf-8`, `utf-16le`,
  `utf-16be`, `windows-1251`, `windows-1252`, `koi8-r`, `iso-8859-2`, `shift_jis`, `gb18030`, ...
- `fallback_encoding` is used only by `auto`, only for files which are not valid UTF-8. Without it the invalid bytes
  are replaced with `�` and the response has a `decode_errors` warning.
- binary detection runs after the BOM check: a UTF-16 file with a BOM is text even though it contains zero bytes.
  A UTF-16 file without a BOM is detected by its pattern of zero bytes (every second byte in ASCII text) and decoded
  as UTF-16, not skipped as binary.
- the output shows the encoding of every file which is not UTF-8, see [OUTPUT.md](OUTPUT.md#fields)
- line breaks `\n` and `\r\n` are supported in every encoding

### Archives

Searching inside compressed files and archives, like `rg -z`. Off by default.

| `archives`   | Searched                                                                              |
|--------------|---------------------------------------------------------------------------------------|
| `none`       | nothing; archives are binary files                                                    |
| `compressed` | single compressed files: `.gz`, `.bz2`, `.xz`, `.zst`, `.lz4` — as their decompressed content |
| `all`        | `compressed` + archives: `.zip`, `.jar`, `.tar`, `.tar.gz` / `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`, `.7z` |

- a compressed single file keeps its path and name: `logs/app.log.gz`; `size` is the compressed size on disk
- an archive member has the path `<archive path>!/<member path>`, e.g. `logs/2026-10.zip!/app/server.log`; `name`,
  `size` (uncompressed) and `modified` come from the archive entry; `created` and `accessed` are false
- the archive itself is also an entry: `name ~ '*.zip'` returns the archive file
- archives inside archives are not opened
- `max_file_size` applies to the decompressed content of every member
- `max_archive_size` limits the total decompressed content of one archive (protection against zip bombs); the rest is
  skipped with an `archive_too_big` warning
- encrypted, corrupted and unsupported archives are skipped with an `archive_error` warning
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

Every level multiplies the number of variants to match, keep it small.

### query

| Key               | Type   | Default | Meaning                                                                     |
|-------------------|--------|---------|-----------------------------------------------------------------------------|
| `fuzzy_min_score` | number | `0.5`   | `~~` matches with a lower score (0–1) are dropped, see [QUERY.md](QUERY.md#identifiers-and-fuzzy-names) |

### kinds

Rules of the `kind` target (see [QUERY.md](QUERY.md#kind)). A file gets the first kind which matches, in the order
`generated`, `test`, `doc`, `config`, `source`, `other`; `source` is any file with a known [language](#languages).

| Key                      | Type              | Meaning                                                           |
|--------------------------|-------------------|-------------------------------------------------------------------|
| `generated_markers`      | list of strings   | a file with one of them in the first lines is `generated`         |
| `generated_marker_lines` | number            | how many first lines are checked for the markers                  |
| `generated`              | list of wildcards | paths of generated files                                          |
| `test`                   | list of wildcards | paths of tests                                                    |
| `doc`                    | list of wildcards | paths of documentation                                            |
| `config`                 | list of wildcards | paths of configuration                                            |

The defaults are in the [full example](#full-example). Wildcards are matched like `path ~`, relative to the working
directory. A list in the config file replaces the built-in list:

```toml
[kinds]
test = ["**/tests/**", "**/*_test.go", "e2e/**"]
generated = ["**/*.lock", "src/gen/**", "**/*.pb.go"]
```

### git

| Key    | Type   | Default  | Meaning                                                                         |
|--------|--------|----------|---------------------------------------------------------------------------------|
| `base` | string | `"auto"` | the base of `git = branch`: a branch, tag or commit; `auto` — see below         |

`auto` takes the first one which exists: the upstream of the remote default branch (`origin/HEAD`), `main`, `master`.
`git = branch` is the files changed by the commits between the merge base of the base and `HEAD`, plus the
uncommitted changes. On the base branch itself it is the same as `changed`.

The git status is read only if the query has a `git` condition or the `git` facet is requested, so searches without
it don't pay for it.

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

#### Built-in languages

| Language     | Extensions, names, shebangs                                      | Line comment  | Block comment      |
|--------------|------------------------------------------------------------------|---------------|--------------------|
| `c`          | `c`, `h`                                                         | `//`          | `/* */`            |
| `cpp`        | `cpp`, `cc`, `cxx`, `hpp`, `hh`, `hxx`                           | `//`          | `/* */`            |
| `csharp`     | `cs`                                                             | `//`          | `/* */`            |
| `java`       | `java`                                                           | `//`          | `/* */`            |
| `kotlin`     | `kt`, `kts`                                                      | `//`          | `/* */`, nested    |
| `scala`      | `scala`, `sc`                                                    | `//`          | `/* */`, nested    |
| `go`         | `go`                                                             | `//`          | `/* */`            |
| `swift`      | `swift`                                                          | `//`          | `/* */`, nested    |
| `dart`       | `dart`                                                           | `//`          | `/* */`, nested    |
| `groovy`     | `groovy`, `gradle`                                               | `//`          | `/* */`            |
| `javascript` | `js`, `jsx`, `mjs`, `cjs`; shebang `node`                        | `//`          | `/* */`            |
| `typescript` | `ts`, `tsx`, `mts`, `cts`                                        | `//`          | `/* */`            |
| `rust`       | `rs`                                                             | `//`          | `/* */`, nested    |
| `php`        | `php`                                                            | `//`, `#`     | `/* */`            |
| `css`        | `css`                                                            | —             | `/* */`            |
| `scss`       | `scss`, `less`                                                   | `//`          | `/* */`            |
| `python`     | `py`, `pyi`; shebang `python`, `python3`                         | `#`           | —                  |
| `ruby`       | `rb`; `Gemfile`, `Rakefile`; shebang `ruby`                      | `#`           | `=begin =end`      |
| `shell`      | `sh`, `bash`, `zsh`; shebang `sh`, `bash`, `zsh`                 | `#`           | —                  |
| `perl`       | `pl`, `pm`; shebang `perl`                                       | `#`           | —                  |
| `r`          | `r`                                                              | `#`           | —                  |
| `powershell` | `ps1`, `psm1`                                                    | `#`           | `<# #>`            |
| `batch`      | `bat`, `cmd`                                                     | `REM`, `::`   | —                  |
| `sql`        | `sql`                                                            | `--`          | `/* */`            |
| `lua`        | `lua`                                                            | `--`          | `--[[ ]]`          |
| `haskell`    | `hs`                                                             | `--`          | `{- -}`, nested    |
| `clojure`    | `clj`, `cljs`, `cljc`                                            | `;`           | —                  |
| `lisp`       | `lisp`, `el`, `scm`                                              | `;`           | —                  |
| `html`       | `html`, `htm`                                                    | —             | `<!-- -->`         |
| `xml`        | `xml`, `svg`, `xsd`, `xsl`                                       | —             | `<!-- -->`         |
| `vue`        | `vue`                                                            | `//`          | `<!-- -->`, `/* */` |
| `markdown`   | `md`, `markdown`                                                 | —             | `<!-- -->`         |
| `yaml`       | `yml`, `yaml`                                                    | `#`           | —                  |
| `toml`       | `toml`                                                           | `#`           | —                  |
| `ini`        | `ini`, `cfg`, `conf`                                             | `#`, `;`      | —                  |
| `properties` | `properties`                                                     | `#`, `!`      | —                  |
| `dotenv`     | `env`; `.env`                                                    | `#`           | —                  |
| `json`       | `json`                                                           | —             | —                  |
| `jsonc`      | `jsonc`, `json5`                                                 | `//`          | `/* */`            |
| `dockerfile` | `dockerfile`; `Dockerfile`                                       | `#`           | —                  |
| `makefile`   | `mk`; `Makefile`, `GNUmakefile`                                  | `#`           | —                  |

#### Comment syntax

The `/c` modifier (see [QUERY.md](QUERY.md#comments)) uses the comment keys of the language of the file. Files of an
unknown language are searched as is.

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
| Unicode names          | as stored                    | normalized to NFC for matching       | as stored                                  |
| Config directory       | `~/.config/easy-peasy-mcp`   | `~/Library/Application Support/easy-peasy-mcp` | `%APPDATA%\easy-peasy-mcp`       |

Windows paths in TOML are written in single quotes, where `\` is not an escape: `allowed_roots = ['C:\Users\me\work']`.

## get_config

Returns the effective configuration: all values after merging built-in defaults, the config file, command line and
`update_config` calls. The tool definition is in [TOOLS.md](TOOLS.md#get_config).

The result is TOML, the same format as the config file, so it can be copied into `config.toml` as is.

Request — all fields are optional:

```json
{ "section": "traversal", "sources": true }
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
follow_symlinks = false
hidden = false
ignore_files = [".gitignore", ".ignore", ".aiignore"]
git_excludes = true
ignore_paths = []
exclude = ["**/target/**", "**/node_modules/**"]
path_case = "native"
binary = "skip"
max_file_size = "10M"
encoding = "auto"
archives = "none"
max_archive_size = "100M"
```

With `"sources": true` every value gets a comment:

```toml
[traversal]
exclude = ["**/target/**", "**/node_modules/**"]    # file: /home/me/.config/easy-peasy-mcp/config.toml
hidden = false                                      # default
max_file_size = "50M"                               # update
max_depth = 5                                       # cli
```

Sources: `default`, `file: <path>`, `cli`, `update`. Values which are not set (e.g. `max_depth` without a limit) are
written as comments: `# max_depth =  (not set)`.

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

Result — TOML with the changed keys, the old value in a comment:

```toml
[output]
max_files = 30          # was 100
context = 2             # was 0
format = "text"         # was "json", reset

[traversal]
exclude = ["**/target/**"]    # was []
```

Errors:

| Code             | Example `set`                                       | Hint                                    |
|------------------|-----------------------------------------------------|-----------------------------------------|
| `invalid_toml`   | `output.context = `                                 | line 1, col 18: a value is expected     |
| `read_only`      | `server.allowed_roots = ["/"]`                      | allowed roots are set only at start     |
| `update_disabled`| any call with `allow_update = false`                |                                         |
| `unknown_key`    | `output.max_file = 10`                              | did you mean `max_files`?               |
| `invalid_value`  | `output.detail = "all"`                             | expected one of: exists, count, files, lines |

## Security

- The server only reads. There is no command which writes, moves or deletes files, and `update_config` never writes
  the config file.
- `allowed_roots` can't be changed by the agent: neither `update_config` nor a request can widen them.
- Every working directory, every symlink target and every path from a cursor is canonicalized and checked against the
  allowed roots.
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
max_file_size = "2M"            # skip bundles and snapshots

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
