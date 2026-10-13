# Examples for task 58

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### archives

```text tree
logs/0-plain.log           2026-10-01 10:00
logs/a.7z                  2026-10-01 10:00
logs/a.jar                 2026-10-01 10:00
logs/a.tar                 2026-10-01 10:00
logs/a.tar.bz2             2026-10-01 10:00
logs/a.tar.gz              2026-10-01 10:00
logs/a.tar.xz              2026-10-01 10:00
logs/a.tar.zst             2026-10-01 10:00
logs/a.zip                 2026-10-01 10:00
logs/broken.zip            2026-10-01 10:00
```

File contents are in `tests/fixtures/archives/`. `logs/0-plain.log` has `started` and `plain request req-7f3a9c done`;
every archive `logs/a.<format>` has one member `inner/<format>.log` (`.` of the format written as `-`) with
`started` and `<format> request req-7f3a9c done`, modified 2026-10-01 10:00 UTC; `a.zip` also has
`inner/nested.zip`, an archive with the member `deep.log`; `broken.zip` starts like a zip archive but isn't one.
Times are UTC.

## E58.1 Archives are not opened by default

Fixture: `archives`

```json request
{"query": "name ~ '*.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

logs/
0-plain.log  [#1]
```

## E58.2 The members of every archive in a request

A member has the path `<archive>!/<member>`. The nested archive is not opened, so `deep.log` is not found.

Fixture: `archives`

```json request
{"query": "name ~ '*.log'", "traversal": {"archives": "all"}}
```

```text response
files: 9  lines: 0  matches: 0  result: r1
#1 name ~ '*.log'

logs/
0-plain.log  [#1]
logs/a.7z!/inner/
7z.log  [#1]
logs/a.jar!/inner/
jar.log  [#1]
logs/a.tar!/inner/
tar.log  [#1]
logs/a.tar.bz2!/inner/
tar-bz2.log  [#1]
logs/a.tar.gz!/inner/
tar-gz.log  [#1]
logs/a.tar.xz!/inner/
tar-xz.log  [#1]
logs/a.tar.zst!/inner/
tar-zst.log  [#1]
logs/a.zip!/inner/
zip.log  [#1]

warning: archive_error (1): logs/broken.zip
```

## E58.3 The content of the members

Only inside archives. The member `inner/nested.zip` is binary.

Fixture: `archives`

```json request
{"query": "path ~ '**/*!/**' and text has 'req-7f3a9c'", "traversal": {"archives": "all"}}
```

```text response
files: 8  lines: 8  matches: 8  result: r1
#1 path ~ '**/*!/**'
#2 text has 'req-7f3a9c'

logs/a.7z!/inner/
7z.log  [#1 #2]  lines: 1  matches: 1
2:7z request ⟦#2:req-7f3a9c⟧ done
logs/a.jar!/inner/
jar.log  [#1 #2]  lines: 1  matches: 1
2:jar request ⟦#2:req-7f3a9c⟧ done
logs/a.tar!/inner/
tar.log  [#1 #2]  lines: 1  matches: 1
2:tar request ⟦#2:req-7f3a9c⟧ done
logs/a.tar.bz2!/inner/
tar-bz2.log  [#1 #2]  lines: 1  matches: 1
2:tar.bz2 request ⟦#2:req-7f3a9c⟧ done
logs/a.tar.gz!/inner/
tar-gz.log  [#1 #2]  lines: 1  matches: 1
2:tar.gz request ⟦#2:req-7f3a9c⟧ done
logs/a.tar.xz!/inner/
tar-xz.log  [#1 #2]  lines: 1  matches: 1
2:tar.xz request ⟦#2:req-7f3a9c⟧ done
logs/a.tar.zst!/inner/
tar-zst.log  [#1 #2]  lines: 1  matches: 1
2:tar.zst request ⟦#2:req-7f3a9c⟧ done
logs/a.zip!/inner/
zip.log  [#1 #2]  lines: 1  matches: 1
2:zip request ⟦#2:req-7f3a9c⟧ done

warning: binary_skipped (1): logs/a.zip!/inner/nested.zip
warning: archive_error (1): logs/broken.zip
```

## E58.4 The archive file itself is an entry

Fixture: `archives`

```json request
{"query": "name = 'a.zip' or name = 'nested.zip'", "traversal": {"archives": "all"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'a.zip'
#2 name = 'nested.zip'

logs/
a.zip  [#1]
logs/a.zip!/inner/
nested.zip  [#2]

warning: archive_error (1): logs/broken.zip
```

## E58.5 `max_file_size` applies to every member

The members of 37 bytes and more are not searched by content.

Fixture: `archives`

```json request
{"query": "path ~ '**/*!/**' and text has 'req-7f3a9c'", "traversal": {"archives": "all", "max_file_size": "36"}}
```

```text response
files: 4  lines: 4  matches: 4  result: r1
#1 path ~ '**/*!/**'
#2 text has 'req-7f3a9c'

logs/a.7z!/inner/
7z.log  [#1 #2]  lines: 1  matches: 1
2:7z request ⟦#2:req-7f3a9c⟧ done
logs/a.jar!/inner/
jar.log  [#1 #2]  lines: 1  matches: 1
2:jar request ⟦#2:req-7f3a9c⟧ done
logs/a.tar!/inner/
tar.log  [#1 #2]  lines: 1  matches: 1
2:tar request ⟦#2:req-7f3a9c⟧ done
logs/a.zip!/inner/
zip.log  [#1 #2]  lines: 1  matches: 1
2:zip request ⟦#2:req-7f3a9c⟧ done

warning: too_big (5): logs/a.tar.bz2!/inner/tar-bz2.log, logs/a.tar.gz!/inner/tar-gz.log, logs/a.tar.xz!/inner/tar-xz.log, … 2 more
warning: archive_error (1): logs/broken.zip
```

## E58.6 Archives for the session

Fixture: `archives`

1. `update_config`

```json request update_config
{"set": "traversal.archives = \"all\""}
```

```text response
[traversal]
archives = "all"    # was "none"
```

2. `find`

```json request
{"query": "text has 'jar request'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'jar request'

logs/a.jar!/inner/
jar.log  [#1]  lines: 1  matches: 1
2:⟦#1:jar request⟧ req-7f3a9c done

warning: binary_skipped (9): logs/a.7z, logs/a.jar, logs/a.tar, … 6 more
warning: archive_error (1): logs/broken.zip
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "all"    # update
max_archive_size = "100M"    # default
skip_generated = true    # default
```

## E58.7 An invalid value in a request

Fixture: `archives`

```json request
{"traversal": {"archives": "tar"}}
```

```text error
error[invalid_option]: 'traversal.archives' must be one of: none, compressed, all
```

## E58.8 An invalid value for `update_config`

Fixture: `archives`

```json request update_config
{"set": "traversal.archives = [\"all\"]"}
```

```text error
error[invalid_value]: invalid value for 'traversal.archives'
  hint: expected one of: none, compressed, all
```

## E58.9 From the config file

Fixture: `archives`

```toml conf/a.toml
[traversal]
archives = "all"
```

```text command
easy-peasy-mcp --config conf/a.toml
```

1. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update, server.allow_write, server.output_dir, server.max_output_file_size

[traversal]
min_depth = 0    # default
# max_depth =  (not set)    # default
follow_symlinks = false    # default
hidden = false    # default
ignore_files = ['.gitignore', '.ignore', '.aiignore']    # default
git_excludes = true    # default
ignore_paths = []    # default
exclude = []    # default
path_case = "native"    # default
binary = "skip"    # default
max_file_size = "10M"    # default
encoding = "auto"    # default
# fallback_encoding =  (not set)    # default
archives = "all"    # file: conf/a.toml
max_archive_size = "100M"    # default
skip_generated = true    # default
```

2. `find`

```json request
{"query": "name = 'zip.log'"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'zip.log'

logs/a.zip!/inner/
zip.log  [#1]

warning: archive_error (1): logs/broken.zip
```

## E58.10 An invalid value in the config file

Fixture: `archives`

```toml conf/a.toml
[traversal]
archives = "every"
```

```text command
easy-peasy-mcp --config conf/a.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/a.toml:2: invalid value for 'traversal.archives': expected one of: none, compressed, all
```
