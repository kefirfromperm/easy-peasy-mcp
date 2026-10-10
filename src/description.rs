//! What the agent reads: the server instructions, the tool descriptions and input schemas (TOOLS.md#tools).

use serde_json::{Value, json};

/// The `instructions` of `initialize`.
pub const INSTRUCTIONS: &str = "File search: find + grep in one query. Spec version 1.";

/// The description of the `find` tool: the cheat sheet of TOOLS.md#description for the implemented subset.
pub const FIND_DESCRIPTION: &str = "\
Search files by name and path in one query. Read-only. Only regular files are returned. Spec version 1.

QUERY: one condition on the file name or path. Empty query = all files.
  name = 'Cargo.toml'            exact file name; also: has (contains), starts, ends
  name ~ '*.rs'                  wildcard on the file name: * ? [abc] [a-z] [!abc] {a,b};
                                 [*] [?] [[] [{] match the character itself
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator; ** any dirs
  path starts 'docs/'            =, has, starts, ends work on paths too
Modifier after the value: /i ignore case (Unicode), e.g. name = 'readme.md' /i.
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
