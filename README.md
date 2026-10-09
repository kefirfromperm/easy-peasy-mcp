# Easy-Peasy MCP Server
The main goal is to allow to an agent to find a file or files with complex criteria.

It covers `find` and `grep` in one query and adds what they can't do: logical operators over any conditions, several
conditions on the same line, proximity, matching escaped text, ignoring comments, and telling which condition matched.

```
name ~ '*.rs' and modified > -7d and not path ~ '**/tests/**'
and line(text has 'unwrap' /i and not text starts '//')
```

I will use RUST

Candidate crates: `ignore` (parallel directory walking, `.gitignore` support), `grep-regex` / `grep-searcher` (fast
content search, binary detection), `globset` (wildcards), `pest` or `chumsky` (query parser).

## Documentation

- [QUERY.md](QUERY.md) — the query language: targets, operators, scopes, modifiers, escaping, proximity, grammar,
  errors, recipes
- [OUTPUT.md](OUTPUT.md) — output options, response format, condition ids, fragments, highlight, limits, pagination
- [CONFIG.md](CONFIG.md) — config file, command line, allowed roots, defaults, comment syntax, `get_config` and
  `update_config`

## Commands

| Command         | Meaning                                                    |
|-----------------|------------------------------------------------------------|
| `find`          | search files                                               |
| `get_config`    | the effective configuration                                |
| `update_config` | change the configuration until the server stops            |

## Request

The `find` request contains fields, all of them optional:

| Field       | Meaning                                                                      | Details                       |
|-------------|------------------------------------------------------------------------------|-------------------------------|
| `dirs`      | one or more working directories, inside the allowed roots                    | [CONFIG.md](CONFIG.md#server) |
| `query`     | search criteria; empty — all files                                           | [QUERY.md](QUERY.md)          |
| `traversal` | depth, symlinks, hidden files, ignore files, excludes, binary files, max file size | [CONFIG.md](CONFIG.md#traversal) |
| `output`    | format, detail level, context, limits, sorting, highlight, cursor           | [OUTPUT.md](OUTPUT.md#options) |

Missing fields are taken from the configuration.

```json
{
  "dirs": ["."],
  "query": "name ~ '*.rs' and line(text has 'unwrap(' /c @unwrap or text has 'expect(' /c @expect)",
  "traversal": { "exclude": ["**/target/**"] },
  "output": { "context": 1, "max_files": 20 }
}
```

```
files: 2  lines: 3  matches: 3
#1 name ~ '*.rs'
#2 line(text has 'unwrap(' /c or text has 'expect(' /c)
@unwrap text has 'unwrap(' /c
@expect text has 'expect(' /c

src/config.rs  [#1 #2]  lines: 2  matches: 2
41-    let raw = fs::read_to_string(path)?;
42:    let cfg: Config = toml::from_str(&raw).⟦#2,@unwrap:unwrap(⟧);
43:    let port = cfg.port.⟦#2,@expect:expect(⟧"port is required");
44-    Ok(cfg)

src/main.rs  [#1 #2]  lines: 1  matches: 1
17-    let args = Args::parse();
18:    let cfg = config::load(&args.config).⟦#2,@unwrap:unwrap(⟧);
19-    run(cfg)
```

## Encoding

Files are read as UTF-8. Invalid byte sequences are replaced (lossy) and do not stop the search.
