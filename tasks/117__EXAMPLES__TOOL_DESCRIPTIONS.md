# Examples for task 117

Startup examples (AGENTS.md) with JSON-RPC exchanges: the server instructions, the tool descriptions, the input
schemas and the annotations are the texts of [TOOLS.md](../TOOLS.md#tools). `*` in a string of a `json receive` block
matches any characters, so the `*` of the descriptions match themselves too.

## E117.1 The answer to initialize

The capabilities, the server name and the instructions of TOOLS.md#transport.

Fixture: `mvp`

```text command
easy-peasy-mcp
```

```json receive
{"jsonrpc": "2.0", "id": 1, "result": {"capabilities": {"tools": {"listChanged": false}}, "serverInfo": {"name": "easy-peasy-mcp", "version": "*"}, "instructions": "File search: find + grep in one query. Call get_config first to learn allowed roots and defaults. Spec version 1."}}
```

## E117.2 The tools

The descriptions, input schemas and annotations of TOOLS.md#tools.

Fixture: `mvp`

```text command
easy-peasy-mcp
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "find",
        "description": "Search files by name, path, metadata and content in one query (find + grep in one). Read-only. Spec version 1.\nCall get_config once first to learn the allowed roots and defaults.\n\nQUERY: conditions joined with not, and, xor, or (this precedence) and brackets. Empty query = all files.\n  name ~ '*.rs'                  wildcard on the file name: * ? ** [abc] [!abc] {a,b}\n  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator\n  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)\n  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it\n  size > 1M  modified > -7d  created  accessed  type = dir  empty\n  lang = rust   kind in (source, test)   git = changed|branch|staged|untracked\n  name ~~ 'cfgldr'               fuzzy file name (also path ~~)\n  line(text has 'a' and not text has 'b')    conditions on the same line; len > 120 works inside line()\n  near(text has 'lock', text has 'await', 3[, ordered][, closest|first])   lines within 3 of each other\nModifiers after a text value: /i ignore case, /w whole word, /m multiline regex, /c skip comments,\n  /n any naming style ('parse query' = parseQuery, parse_query, PARSE_QUERY ...),\n  /e(backslash|quote_doubling|xml|url, 0..2|any) also match the escaped text (JSON inside JSON etc.)\nLabels: text has 'x' @name — the output shows which condition matched; (... or ...) @name labels a bracket group.\nStrings: '...' or \"...\" with \\ escapes; `...` raw, best for regex.\nDates: 2026-10-01, -30m -2h -7d -2w. Sizes: 512 10K 5M 1G.\n\nOUTPUT: output.detail = exists | count | facets | files | lines (default); output.context = N lines around.\noutput.budget = N tokens: the server picks the richest detail which fits. By default files are sorted by relevance.\nText response with the default options: \"42:\" matched line, \"41-\" context line, \"--\" gap, ⟦ids:text⟧ a match,\n[#1 @x] matched conditions, ⇥N removed indentation; empty or huge results come with a funnel and hints how to fix\nthe query. output.omit = ['legend', 'line_numbers', ...] or 'all' leaves those parts out.\nIf the response has next_cursor, pass it back as output.cursor with the same request to get the next page.\nResponses with count detail or higher have a result id (r12), unless the result cache is off: within = \"r12\"\nsearches only its files, since = \"r12\" returns only the changes.\nqueries = [{label, query}, ...] runs several queries in one call.\noutput.file = 'x.txt' writes the response to a file under the output dir and returns its path; for big results.\n\nEXAMPLES\n  name ~ '*.py' and text has 'import requests'\n  not text has 'license' /i                                   files without a match\n  name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')\n  text has `\"role\": \"admin\"` /e(backslash, any)               also inside stringified JSON\n  kind = source and name has 'config loader' /n               a file by a vague name\n  git = changed and text has 'dbg!('                          leftovers in files with uncommitted changes",
        "inputSchema": {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "version": {
              "type": "integer",
              "minimum": 1,
              "description": "Spec version. Omit to use the current one."
            },
            "dirs": {
              "type": "array",
              "items": {
                "type": "string"
              },
              "minItems": 1,
              "description": "Working directories inside the allowed roots. Default: server.default_dirs."
            },
            "query": {
              "type": "string",
              "description": "Search query, see the tool description. Empty: all files."
            },
            "queries": {
              "type": "array",
              "minItems": 1,
              "description": "Several queries in one call instead of `query`.",
              "items": {
                "type": "object",
                "additionalProperties": false,
                "required": [
                  "query"
                ],
                "properties": {
                  "label": {
                    "type": "string"
                  },
                  "query": {
                    "type": "string"
                  },
                  "dirs": {
                    "$ref": "#/properties/dirs"
                  },
                  "timeout": {
                    "$ref": "#/properties/timeout"
                  },
                  "traversal": {
                    "$ref": "#/properties/traversal"
                  },
                  "output": {
                    "$ref": "#/properties/output"
                  }
                }
              }
            },
            "within": {
              "type": "string",
              "description": "Result id: search only the files of that result."
            },
            "since": {
              "type": "string",
              "description": "Result id: return only the differences to that result."
            },
            "timeout": {
              "type": "string",
              "pattern": "^[0-9]+(ms|s|m)$",
              "description": "Max search time, e.g. '10s'. '0s': no limit."
            },
            "traversal": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "min_depth": {
                  "type": "integer",
                  "minimum": 0
                },
                "max_depth": {
                  "type": "integer",
                  "minimum": 0
                },
                "follow_symlinks": {
                  "type": "boolean"
                },
                "hidden": {
                  "type": "boolean"
                },
                "ignore_files": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "git_excludes": {
                  "type": "boolean"
                },
                "ignore_paths": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "exclude": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "path_case": {
                  "enum": [
                    "native",
                    "sensitive",
                    "insensitive"
                  ]
                },
                "binary": {
                  "enum": [
                    "skip",
                    "search",
                    "exclude"
                  ]
                },
                "max_file_size": {
                  "type": "string"
                },
                "encoding": {
                  "type": "string"
                },
                "fallback_encoding": {
                  "type": "string"
                },
                "archives": {
                  "enum": [
                    "none",
                    "compressed",
                    "all"
                  ]
                },
                "max_archive_size": {
                  "type": "string"
                },
                "skip_generated": {
                  "type": "boolean"
                }
              }
            },
            "output": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "format": {
                  "enum": [
                    "text",
                    "json"
                  ]
                },
                "detail": {
                  "enum": [
                    "exists",
                    "count",
                    "facets",
                    "files",
                    "lines"
                  ]
                },
                "budget": {
                  "type": "integer",
                  "minimum": 100,
                  "description": "Max response size in tokens."
                },
                "chars_per_token": {
                  "type": "number",
                  "exclusiveMinimum": 0
                },
                "funnel": {
                  "enum": [
                    "auto",
                    "always",
                    "never"
                  ]
                },
                "facets": {
                  "type": "array",
                  "items": {
                    "enum": [
                      "dir",
                      "ext",
                      "lang",
                      "kind",
                      "git",
                      "condition"
                    ]
                  }
                },
                "facet_depth": {
                  "type": "integer",
                  "minimum": 1
                },
                "facet_limit": {
                  "type": "integer",
                  "minimum": 1
                },
                "meta": {
                  "type": "boolean"
                },
                "context": {
                  "type": "integer",
                  "minimum": 0
                },
                "before": {
                  "type": "integer",
                  "minimum": 0
                },
                "after": {
                  "type": "integer",
                  "minimum": 0
                },
                "path_fragments": {
                  "type": "boolean"
                },
                "highlight": {
                  "enum": [
                    "auto",
                    "none",
                    "columns",
                    "markers"
                  ]
                },
                "markers": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  },
                  "minItems": 2,
                  "maxItems": 2
                },
                "max_files": {
                  "type": "integer",
                  "minimum": 1
                },
                "max_lines": {
                  "type": "integer",
                  "minimum": 1
                },
                "max_matches": {
                  "type": "integer",
                  "minimum": 1
                },
                "max_matches_per_file": {
                  "type": "integer",
                  "minimum": 1
                },
                "max_line_length": {
                  "type": "integer",
                  "minimum": 20
                },
                "max_output_per_file": {
                  "type": "integer",
                  "minimum": 100
                },
                "max_output": {
                  "type": "integer",
                  "minimum": 100
                },
                "dedent": {
                  "type": "boolean"
                },
                "blank_context": {
                  "type": "boolean"
                },
                "group_dirs": {
                  "type": "boolean"
                },
                "group_repeated": {
                  "type": "integer",
                  "minimum": 0
                },
                "sort": {
                  "type": "string",
                  "pattern": "^(relevance|(path|modified|size|matches) (asc|desc))$"
                },
                "path_style": {
                  "enum": [
                    "relative",
                    "absolute"
                  ]
                },
                "omit": {
                  "anyOf": [
                    {
                      "type": "array",
                      "items": {
                        "enum": [
                          "counts",
                          "result",
                          "legend",
                          "funnel",
                          "hints",
                          "conditions",
                          "file_counts",
                          "match_ids",
                          "groups",
                          "line_numbers",
                          "gaps",
                          "indent",
                          "warnings"
                        ]
                      }
                    },
                    {
                      "enum": [
                        "all"
                      ]
                    }
                  ],
                  "description": "Parts of the response to leave out, or \"all\"."
                },
                "cursor": {
                  "type": "string"
                },
                "file": {
                  "type": [
                    "string",
                    "boolean"
                  ],
                  "description": "Write the response to this file under server.output_dir and return its path; true: a name like r12.txt."
                }
              }
            },
            "escape": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "max_level": {
                  "type": "integer",
                  "minimum": 0
                }
              }
            },
            "query_options": {
              "type": "object",
              "additionalProperties": false,
              "description": "The [query] section of the configuration.",
              "properties": {
                "fuzzy_min_score": {
                  "type": "number",
                  "minimum": 0,
                  "maximum": 1
                }
              }
            },
            "kinds": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "generated_markers": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "generated_marker_lines": {
                  "type": "integer",
                  "minimum": 1
                },
                "generated": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "test": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "doc": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                },
                "config": {
                  "type": "array",
                  "items": {
                    "type": "string"
                  }
                }
              }
            },
            "git": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "base": {
                  "type": "string"
                }
              }
            },
            "languages": {
              "type": "object",
              "description": "Languages, replace the configured languages with the same name.",
              "additionalProperties": {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                  "extensions": {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  "names": {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  "shebangs": {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  "line_comments": {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  "block_comments": {
                    "type": "array",
                    "items": {
                      "type": "array",
                      "items": {
                        "type": "string"
                      },
                      "minItems": 2,
                      "maxItems": 2
                    }
                  },
                  "nested_block_comments": {
                    "type": "boolean"
                  },
                  "strings": {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  "source": {
                    "type": "boolean"
                  }
                }
              }
            }
          }
        },
        "annotations": {
          "readOnlyHint": true,
          "openWorldHint": false
        }
      },
      {
        "name": "get_config",
        "description": "Returns the effective configuration of the search server as TOML: allowed roots, default directories, traversal and\noutput defaults, limits, supported spec versions. Read-only. Call it once before the first find.",
        "inputSchema": {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "section": {
              "enum": [
                "server",
                "traversal",
                "output",
                "escape",
                "query",
                "kinds",
                "git",
                "languages"
              ]
            },
            "sources": {
              "type": "boolean",
              "description": "Add where each value comes from."
            }
          }
        },
        "annotations": {
          "readOnlyHint": true,
          "openWorldHint": false
        }
      },
      {
        "name": "update_config",
        "description": "Changes the search server configuration in memory until the server stops: any value except the read-only ones of\nserver (allowed_roots, allow_update, allow_write, output_dir, max_output_file_size). `set` is TOML like the config file, e.g.\n\"output.context = 2\" or \"[output]\\ncontext = 2\\nmax_files = 30\". Lists are replaced, not merged. Atomic: if any value\nis invalid, nothing changes. To change a value for one search only, pass it in the find request instead.",
        "inputSchema": {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "set": {
              "type": "string",
              "description": "TOML with the values to change, like the config file, e.g. 'output.context = 2'."
            },
            "reset": {
              "type": "array",
              "items": {
                "type": "string"
              },
              "description": "Keys like 'output.context', or ['all']."
            }
          }
        },
        "annotations": {
          "readOnlyHint": false,
          "destructiveHint": false,
          "idempotentHint": true,
          "openWorldHint": false
        }
      }
    ]
  }
}
```

## E117.3 No update_config when updates are off

`update_config` is not listed when `server.allow_update = false`.

Fixture: `mvp`

```text command
easy-peasy-mcp --no-update
```

```json send
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

```json receive
{"jsonrpc": "2.0", "id": 2, "result": {"tools": [{"name": "find", "annotations": {"readOnlyHint": true, "openWorldHint": false}}, {"name": "get_config", "annotations": {"readOnlyHint": true, "openWorldHint": false}}]}}
```
