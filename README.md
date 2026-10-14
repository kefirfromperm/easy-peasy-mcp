# Easy-Peasy MCP Server
The main goal is to allow to an agent to find a file or files with complex criteria.

It covers `find` and `grep` in one query and adds what they can't do: logical operators over any conditions, several
conditions on the same line, proximity, matching escaped text, ignoring comments, and telling which condition matched.

```
name ~ '*.rs' and modified > -7d and not path ~ '**/tests/**'
and line(text has 'unwrap' /i and not text starts '//')
```

I will use RUST

Candidate crates: `rmcp` (MCP server), `ignore` (parallel directory walking, `.gitignore` style ignore files),
`grep-regex` / `grep-searcher` (fast content search, binary detection), `globset` (wildcards), `pest` or `chumsky`
(query parser), `encoding_rs` (encodings), `flate2` / `zip` / `tar` and friends (archives), `gix` (git status).

It is built to help an agent find the exact file and place with as few tokens and round trips as possible:
identifier-aware and fuzzy matching, relevance sorting, a funnel that explains empty results, a token budget, facets,
compact output, refining previous results and batches of queries.

It runs locally over stdio and is built natively for Linux, macOS and Windows.

## Documentation

- [INSTALL.md](INSTALL.md) — install scripts, building from source, registration in an MCP client
- [QUERY.md](QUERY.md) — the query language: targets (including `lang`, `kind`, `git`), operators, scopes, modifiers,
  escaping, proximity, identifiers and fuzzy names, grammar, errors, recipes
- [OUTPUT.md](OUTPUT.md) — output options, response format, condition ids, fragments, highlight, limits, pagination,
  relevance, funnel, budget, facets, compact output, result ids, batches, output to a file
- [CONFIG.md](CONFIG.md) — config file, command line, allowed roots, defaults, ignore files, encodings, archives,
  kinds, git, languages and comment syntax, platforms, `get_config` and `update_config`
- [TOOLS.md](TOOLS.md) — MCP side: stdio transport, versioning, tool descriptions and input schemas, write tools,
  results, errors, timeout, progress, cancellation

## Commands

| Command         | Meaning                                                    | Details                           |
|-----------------|------------------------------------------------------------|-----------------------------------|
| `find`          | search files                                               | [TOOLS.md](TOOLS.md#find)          |
| `get_config`    | the effective configuration                                | [TOOLS.md](TOOLS.md#get_config)    |
| `update_config` | change the configuration until the server stops            | [TOOLS.md](TOOLS.md#update_config) |
| `delete`        | move found files to the trash, or delete them; only with `server.allow_write = true` | [TOOLS.md](TOOLS.md#delete) |
| `rename`        | rename or move found files by a template; only with `server.allow_write = true` | [TOOLS.md](TOOLS.md#rename) |

## Request

The `find` request contains fields, all of them optional:

| Field       | Meaning                                                                      | Details                       |
|-------------|------------------------------------------------------------------------------|-------------------------------|
| `version`   | spec version to use; not set — the current one                               | [TOOLS.md](TOOLS.md#versioning) |
| `dirs`      | one or more working directories, inside the allowed roots                    | [CONFIG.md](CONFIG.md#server) |
| `query`     | search criteria; empty — all files                                           | [QUERY.md](QUERY.md)          |
| `queries`   | several queries in one call, instead of `query`                              | [OUTPUT.md](OUTPUT.md#batch)  |
| `within`    | result id: search only the files of a previous result                        | [OUTPUT.md](OUTPUT.md#result-ids) |
| `since`     | result id: return only the differences to a previous result                  | [OUTPUT.md](OUTPUT.md#result-ids) |
| `timeout`   | max search time, e.g. `10s`                                                  | [TOOLS.md](TOOLS.md#timeout)  |
| `traversal` | depth, symlinks, hidden files, ignore files, excludes, binary files, max file size, encoding, archives | [CONFIG.md](CONFIG.md#traversal) |
| `output`    | format, detail level, budget, funnel, facets, context, limits, sorting, compaction, highlight, cursor, output to a file | [OUTPUT.md](OUTPUT.md#options) |
| `escape`    | max escape level                                                             | [CONFIG.md](CONFIG.md#escape) |
| `query_options` | fuzzy match threshold                                                    | [CONFIG.md](CONFIG.md#query)  |
| `kinds`     | rules of `kind`: generated, test, doc, config                                | [CONFIG.md](CONFIG.md#kinds)  |
| `git`       | base of `git = branch`                                                       | [CONFIG.md](CONFIG.md#git)    |
| `languages` | languages: extensions, names, shebangs, comment syntax                       | [CONFIG.md](CONFIG.md#languages) |

Missing fields are taken from the configuration. A request can replace any configuration value except the `server`
section; its `dirs` and `timeout` replace `server.default_dirs` and `server.timeout`, see
[CONFIG.md](CONFIG.md#where-the-values-come-from).

```json
{
  "dirs": ["."],
  "query": "name ~ '*.rs' and line(text has 'unwrap(' /c @unwrap or text has 'expect(' /c @expect)",
  "traversal": { "exclude": ["**/target/**"] },
  "output": { "context": 1, "max_files": 20 }
}
```

```
files: 2  lines: 3  matches: 3  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' /c or text has 'expect(' /c)
@unwrap text has 'unwrap(' /c
@expect text has 'expect(' /c

src/
config.rs  [#1 #2]  lines: 2  matches: 2
⇥4
41-let raw = fs::read_to_string(path)?;
42:let cfg: Config = toml::from_str(&raw).⟦#2,@unwrap:unwrap(⟧);
43:let port = cfg.port.⟦#2,@expect:expect(⟧"port is required");
44-Ok(cfg)
main.rs  [#1 #2]  lines: 1  matches: 1
⇥4
17-let args = Args::parse();
18:let cfg = config::load(&args.config).⟦#2,@unwrap:unwrap(⟧);
19-run(cfg)
```

## Encoding

By default files are decoded by their BOM (UTF-8, UTF-16, UTF-32). Without a BOM, UTF-16 is recognized by its zero
bytes, other files are read as UTF-8 with an optional fallback encoding, see [CONFIG.md](CONFIG.md#encoding).
