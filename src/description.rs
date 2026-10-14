//! What the agent reads: the server instructions, the tool descriptions and input schemas (TOOLS.md#tools).

use serde_json::{Value, json};

/// The `instructions` of `initialize`.
pub const INSTRUCTIONS: &str = "File search: find + grep in one query. Call get_config first to learn allowed roots and \
defaults. Spec version 1.";

/// The description of the `find` tool: the cheat sheet of TOOLS.md#find.
pub const FIND_DESCRIPTION: &str = r#"Search files by name, path, metadata and content in one query (find + grep in one). Read-only. Spec version 1.
Call get_config once first to learn the allowed roots and defaults.

QUERY: conditions joined with not, and, xor, or (this precedence) and brackets. Empty query = all files.
  name ~ '*.rs'                  wildcard on the file name: * ? ** [abc] [!abc] {a,b}
  path ~ 'src/**/*.ts'           wildcard on the path relative to the working dir, / separator
  text has 'TODO'                content contains; also: = starts ends, =~ regex (Rust syntax)
  text starts '#!'               text is the whole file; line(text starts '## ') = a line starts with it
  size > 1M  modified > -7d  created  accessed  type = dir  empty
  lang = rust   kind in (source, test)   git = changed|branch|staged|untracked
  name ~~ 'cfgldr'               fuzzy file name (also path ~~)
  line(text has 'a' and not text has 'b')    conditions on the same line; len > 120 works inside line()
  near(text has 'lock', text has 'await', 3[, ordered][, closest|first])   lines within 3 of each other
Modifiers after a text value: /i ignore case, /w whole word, /m multiline regex, /c skip comments,
  /n any naming style ('parse query' = parseQuery, parse_query, PARSE_QUERY ...),
  /e(backslash|quote_doubling|xml|url, 0..2|any) also match the escaped text (JSON inside JSON etc.)
Labels: text has 'x' @name — the output shows which condition matched; (... or ...) @name labels a bracket group.
Strings: '...' or "..." with \ escapes; `...` raw, best for regex.
Dates: 2026-10-01, -30m -2h -7d -2w. Sizes: 512 10K 5M 1G.

OUTPUT: output.detail = exists | count | facets | files | lines (default); output.context = N lines around.
output.budget = N tokens: the server picks the richest detail which fits. By default files are sorted by relevance.
Text response with the default options: "42:" matched line, "41-" context line, "--" gap, ⟦ids:text⟧ a match,
[#1 @x] matched conditions, ⇥N removed indentation; empty or huge results come with a funnel and hints how to fix
the query. output.omit = ['legend', 'line_numbers', ...] or 'all' leaves those parts out.
If the response has next_cursor, pass it back as output.cursor with the same request to get the next page.
Responses with count detail or higher have a result id (r12), unless the result cache is off: within = "r12"
searches only its files, since = "r12" returns only the changes.
queries = [{label, query}, ...] runs several queries in one call.
output.file = 'x.txt' writes the response to a file under the output dir and returns its path; for big results.

EXAMPLES
  name ~ '*.py' and text has 'import requests'
  not text has 'license' /i                                   files without a match
  name ~ '*.{ts,tsx}' and line(text has 'any' /w and not text starts '//')
  text has `"role": "admin"` /e(backslash, any)               also inside stringified JSON
  kind = source and name has 'config loader' /n               a file by a vague name
  git = changed and text has 'dbg!('                          leftovers in files with uncommitted changes"#;

/// The input schema of the `find` tool, the JSON text of TOOLS.md#find.
const FIND_SCHEMA: &str = r##"{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
    "query": { "type": "string", "description": "Search query, see the tool description. Empty: all files." },
    "queries": {
      "type": "array", "minItems": 1,
      "description": "Several queries in one call instead of `query`.",
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["query"],
        "properties": {
          "label": { "type": "string" },
          "query": { "type": "string" },
          "dirs": { "$ref": "#/properties/dirs" },
          "timeout": { "$ref": "#/properties/timeout" },
          "traversal": { "$ref": "#/properties/traversal" },
          "output": { "$ref": "#/properties/output" }
        }
      }
    },
    "within": { "type": "string", "description": "Result id: search only the files of that result." },
    "since": { "type": "string", "description": "Result id: return only the differences to that result." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max search time, e.g. '10s'. '0s': no limit." },
    "traversal": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "min_depth": { "type": "integer", "minimum": 0 },
        "max_depth": { "type": "integer", "minimum": 0 },
        "follow_symlinks": { "type": "boolean" },
        "hidden": { "type": "boolean" },
        "ignore_files": { "type": "array", "items": { "type": "string" } },
        "git_excludes": { "type": "boolean" },
        "ignore_paths": { "type": "array", "items": { "type": "string" } },
        "exclude": { "type": "array", "items": { "type": "string" } },
        "path_case": { "enum": ["native", "sensitive", "insensitive"] },
        "binary": { "enum": ["skip", "search", "exclude"] },
        "max_file_size": { "type": "string" },
        "encoding": { "type": "string" },
        "fallback_encoding": { "type": "string" },
        "archives": { "enum": ["none", "compressed", "all"] },
        "max_archive_size": { "type": "string" },
        "skip_generated": { "type": "boolean" }
      }
    },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "detail": { "enum": ["exists", "count", "facets", "files", "lines"] },
        "budget": { "type": "integer", "minimum": 100, "description": "Max response size in tokens." },
        "chars_per_token": { "type": "number", "exclusiveMinimum": 0 },
        "funnel": { "enum": ["auto", "always", "never"] },
        "facets": { "type": "array", "items": { "enum": ["dir", "ext", "lang", "kind", "git", "condition"] } },
        "facet_depth": { "type": "integer", "minimum": 1 },
        "facet_limit": { "type": "integer", "minimum": 1 },
        "meta": { "type": "boolean" },
        "context": { "type": "integer", "minimum": 0 },
        "before": { "type": "integer", "minimum": 0 },
        "after": { "type": "integer", "minimum": 0 },
        "path_fragments": { "type": "boolean" },
        "highlight": { "enum": ["auto", "none", "columns", "markers"] },
        "markers": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 },
        "max_files": { "type": "integer", "minimum": 1 },
        "max_lines": { "type": "integer", "minimum": 1 },
        "max_matches": { "type": "integer", "minimum": 1 },
        "max_matches_per_file": { "type": "integer", "minimum": 1 },
        "max_line_length": { "type": "integer", "minimum": 20 },
        "max_output_per_file": { "type": "integer", "minimum": 100 },
        "max_output": { "type": "integer", "minimum": 100 },
        "dedent": { "type": "boolean" },
        "blank_context": { "type": "boolean" },
        "group_dirs": { "type": "boolean" },
        "group_repeated": { "type": "integer", "minimum": 0 },
        "sort": { "type": "string", "pattern": "^(relevance|(path|modified|size|matches) (asc|desc))$" },
        "path_style": { "enum": ["relative", "absolute"] },
        "omit": {
          "anyOf": [
            { "type": "array", "items": { "enum": ["counts", "result", "legend", "funnel", "hints", "conditions", "file_counts", "match_ids", "groups", "line_numbers", "gaps", "indent", "warnings"] } },
            { "enum": ["all"] }
          ],
          "description": "Parts of the response to leave out, or \"all\"."
        },
        "cursor": { "type": "string" },
        "file": {
          "type": ["string", "boolean"],
          "description": "Write the response to this file under server.output_dir and return its path; true: a name like r12.txt."
        }
      }
    },
    "escape": {
      "type": "object",
      "additionalProperties": false,
      "properties": { "max_level": { "type": "integer", "minimum": 0 } }
    },
    "query_options": {
      "type": "object",
      "additionalProperties": false,
      "description": "The [query] section of the configuration.",
      "properties": { "fuzzy_min_score": { "type": "number", "minimum": 0, "maximum": 1 } }
    },
    "kinds": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "generated_markers": { "type": "array", "items": { "type": "string" } },
        "generated_marker_lines": { "type": "integer", "minimum": 1 },
        "generated": { "type": "array", "items": { "type": "string" } },
        "test": { "type": "array", "items": { "type": "string" } },
        "doc": { "type": "array", "items": { "type": "string" } },
        "config": { "type": "array", "items": { "type": "string" } }
      }
    },
    "git": {
      "type": "object",
      "additionalProperties": false,
      "properties": { "base": { "type": "string" } }
    },
    "languages": {
      "type": "object",
      "description": "Languages, replace the configured languages with the same name.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "extensions": { "type": "array", "items": { "type": "string" } },
          "names": { "type": "array", "items": { "type": "string" } },
          "shebangs": { "type": "array", "items": { "type": "string" } },
          "line_comments": { "type": "array", "items": { "type": "string" } },
          "block_comments": { "type": "array", "items": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 } },
          "nested_block_comments": { "type": "boolean" },
          "strings": { "type": "array", "items": { "type": "string" } },
          "source": { "type": "boolean" }
        }
      }
    }
  }
}"##;

/// The input schema of the `find` tool.
#[must_use]
pub fn find_schema() -> Value {
    serde_json::from_str(FIND_SCHEMA).expect("the schema of find is JSON")
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
Changes the search server configuration in memory until the server stops: any value except the read-only ones of
server (allowed_roots, allow_update, allow_write, output_dir, max_output_file_size). `set` is TOML like the config file, e.g.
\"output.context = 2\" or \"[output]\\ncontext = 2\\nmax_files = 30\". Lists are replaced, not merged. Atomic: if any value
is invalid, nothing changes. To change a value for one search only, pass it in the find request instead.";

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

/// The description of the `delete` tool (`TOOLS.md#delete`).
pub const DELETE_DESCRIPTION: &str = r#"Deletes files: moves them to the trash of the system (Trash, Recycle Bin), so a mistake can be undone. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
A directory is deleted with everything in it. Symlinks are deleted themselves, never followed.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be deleted and changes nothing. Use it for a selection you haven't seen.
permanent = true: deletes instead of moving to the trash. It can't be undone."#;

/// The input schema of the `delete` tool (`TOOLS.md#delete`).
const DELETE_SCHEMA: &str = r#"{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
    "traversal": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "min_depth": { "type": "integer", "minimum": 0 },
        "max_depth": { "type": "integer", "minimum": 0 },
        "follow_symlinks": { "type": "boolean" },
        "hidden": { "type": "boolean" },
        "ignore_files": { "type": "array", "items": { "type": "string" } },
        "git_excludes": { "type": "boolean" },
        "ignore_paths": { "type": "array", "items": { "type": "string" } },
        "exclude": { "type": "array", "items": { "type": "string" } },
        "path_case": { "enum": ["native", "sensitive", "insensitive"] },
        "binary": { "enum": ["skip", "search", "exclude"] },
        "max_file_size": { "type": "string" },
        "encoding": { "type": "string" },
        "fallback_encoding": { "type": "string" },
        "archives": { "enum": ["none", "compressed", "all"] },
        "max_archive_size": { "type": "string" },
        "skip_generated": { "type": "boolean" }
      }
    },
    "kinds": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "generated_markers": { "type": "array", "items": { "type": "string" } },
        "generated_marker_lines": { "type": "integer", "minimum": 1 },
        "generated": { "type": "array", "items": { "type": "string" } },
        "test": { "type": "array", "items": { "type": "string" } },
        "doc": { "type": "array", "items": { "type": "string" } },
        "config": { "type": "array", "items": { "type": "string" } }
      }
    },
    "languages": {
      "type": "object",
      "description": "Languages, replace the configured languages with the same name.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "extensions": { "type": "array", "items": { "type": "string" } },
          "names": { "type": "array", "items": { "type": "string" } },
          "shebangs": { "type": "array", "items": { "type": "string" } },
          "line_comments": { "type": "array", "items": { "type": "string" } },
          "block_comments": { "type": "array", "items": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 } },
          "nested_block_comments": { "type": "boolean" },
          "strings": { "type": "array", "items": { "type": "string" } },
          "source": { "type": "boolean" }
        }
      }
    },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "dry_run": { "type": "boolean", "description": "List what would be deleted, change nothing." },
    "permanent": { "type": "boolean", "description": "Delete instead of moving to the trash. Can't be undone." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" }
      }
    }
  }
}"#;

/// The input schema of the `delete` tool.
#[must_use]
pub fn delete_schema() -> Value {
    serde_json::from_str(DELETE_SCHEMA).expect("the schema of delete is JSON")
}

/// The description of the `rename` tool (`TOOLS.md#rename`).
pub const RENAME_DESCRIPTION: &str = r#"Renames or moves files: every file gets a new path made from a template. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
to = "...": the new path, relative to the working directory, / as the separator. Placeholders:
  {dir} {name} {stem} {ext} {path}   parts of the old path: src/a.test.ts = src, a.test.ts, a.test, ts, src/a.test.ts
  {1} {2} {word}                     groups of pattern; {{ and }} are braces
  e.g. "{dir}/{stem}.tsx", "archive/{path}"
pattern = "regex": must match the whole old path, e.g. "src/(.*)_test\.rs" with to = "tests/{1}.rs".
  A file which doesn't match is skipped with a warning.
Missing directories are created. A directory is moved with everything in it. Symlinks are moved themselves.
on_conflict: a target which exists, two files to one target, a target which is another selected file:
  "fail" (default): nothing is renamed, the error lists them; "skip": those files are skipped;
  "overwrite": an existing file is moved to the trash and replaced, a directory never.
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be renamed and changes nothing. Use it for a selection you haven't seen."#;

/// The input schema of the `rename` tool (`TOOLS.md#rename`).
const RENAME_SCHEMA: &str = r#"{
  "type": "object",
  "additionalProperties": false,
  "required": ["to"],
  "properties": {
    "version": { "type": "integer", "minimum": 1, "description": "Spec version. Omit to use the current one." },
    "result": { "type": "string", "description": "Result id of find, e.g. 'r12': all its files. Instead of query." },
    "query": { "type": "string", "description": "Query like in find: the files it matches. Instead of result." },
    "dirs": {
      "type": "array", "items": { "type": "string" }, "minItems": 1,
      "description": "Working directories inside the allowed roots. Default: server.default_dirs."
    },
    "traversal": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "min_depth": { "type": "integer", "minimum": 0 },
        "max_depth": { "type": "integer", "minimum": 0 },
        "follow_symlinks": { "type": "boolean" },
        "hidden": { "type": "boolean" },
        "ignore_files": { "type": "array", "items": { "type": "string" } },
        "git_excludes": { "type": "boolean" },
        "ignore_paths": { "type": "array", "items": { "type": "string" } },
        "exclude": { "type": "array", "items": { "type": "string" } },
        "path_case": { "enum": ["native", "sensitive", "insensitive"] },
        "binary": { "enum": ["skip", "search", "exclude"] },
        "max_file_size": { "type": "string" },
        "encoding": { "type": "string" },
        "fallback_encoding": { "type": "string" },
        "archives": { "enum": ["none", "compressed", "all"] },
        "max_archive_size": { "type": "string" },
        "skip_generated": { "type": "boolean" }
      }
    },
    "kinds": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "generated_markers": { "type": "array", "items": { "type": "string" } },
        "generated_marker_lines": { "type": "integer", "minimum": 1 },
        "generated": { "type": "array", "items": { "type": "string" } },
        "test": { "type": "array", "items": { "type": "string" } },
        "doc": { "type": "array", "items": { "type": "string" } },
        "config": { "type": "array", "items": { "type": "string" } }
      }
    },
    "languages": {
      "type": "object",
      "description": "Languages, replace the configured languages with the same name.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "extensions": { "type": "array", "items": { "type": "string" } },
          "names": { "type": "array", "items": { "type": "string" } },
          "shebangs": { "type": "array", "items": { "type": "string" } },
          "line_comments": { "type": "array", "items": { "type": "string" } },
          "block_comments": { "type": "array", "items": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 } },
          "nested_block_comments": { "type": "boolean" },
          "strings": { "type": "array", "items": { "type": "string" } },
          "source": { "type": "boolean" }
        }
      }
    },
    "to": { "type": "string", "minLength": 1, "description": "The new path, relative to the working directory: {dir} {name} {stem} {ext} {path}, groups {1} {word} of pattern." },
    "pattern": { "type": "string", "description": "Regex on the old path, the whole path must match; its groups are {1}, {2}, {name} in to." },
    "on_conflict": { "enum": ["fail", "skip", "overwrite"], "description": "A target which exists or collides. Default: fail." },
    "expect": { "type": "integer", "minimum": 0, "description": "Fail without changing anything unless the selection has this many files." },
    "dry_run": { "type": "boolean", "description": "List what would be renamed, change nothing." },
    "timeout": { "type": "string", "pattern": "^[0-9]+(ms|s|m)$", "description": "Max time of the call, e.g. '10s'. '0s': no limit." },
    "output": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "format": { "enum": ["text", "json"] },
        "group_dirs": { "type": "boolean" }
      }
    }
  }
}"#;

/// The input schema of the `rename` tool.
#[must_use]
pub fn rename_schema() -> Value {
    serde_json::from_str(RENAME_SCHEMA).expect("the schema of rename is JSON")
}

/// The description of the `copy` tool (`TOOLS.md#copy`).
pub const COPY_DESCRIPTION: &str = r#"Copies files: every file gets a copy at a path made from a template. Spec version 1.
Select the files with exactly one of:
  result = "r12"      all files of a find result, not only the shown ones
  query = "..."       a find query, with dirs, traversal, kinds, languages like in find
to = "...": the path of the copy, relative to the working directory, / as the separator. Placeholders:
  {dir} {name} {stem} {ext} {path}   parts of the old path: src/a.test.ts = src, a.test.ts, a.test, ts, src/a.test.ts
  {1} {2} {word}                     groups of pattern; {{ and }} are braces
  e.g. "{dir}/{stem}.bak.{ext}", "backup/{path}"
pattern = "regex": must match the whole old path, e.g. "src/(.*)\.rs" with to = "old/{1}.rs".
  A file which doesn't match is skipped with a warning.
Missing directories are created. A directory is copied with everything in it. Symlinks are copied as symlinks.
The content, the modification time and the permissions are kept; a failed copy leaves nothing behind.
on_conflict: a target which exists, two files to one target, a target which is another selected file:
  "fail" (default): nothing is copied, the error lists them; "skip": those files are skipped;
  "overwrite": an existing file is moved to the trash and replaced, a directory never.
A file changed since the result, a member of an archive, a path outside of the allowed roots: skipped with a warning.
expect = N: fails without changing anything unless the selection has N files.
dry_run = true: lists what would be copied and changes nothing. Use it for a selection you haven't seen."#;

/// The input schema of the `copy` tool (`TOOLS.md#copy`): the one of `rename` with its own descriptions of `to` and
/// `dry_run`.
#[must_use]
pub fn copy_schema() -> Value {
    let mut schema = rename_schema();
    let properties = &mut schema["properties"];
    properties["to"]["description"] = Value::from(
        "The path of the copy, relative to the working directory: {dir} {name} {stem} {ext} {path}, groups {1} {word} of pattern.",
    );
    properties["dry_run"]["description"] =
        Value::from("List what would be copied, change nothing.");
    schema
}
