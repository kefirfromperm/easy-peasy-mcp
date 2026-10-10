# Examples for task 57

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### compressed

```text tree
big/huge.log.gz            2026-10-01 10:00
logs/app.log.bz2           2026-10-01 10:00
logs/app.log.gz            2026-10-01 10:00
logs/app.log.lz4           2026-10-01 10:00
logs/app.log.xz            2026-10-01 10:00
logs/app.log.zst           2026-10-01 10:00
logs/backup.tgz            2026-10-01 10:00
logs/broken.gz             2026-10-01 10:00
```

File contents are in `tests/fixtures/compressed/`: every `logs/app.log.*` is the text `started` and
`<format> request req-7f3a9c done` compressed with its format; `big/huge.log.gz` is 63 bytes, decompressed a line of
2000 `x` and `huge request req-7f3a9c done`; `logs/backup.tgz` is a tar archive with one small file, gzipped;
`logs/broken.gz` starts like gzip but isn't. Times are UTC.

## E57.1 Compressed files are not opened by default

They are files like others, found by their names.

Fixture: `compressed`

1. `find`

```json request
{"query": "name ~ 'app.log.*'"}
```

```text response
files: 5  lines: 0  matches: 0  result: r1
#1 name ~ 'app.log.*'

logs/
app.log.bz2  [#1]
app.log.gz  [#1]
app.log.lz4  [#1]
app.log.xz  [#1]
app.log.zst  [#1]
```

2. `get_config`

```json request get_config
{ "section": "traversal" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[traversal]
min_depth = 0
# max_depth =  (not set)
follow_symlinks = false
hidden = false
ignore_files = ['.gitignore', '.ignore', '.aiignore']
git_excludes = true
ignore_paths = []
exclude = []
path_case = "native"
binary = "skip"
max_file_size = "10M"
encoding = "auto"
# fallback_encoding =  (not set)
archives = "none"
max_archive_size = "100M"
skip_generated = true
```

## E57.2 Compressed files in a request

Every format is searched as its decompressed content, with its own path and name. The content of `backup.tgz` is a
tar archive, which is binary; `broken.gz` can't be decompressed.

Fixture: `compressed`

```json request
{"query": "text has 'req-7f3a9c'", "traversal": {"archives": "compressed"}}
```

```text response
files: 6  lines: 6  matches: 6  result: r1
#1 text has 'req-7f3a9c'

big/
huge.log.gz  [#1]  lines: 1  matches: 1
2:huge request ⟦#1:req-7f3a9c⟧ done
logs/
app.log.bz2  [#1]  lines: 1  matches: 1
2:bz2 request ⟦#1:req-7f3a9c⟧ done
app.log.gz  [#1]  lines: 1  matches: 1
2:gz request ⟦#1:req-7f3a9c⟧ done
app.log.lz4  [#1]  lines: 1  matches: 1
2:lz4 request ⟦#1:req-7f3a9c⟧ done
app.log.xz  [#1]  lines: 1  matches: 1
2:xz request ⟦#1:req-7f3a9c⟧ done
app.log.zst  [#1]  lines: 1  matches: 1
2:zst request ⟦#1:req-7f3a9c⟧ done

warning: binary_skipped (1): logs/backup.tgz
warning: archive_error (1): logs/broken.gz
```

## E57.3 `max_file_size` applies to the decompressed content

Fixture: `compressed`

```json request
{"query": "text has 'req-7f3a9c'", "traversal": {"archives": "compressed", "max_file_size": "1K"}}
```

```text response
files: 5  lines: 5  matches: 5  result: r1
#1 text has 'req-7f3a9c'

logs/
app.log.bz2  [#1]  lines: 1  matches: 1
2:bz2 request ⟦#1:req-7f3a9c⟧ done
app.log.gz  [#1]  lines: 1  matches: 1
2:gz request ⟦#1:req-7f3a9c⟧ done
app.log.lz4  [#1]  lines: 1  matches: 1
2:lz4 request ⟦#1:req-7f3a9c⟧ done
app.log.xz  [#1]  lines: 1  matches: 1
2:xz request ⟦#1:req-7f3a9c⟧ done
app.log.zst  [#1]  lines: 1  matches: 1
2:zst request ⟦#1:req-7f3a9c⟧ done

warning: too_big (2): big/huge.log.gz, logs/backup.tgz
warning: archive_error (1): logs/broken.gz
```

## E57.4 Compressed files for the session

Fixture: `compressed`

1. `update_config`

```json request update_config
{"set": "traversal.archives = \"compressed\""}
```

```text response
[traversal]
archives = "compressed"    # was "none"
```

2. `find`

```json request
{"query": "text has 'gz request'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'gz request'

logs/
app.log.gz  [#1]  lines: 1  matches: 1
2:⟦#1:gz request⟧ req-7f3a9c done

warning: binary_skipped (1): logs/backup.tgz
warning: archive_error (1): logs/broken.gz
```

3. `get_config`

```json request get_config
{ "section": "traversal", "sources": true }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

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
archives = "compressed"    # update
max_archive_size = "100M"    # default
skip_generated = true    # default
```

## E57.5 An invalid value in a request

Fixture: `compressed`

```json request
{"traversal": {"archives": "zip"}}
```

```text error
error[invalid_option]: 'traversal.archives' must be one of: none, compressed, all
```

## E57.6 An invalid value for `update_config`

Fixture: `compressed`

```json request update_config
{"set": "traversal.archives = true"}
```

```text error
error[invalid_value]: invalid value for 'traversal.archives'
  hint: expected one of: none, compressed, all
```

## E57.7 From the config file

Fixture: `compressed`

```toml conf/a.toml
[traversal]
archives = "compressed"
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
# read-only: server.allowed_roots, server.allow_update

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
archives = "compressed"    # file: conf/a.toml
max_archive_size = "100M"    # default
skip_generated = true    # default
```

2. `find`

```json request
{"query": "name ~ '*.xz' and text has 'request'"}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name ~ '*.xz'
#2 text has 'request'

logs/
app.log.xz  [#1 #2]  lines: 1  matches: 1
2:xz ⟦#2:request⟧ req-7f3a9c done
```

## E57.8 An invalid value in the config file

Fixture: `compressed`

```toml conf/a.toml
[traversal]
archives = "zip"
```

```text command
easy-peasy-mcp --config conf/a.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/a.toml:2: invalid value for 'traversal.archives': expected one of: none, compressed, all
```
