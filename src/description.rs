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
entries; \"exclude\": [\"**/target/**\"] never walks these paths (wildcards like path ~), much faster than not path ~;
\"path_case\": \"insensitive\" compares paths and names case-insensitively (default native: sensitive on Linux only).

QUERY: conditions on the name, path, type, size, dates, language, kind, git status and content joined with not, and, xor, or (this precedence) and brackets.
Empty query = all files.
  name = 'Cargo.toml'            exact file name; also: has (contains), starts, ends
  name ~ '*.rs'                  wildcard on the file name: * ? [abc] [a-z] [!abc] {a,b};
                                 [*] [?] [[] [{] match the character itself
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator; ** any dirs
  path starts 'docs/'            =, has, starts, ends, =~ (regex) work on paths and names too
  name ~~ 'cfgldr'               fuzzy, like fzf: the characters in order, case-insensitive; space separated terms
                                 must all match (path ~~ 'srv hndl'); weak matches (score below
                                 query_options.fuzzy_min_score, default 0.5) are dropped
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
                                 or traversal.skip_generated is false
  git = changed                  also: modified staged untracked conflicted clean; git in (modified, untracked);
                                 git = branch: changed, or changed by the commits since the base (main/master);
                                 false outside of a repository
  text has 'TODO'                content contains, /i ignore case; a value with '\n' matches across lines;
                                 also = starts ends on the whole content: text starts '#!/bin/bash';
                                 text =~ `fn \\w+_test` regex (Rust syntax), per line: ^ $ are line ends;
                                 with /m on the whole content: \\n matches a line break;
                                 binary files are not searched: content conditions are unknown for them
                                 (traversal.binary: \"search\" searches them as text, \"exclude\" never returns them),
                                 neither are files bigger than traversal.max_file_size (default \"10M\");
                                 traversal.archives = \"compressed\" searches .gz .bz2 .xz .zst .lz4 decompressed,
                                 \"all\" also archive members: path ~ '**/*!/**' is only inside archives
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
many times (JSON inside JSON etc.), e.g. text has `\"type\": \"admin\"` /e(backslash, any); any = 0 to
escape.max_level (default 3, \"escape\": {\"max_level\": 5} in the request); the
level of each match is shown after the ids: ⟦#1/e1:...⟧; /n any naming style on = has starts ends of text, name
and path, case-insensitive: text has 'parse query' /n matches parseQuery ParseQuery parse_query PARSE_QUERY
parse-query parse.query parsequery; with /w the whole identifier; not with /e.
Labels: name ~ '*.rs' @rust or name ~ '*.go' @go — the output shows which condition matched.
# starts a comment until the end of the line (outside of strings).
Strings: '...' or \"...\" with the escapes \\' \\\" \\\\ \\n \\t; `...` raw.
Hidden files (unless traversal.hidden), files in .gitignore / .ignore / .aiignore (traversal.ignore_files), in
.git/info/exclude (traversal.git_excludes) and in the extra ignore files of traversal.ignore_paths, and generated files
are skipped; unreadable files and directories too, with a permission_denied warning.

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
                    "hidden": { "type": "boolean", "description": "Include hidden entries (names starting with '.')." },
                    "ignore_files": { "type": "array", "items": { "type": "string" }, "description": "Ignore files looked up in every directory, a later one wins. Default: .gitignore, .ignore, .aiignore; [] none." },
                    "git_excludes": { "type": "boolean", "description": "Respect .git/info/exclude and the global git excludes file. Default: true." },
                    "ignore_paths": { "type": "array", "items": { "type": "string" }, "description": "Extra ignore files (gitignore format) applied as if they were in every working directory, with the lowest priority; ~ is the home directory. Default: []." },
                    "exclude": { "type": "array", "items": { "type": "string" }, "description": "Wildcards of paths which are never walked, relative to the working directory, matched like path ~: ['**/target/**'] prunes every target directory. Default: []." },
                    "path_case": { "enum": ["native", "sensitive", "insensitive"], "description": "Case sensitivity of path and name conditions and of exclude; native: sensitive on Linux, insensitive on macOS and Windows. /i, ~~ and /n are always case-insensitive. Default: native." },
                    "binary": { "enum": ["skip", "search", "exclude"], "description": "Binary files (a zero byte in the first 8K): skip — not searched by content, content conditions are unknown for them; search — searched as text; exclude — never returned. Default: skip." },
                    "max_file_size": { "type": "string", "description": "Bigger files are not searched by content (content conditions are unknown for them, a too_big warning), e.g. '1G'; units B K M G. Default: 10M." },
                    "encoding": { "type": "string", "description": "auto: by the BOM, UTF-16 by its zero bytes, otherwise UTF-8; or a WHATWG encoding name like windows-1252 for every file without a BOM. The output shows the encoding of every file which is not UTF-8. Default: auto." },
                    "fallback_encoding": { "type": "string", "description": "With encoding auto: files which are not valid UTF-8 are decoded with this WHATWG encoding, e.g. windows-1251, instead of replacing the invalid bytes. Default: not set." },
                    "archives": { "enum": ["none", "compressed", "all"], "description": "none: compressed files and archives are binary files; compressed: .gz .bz2 .xz .zst .lz4 files are searched as their decompressed content (max_file_size applies to it); all: also the members of .zip .jar .tar .tar.gz/.tgz .tar.bz2 .tar.xz .tar.zst .7z, with the path <archive>!/<member>. Default: none." },
                    "max_archive_size": { "type": "string", "description": "The total uncompressed size of the members of one archive; the rest is skipped with an archive_too_big warning (zip bombs). Default: 100M." },
                    "skip_generated": { "type": "boolean", "description": "Skip generated files (kind = generated: lock files, *.min.js, @generated markers) unless the query has a kind condition. Default: true." }
                }
            },
            "escape": {
                "type": "object",
                "properties": {
                    "max_level": { "type": "integer", "minimum": 0, "description": "The upper level of /e(style, any) and the highest level a query may ask for. Default: 3." }
                }
            },
            "query_options": {
                "type": "object",
                "description": "The [query] section of the configuration.",
                "properties": {
                    "fuzzy_min_score": { "type": "number", "minimum": 0, "maximum": 1, "description": "~~ matches with a lower score (0-1) are dropped. Default: 0.5." }
                }
            },
            "kinds": {
                "type": "object",
                "description": "The rules of the kind target; a list replaces the configured one.",
                "properties": {
                    "generated_markers": { "type": "array", "items": { "type": "string" }, "description": "A file with one of them in its first lines is generated. Default: @generated, DO NOT EDIT, Code generated by, auto-generated." },
                    "generated_marker_lines": { "type": "integer", "minimum": 1, "description": "How many first lines are checked for the markers. Default: 5." },
                    "generated": { "type": "array", "items": { "type": "string" }, "description": "Wildcards of the paths of generated files (matched like path ~). Default: lock files, *.min.js, *.pb.go, ..." },
                    "test": { "type": "array", "items": { "type": "string" }, "description": "Wildcards of the paths of tests (matched like path ~). Default: **/test/**, **/tests/**, **/*_test.*, ..." },
                    "doc": { "type": "array", "items": { "type": "string" }, "description": "Wildcards of the paths of documentation (matched like path ~). Default: **/*.md, **/README*, **/docs/**, ..." },
                    "config": { "type": "array", "items": { "type": "string" }, "description": "Wildcards of the paths of configuration (matched like path ~). Default: **/*.toml, **/*.yaml, **/Dockerfile, ..." }
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
