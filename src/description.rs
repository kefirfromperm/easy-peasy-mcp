//! What the agent reads: the server instructions, the tool descriptions and input schemas (TOOLS.md#tools).

use serde_json::{Value, json};

/// The `instructions` of `initialize`.
pub const INSTRUCTIONS: &str = "File search: find + grep in one query. Spec version 1.";

/// The description of the `find` tool: the cheat sheet of TOOLS.md#description for the implemented subset.
pub const FIND_DESCRIPTION: &str = "\
Search files and directories by name, path and type in one query. Read-only. Only regular files are returned. Spec version 1.

QUERY: conditions on the name, path, type, size and modification date joined with not, and, xor, or (this precedence) and brackets.
Empty query = all files.
  name = 'Cargo.toml'            exact file name; also: has (contains), starts, ends
  name ~ '*.rs'                  wildcard on the file name: * ? [abc] [a-z] [!abc] {a,b};
                                 [*] [?] [[] [{] match the character itself
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator; ** any dirs
  path starts 'docs/'            =, has, starts, ends work on paths too
  size > 1M                      also: = != < <= >= and size in 10K..1M; units B K M G (1024)
  modified > -7d                 also: modified = 2026-10-01 (the whole day), modified in -7d..-1d;
                                 dates 2026-10-01, 2026-10-01T14:30, ...T14:30:00Z, ...+02:00 (local time without
                                 an offset), relative -30m -2h -7d -2w (units s m h d w)
  empty                          a file of size 0 or a directory without entries; type = dir and empty
  type = dir                     also: type = file|link, type != file, type in (dir, link); without a type
                                 condition only regular files are returned; directories are written as 'name/'
Modifier after the value: /i ignore case (Unicode), e.g. name = 'readme.md' /i.
Labels: name ~ '*.rs' @rust or name ~ '*.go' @go — the output shows which condition matched.
# starts a comment until the end of the line (outside of strings).
Strings: '...' or \"...\" with the escapes \\' \\\" \\\\ \\n \\t; `...` raw.
Hidden files, files in .gitignore / .ignore / .aiignore and generated files are skipped.

OUTPUT: the header \"files: N  lines: 0  matches: 0  result: r1\", the legend \"#1 <condition>\", then the files
under their directory headers (\"src/\", \"./\" for the working directory), each with the matched conditions [#1].
Spec version 1.";

/// The input schema of the `find` tool.
#[must_use]
pub fn find_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": { "type": "string", "description": "Search query, see the tool description. Empty: all files." }
        }
    })
}
