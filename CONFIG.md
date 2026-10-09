# Configuration

The configuration holds the security boundaries of the server and the defaults of every `find` request. The query is
described in [QUERY.md](QUERY.md), the output in [OUTPUT.md](OUTPUT.md).

- [Where the values come from](#where-the-values-come-from)
- [Config file](#config-file)
- [Command line](#command-line)
- [Sections](#sections)
  - [server](#server)
  - [traversal](#traversal)
  - [output](#output)
  - [escape](#escape)
  - [Comment syntax](#comment-syntax)
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

Read-only values (`server.allowed_roots`, `server.allow_update`) can be set only by 2 and 3.

## Config file

TOML. The server looks for it in this order and takes the first one found:

1. `--config <path>`
2. `$EASY_PEASY_CONFIG`
3. `$XDG_CONFIG_HOME/easy-peasy-mcp/config.toml` (usually `~/.config/easy-peasy-mcp/config.toml`)

Without a file the built-in defaults are used. An invalid file stops the server at start with an error that names the
key and the line.

There is no project level config file (e.g. in a repository): a cloned repository must not be able to change what the
server is allowed to read.

### Full example

All keys with their default values:

```toml
[server]
allowed_roots = []              # empty: the directory the server is started in
default_dirs = ["."]            # working directories when the request has no `dirs`
timeout = "30s"
threads = 0                     # 0: number of CPU cores
allow_update = true             # whether `update_config` is available

[traversal]
min_depth = 0
# max_depth = 10                # not set: unlimited
follow_symlinks = false
hidden = false
ignore_files = true             # .gitignore, .ignore, .git/info/exclude, global git excludes
exclude = []                    # wildcards of paths which are never walked
binary = "skip"                 # skip | search | list
max_file_size = "10M"

[output]
format = "text"                 # text | json
detail = "lines"                # exists | count | files | lines
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
sort = "path asc"
path_style = "relative"         # relative | absolute

[escape]
max_level = 3

# Comment syntax, see "Comment syntax" below. Built-in languages can be overridden, new ones added.
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

MCP client registration, e.g. `.mcp.json` of Claude Code:

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

| Key             | Type             | Default  | Read-only | Meaning                                                      |
|-----------------|------------------|----------|-----------|--------------------------------------------------------------|
| `allowed_roots` | list of paths    | `[]`     | yes       | the server never reads outside of them; empty — the start directory |
| `default_dirs`  | list of paths    | `["."]`  | no        | working directories of a request without `dirs`              |
| `timeout`       | duration         | `"30s"`  | no        | max time of one search, units `ms`, `s`, `m`                 |
| `threads`       | number           | `0`      | no        | search threads, `0` — number of CPU cores                    |
| `allow_update`  | bool             | `true`   | yes       | `false` disables `update_config`                             |

Paths

- `~` is expanded to the home directory
- relative paths are resolved against the directory the server is started in
- `allowed_roots` are canonicalized at start (symlinks resolved); a root which doesn't exist is an error

A request working directory must be inside an allowed root after canonicalization, otherwise the request fails with
`dir_not_allowed`.

### traversal

Can also be set per request in the `traversal` field.

| Key               | Type              | Default  | Meaning                                                              |
|-------------------|-------------------|----------|----------------------------------------------------------------------|
| `min_depth`       | number            | `0`      | skip entries closer to the working directory; its direct children are depth 1 |
| `max_depth`       | number            | not set  | don't descend deeper; not set — unlimited                            |
| `follow_symlinks` | bool              | `false`  | descend into symlinked directories, read symlinked files             |
| `hidden`          | bool              | `false`  | include entries starting with `.`                                    |
| `ignore_files`    | bool              | `true`   | respect `.gitignore`, `.ignore`, `.git/info/exclude`, global git excludes |
| `exclude`         | list of wildcards | `[]`     | paths which are never walked, matched like `path ~`                  |
| `binary`          | string            | `"skip"` | binary files: `skip` — content conditions are false, `search` — search them as text, `list` — content conditions are false, but files are still returned by metadata conditions |
| `max_file_size`   | size              | `"10M"`  | bigger files are not searched by content                             |

Notes

- `exclude` prunes directories, they are not even read, so it is much faster than `not path ~ ...` in the query.
  Wildcards are relative to the working directory: `**/target/**` excludes every `target` directory, `target/**` only
  the top level one.
- With `follow_symlinks` loops are detected and skipped, and a symlink which points outside of the allowed roots is
  skipped with a `permission_denied` warning.
- A file is binary if its first 8K contain a zero byte.
- `.git` is hidden, so it is skipped by default. With `hidden = true` add `**/.git/**` to `exclude` if you don't need
  it.

### output

Defaults of the `output` field of the request. The meaning of every key is in [OUTPUT.md](OUTPUT.md#options).

| Key                    | Type        | Default        |
|------------------------|-------------|----------------|
| `format`               | string      | `"text"`       |
| `detail`               | string      | `"lines"`      |
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
| `sort`                 | string      | `"path asc"`   |
| `path_style`           | string      | `"relative"`   |

`markers` is configuration only, a request can't change it. Choose characters which don't appear in your files, e.g.
`["«", "»"]` or `["[[", "]]"]`.

### escape

| Key         | Type   | Default | Meaning                                                                   |
|-------------|--------|---------|---------------------------------------------------------------------------|
| `max_level` | number | `3`     | the upper level of `/e(style, any)` and the max level a query may ask for |

Every level multiplies the number of variants to match, keep it small.

### Comment syntax

Used by the `/c` modifier (see [QUERY.md](QUERY.md#comments)). A language is chosen by the file extension.

| Key                     | Type                      | Meaning                                                       |
|-------------------------|---------------------------|---------------------------------------------------------------|
| `extensions`            | list of strings           | file extensions without the dot; `Dockerfile` style names without extension are written as the full name |
| `line_comments`         | list of strings           | start a comment until the end of the line                     |
| `block_comments`        | list of `[start, end]`    | start and end of a block comment                              |
| `nested_block_comments` | bool                      | block comments can be nested (Rust, Haskell)                  |
| `strings`               | list of strings           | string delimiters; comment markers inside strings are ignored |

Built-in languages

| Language    | Extensions                                                            | Line          | Block              |
|-------------|-----------------------------------------------------------------------|---------------|--------------------|
| `c_like`    | `c`, `h`, `cpp`, `hpp`, `cc`, `cs`, `java`, `kt`, `kts`, `scala`, `go`, `swift`, `dart`, `groovy`, `gradle` | `//` | `/* */` |
| `js`        | `js`, `jsx`, `mjs`, `cjs`, `ts`, `tsx`                                | `//`          | `/* */`            |
| `rust`      | `rs`                                                                  | `//`          | `/* */`, nested    |
| `css`       | `css`, `scss`, `less`                                                 | `//` (not css) | `/* */`           |
| `hash`      | `py`, `rb`, `sh`, `bash`, `zsh`, `pl`, `r`, `yml`, `yaml`, `toml`, `conf`, `ini`, `properties`, `env`, `Dockerfile`, `Makefile` | `#` | — |
| `sql`       | `sql`                                                                 | `--`          | `/* */`            |
| `lua`       | `lua`                                                                 | `--`          | `--[[ ]]`          |
| `haskell`   | `hs`                                                                  | `--`          | `{- -}`, nested    |
| `markup`    | `html`, `htm`, `xml`, `svg`, `vue`, `md`                              | —             | `<!-- -->`         |
| `lisp`      | `clj`, `cljs`, `el`, `lisp`, `scm`                                    | `;`           | —                  |

`ini` also has `;` comments, `properties` also `!`.

Override or add a language:

```toml
# PHP: // # and /* */
[languages.php]
extensions = ["php"]
line_comments = ["//", "#"]
block_comments = [["/*", "*/"]]
strings = ['"', "'"]

# treat .tpl files as HTML
[languages.markup]
extensions = ["html", "htm", "xml", "svg", "vue", "md", "tpl"]
block_comments = [["<!--", "-->"]]
```

A language section with the same name as a built-in one replaces it completely. An extension listed in two languages
is an error at start.

## get_config

Returns the effective configuration: all values after merging built-in defaults, the config file, command line and
`update_config` calls.

Request — all fields are optional:

```json
{ "section": "traversal" }
```

| Field     | Meaning                                                               |
|-----------|-----------------------------------------------------------------------|
| `section` | `server`, `traversal`, `output`, `escape`, `languages`; not set — all |
| `sources` | `true` — add where each value comes from                              |

Response:

```json
{
  "traversal": {
    "min_depth": 0,
    "follow_symlinks": false,
    "hidden": false,
    "ignore_files": true,
    "exclude": ["**/target/**", "**/node_modules/**"],
    "binary": "skip",
    "max_file_size": "10M"
  },
  "read_only": ["server.allowed_roots", "server.allow_update"]
}
```

With `"sources": true`:

```json
{
  "traversal": {
    "exclude": { "value": ["**/target/**", "**/node_modules/**"], "source": "file:/home/me/.config/easy-peasy-mcp/config.toml" },
    "hidden": { "value": false, "source": "default" },
    "max_file_size": { "value": "50M", "source": "update" }
  }
}
```

Sources: `default`, `file:<path>`, `cli`, `update`.

An agent should call `get_config` once at the start to learn the allowed roots and limits instead of guessing.

## update_config

Changes the configuration in memory until the server stops. The config file is never written.

Request:

```json
{
  "set": { "output": { "max_files": 30, "context": 2 }, "traversal": { "exclude": ["**/target/**"] } },
  "reset": ["output.format"]
}
```

| Field   | Meaning                                                              |
|---------|----------------------------------------------------------------------|
| `set`   | values to change, nested like the config file                        |
| `reset` | keys to return to the value from the file / command line / default; `"all"` resets everything |

- lists are replaced, not merged: to add an exclude, send the whole new list
- the update is atomic: if any value is invalid, nothing is changed
- `reset` is applied before `set`

Response — the changed keys with the old and new values:

```json
{
  "changed": {
    "output.max_files": { "old": 100, "new": 30 },
    "output.context": { "old": 0, "new": 2 },
    "traversal.exclude": { "old": [], "new": ["**/target/**"] },
    "output.format": { "old": "json", "new": "text" }
  }
}
```

Errors:

| Code             | Example                                             |
|------------------|-----------------------------------------------------|
| `read_only`      | `{ "set": { "server": { "allowed_roots": ["/"] } } }` |
| `update_disabled`| any call with `allow_update = false`                |
| `unknown_key`    | `{ "set": { "output": { "max_file": 10 } } }` — hint: did you mean `max_files`? |
| `invalid_value`  | `{ "set": { "output": { "detail": "all" } } }` — hint: exists, count, files, lines |

## Security

- The server only reads. There is no command which writes, moves or deletes files, and `update_config` never writes
  the config file.
- `allowed_roots` can't be changed by the agent: neither `update_config` nor a request can widen them.
- Every working directory, every symlink target and every path from a cursor is canonicalized and checked against the
  allowed roots.
- `..` in `dirs` is allowed as long as the result is inside an allowed root.
- Files the server process can't read are skipped with a warning; the server never tries to change permissions.
- `timeout`, `max_file_size` and the output limits protect against huge results and slow regexes on big trees. The
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

### Several roots, read-only configuration

The agent can search the code and the logs but can't change the configuration:

```toml
[server]
allowed_roots = ["~/projects", "/var/log/my-app"]
default_dirs = ["~/projects"]
timeout = "60s"
allow_update = false

[traversal]
max_file_size = "200M"          # logs are big
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

### Tune the session from the agent

The agent wants more context for the rest of the session, and to search inside `node_modules` once:

```json
{ "set": { "output": { "context": 3 } } }
```

```json
{
  "dirs": ["node_modules/some-lib"],
  "query": "text has 'deprecated' /i",
  "traversal": { "ignore_files": false }
}
```

The second one changes `ignore_files` for this request only.
