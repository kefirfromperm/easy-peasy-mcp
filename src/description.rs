//! What the agent reads: the server instructions, the tool descriptions and input schemas (TOOLS.md#tools).

use serde_json::{Value, json};

/// The `instructions` of `initialize`.
pub const INSTRUCTIONS: &str = "File search: find + grep in one query. Call get_config first to learn allowed roots and \
defaults. Spec version 1.";

/// The description of the `find` tool: the cheat sheet of TOOLS.md#description for the implemented subset.
pub const FIND_DESCRIPTION: &str = "\
Search files by name, path, metadata and content in one query (find + grep in one). Read-only. Spec version 1.
Call get_config once first to learn the allowed roots and defaults.
\"dirs\": [\"src\", \"../api\"] sets the working directories (default server.default_dirs); paths are relative to
them, with the directory first when it is not \".\". \"timeout\": \"10s\" limits the search (default server.timeout, 30s):
the files found so far are returned with \"truncated: yes (timeout)\". \"traversal\": {\"min_depth\": 2, \"max_depth\": 3} skips the
entries directly in the working directory and doesn't descend deeper than 3 (defaults traversal.min_depth, max_depth);
\"follow_symlinks\": true follows symlinks (a symlinked file is then type file and link); \"hidden\": true includes hidden
entries.

QUERY: conditions on the name, path, type, size, dates, language, kind, git status and content joined with not, and, xor, or (this precedence) and brackets.
Empty query = all files.
  name = 'Cargo.toml'            exact file name; also: has (contains), starts, ends
  name ~ '*.rs'                  wildcard on the file name: * ? [abc] [a-z] [!abc] {a,b};
                                 [*] [?] [[] [{] match the character itself
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator; ** any dirs
  path starts 'docs/'            =, has, starts, ends, =~ (regex) work on paths and names too
  name ~~ 'cfgldr'               fuzzy, like fzf: the characters in order, case-insensitive; space separated terms
                                 must all match (path ~~ 'srv hndl'); weak matches (score below 0.5) are dropped
  size > 1M                      also: = != < <= >= and size in 10K..1M; units B K M G (1024)
  modified > -7d                 also: modified = 2026-10-01 (the whole day), modified in -7d..-1d;
                                 dates 2026-10-01, 2026-10-01T14:30, ...T14:30:00Z, ...+02:00 (local time without
                                 an offset), relative -30m -2h -7d -2w (units s m h d w)
  created > -1d                  creation date, the same forms; unknown where the file system has none
  accessed < -30d                last access date, the same forms
  lang = rust                    language by extension, file name (Dockerfile) or shebang; lang in (ts, js) style
                                 lists, lang != markdown, lang = unknown
  kind in (source, test)         source test config doc generated other; generated files (lock files, *.min.js,
                                 '@generated' markers) are skipped unless the query has a kind condition
  git = changed                  also: modified staged untracked conflicted clean; git in (modified, untracked);
                                 git = branch: changed, or changed by the commits since the base (main/master);
                                 false outside of a repository
  text has 'TODO'                content contains, /i ignore case; a value with '\n' matches across lines;
                                 also = starts ends on the whole content: text starts '#!/bin/bash';
                                 text =~ `fn \\w+_test` regex (Rust syntax), per line: ^ $ are line ends;
                                 with /m on the whole content: \\n matches a line break;
                                 binary files are not searched: content conditions are unknown for them
  line(text has 'a' and not text has 'b')    conditions on the same line; not line(...) = no line matches;
                                 len > 120 (characters of the line) works inside line()
  near(text has 'lock', text has 'await', 3[, ordered][, closest|first])   lines within 3 of each other
  empty                          a file of size 0 or a directory without entries; type = dir and empty
  type = dir                     also: type = file|link, type != file, type in (dir, link); without a type
                                 condition only regular files are returned; directories are written as 'name/'
Modifiers after the value: /i ignore case (Unicode), e.g. name = 'readme.md' /i; /w whole word on text
(no letter, digit or _ right before and after the match); /c ignore matches in comments of the file's language
(markers in strings are text; files of an unknown language are searched as is): text has 'unsafe' /c;
/e(backslash|quote_doubling|xml|url, 0..2|1|any) on = has starts ends of text also matches the value escaped that
many times (JSON inside JSON etc.), e.g. text has `\"type\": \"admin\"` /e(backslash, any); any = 0..3; the
level of each match is shown after the ids: ⟦#1/e1:...⟧; /n any naming style on = has starts ends of text, name
and path, case-insensitive: text has 'parse query' /n matches parseQuery ParseQuery parse_query PARSE_QUERY
parse-query parse.query parsequery; with /w the whole identifier; not with /e.
Labels: name ~ '*.rs' @rust or name ~ '*.go' @go — the output shows which condition matched.
# starts a comment until the end of the line (outside of strings).
Strings: '...' or \"...\" with the escapes \\' \\\" \\\\ \\n \\t; `...` raw.
Hidden files (unless traversal.hidden), files in .gitignore / .ignore / .aiignore and generated files are skipped; unreadable files and
directories too, with a permission_denied warning.

OUTPUT: the header \"files: N  lines: L  matches: M  result: r1\", the legend \"#1 <condition>\", then the files
under their directory headers (\"src/\", \"./\" for the working directory), each with the matched conditions [#1]
and its counts, then its matched lines: \"42:\" a matched line, \"--\" a gap, ⟦ids:text⟧ a match.
Spec version 1.";

/// The input schema of the `find` tool.
#[must_use]
pub fn find_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "dirs": {
                "type": "array", "items": { "type": "string" }, "minItems": 1,
                "description": "Working directories inside the allowed roots. Default: server.default_dirs."
            },
            "query": { "type": "string", "description": "Search query, see the tool description. Empty: all files." },
            "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max search time, e.g. '10s'. '0s': no limit." },
            "traversal": {
                "type": "object",
                "properties": {
                    "min_depth": { "type": "integer", "minimum": 0, "description": "Skip entries closer to the working directory; its children are depth 1." },
                    "max_depth": { "type": "integer", "minimum": 0, "description": "Don't descend deeper. Default: unlimited." },
                    "follow_symlinks": { "type": "boolean", "description": "Descend into symlinked directories, read symlinked files." },
                    "hidden": { "type": "boolean", "description": "Include hidden entries (names starting with '.')." }
                }
            }
        }
    })
}

/// The description of the `get_config` tool (`TOOLS.md#get_config`).
pub const GET_CONFIG_DESCRIPTION: &str = "\
Returns the effective configuration of the search server as TOML: allowed roots, default directories, traversal and
output defaults, limits, supported spec versions. Read-only. Call it once before the first find.";

/// The input schema of the `get_config` tool.
#[must_use]
pub fn get_config_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "section": { "enum": ["server", "traversal", "output", "escape", "query", "kinds", "git", "languages"] },
            "sources": { "type": "boolean", "description": "Add where each value comes from." }
        }
    })
}

/// The description of the `update_config` tool (`TOOLS.md#update_config`).
pub const UPDATE_CONFIG_DESCRIPTION: &str = "\
Changes the search server configuration in memory until the server stops: any value except server.allowed_roots and
server.allow_update. `set` is TOML like the config file, e.g. \"output.context = 2\" or
\"[output]\\ncontext = 2\\nmax_files = 30\". Lists are replaced, not merged. Atomic: if any value is invalid, nothing
changes. To change a value for one search only, pass it in the find request instead.";

/// The input schema of the `update_config` tool.
#[must_use]
pub fn update_config_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "set": { "type": "string", "description": "TOML with the values to change, like the config file, e.g. 'output.context = 2'." },
            "reset": { "type": "array", "items": { "type": "string" }, "description": "Keys like 'output.context', or ['all']." }
        }
    })
}
