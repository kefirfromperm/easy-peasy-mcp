# Tasks

The roadmap from the MVP to the whole specification. How a task is worked on is described in
[AGENTS.md](../AGENTS.md#workflow); the status of a task is in its task file, and the
Status column repeats it.

## MVP

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 1 | [MVP — find files by name](1__TASK__MVP.md) | — | implemented |

## Query on file metadata

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 2 | [Empty query: all files](2__TASK__EMPTY_QUERY.md) | 1 | implemented |
| 3 | [`name` with `has`, `starts`, `ends`](3__TASK__NAME_OPERATORS.md) | 1 | implemented |
| 4 | [`path` target](4__TASK__PATH_TARGET.md) | 3 | implemented |
| 5 | [`/i` modifier on `name` and `path`](5__TASK__MODIFIER_I.md) | 4 | implemented |
| 6 | [Logical operators `not`, `and`, `xor`, `or` and brackets](6__TASK__LOGICAL_OPERATORS.md) | 5 | implemented |
| 7 | [Condition labels `@name`](7__TASK__LABELS.md) | 6 | implemented |
| 8 | [Comments in the query](8__TASK__QUERY_COMMENTS.md) | 6 | implemented |
| 9 | [`type` target](9__TASK__TYPE_TARGET.md) | 6 | implemented |
| 10 | [`empty` target](10__TASK__EMPTY_TARGET.md) | 9 | implemented |
| 11 | [`size` target](11__TASK__SIZE_TARGET.md) | 6 | implemented |
| 12 | [`modified` target and dates](12__TASK__MODIFIED_TARGET.md) | 11 | implemented |
| 13 | [`created` target](13__TASK__CREATED_TARGET.md) | 12 | implemented |
| 14 | [`accessed` target](14__TASK__ACCESSED_TARGET.md) | 13 | implemented |
| 15 | [`lang` target](15__TASK__LANG_TARGET.md) | 9 | implemented |
| 16 | [`kind` target](16__TASK__KIND_TARGET.md) | 15 | implemented |
| 17 | [`git` target: working tree statuses](17__TASK__GIT_STATUS_TARGET.md) | 16 | implemented |
| 18 | [`git = branch`](18__TASK__GIT_BRANCH.md) | 17 | implemented |

## Content search

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 19 | [`text has` in file scope](19__TASK__TEXT_HAS.md) | 6 | implemented |
| 20 | [Binary files and unknown content](20__TASK__UNKNOWN_CONTENT.md) | 19, 13 | implemented |
| 21 | [`text` with `=`, `starts`, `ends`](21__TASK__TEXT_OPERATORS.md) | 19 | implemented |
| 22 | [Regex operator `=~`](22__TASK__REGEX.md) | 21 | implemented |
| 23 | [`/w` whole word modifier](23__TASK__MODIFIER_W.md) | 22 | implemented |
| 24 | [`/m` multiline regex](24__TASK__MODIFIER_M.md) | 22 | implemented |
| 25 | [Encoding detection](25__TASK__ENCODING_AUTO.md) | 20 | implemented |
| 26 | [Line scope `line( ... )`](26__TASK__LINE_SCOPE.md) | 22 | implemented |
| 27 | [`len` target](27__TASK__LEN_TARGET.md) | 26 | implemented |
| 28 | [Proximity `near( ... )`](28__TASK__NEAR.md) | 26 | implemented |
| 29 | [`/c` ignore matches in comments](29__TASK__MODIFIER_C.md) | 26 | implemented |
| 30 | [`/e` match escaped text](30__TASK__MODIFIER_E.md) | 23 | implemented |
| 31 | [`/n` any naming style](31__TASK__MODIFIER_N.md) | 23 | implemented |
| 32 | [Fuzzy `~~` on `name` and `path`](32__TASK__FUZZY.md) | 4 | implemented |

## Warnings

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 33 | [Unreadable entries](33__TASK__PERMISSION_DENIED.md) | 20 | implemented |

## Configuration

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 34 | [Config file](34__TASK__CONFIG_FILE.md) | 1 | implemented |
| 35 | [`get_config` tool](35__TASK__GET_CONFIG.md) | 34 | implemented |
| 36 | [`update_config` tool](36__TASK__UPDATE_CONFIG.md) | 35 | implemented |
| 37 | [`--log-level`](37__TASK__LOG_LEVEL.md) | 34 | implemented |

## `[server]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 38 | [`server.allowed_roots`](38__TASK__SERVER_ALLOWED_ROOTS.md) | 34 | implemented |
| 39 | [`server.default_dirs`](39__TASK__SERVER_DEFAULT_DIRS.md) | 38 | implemented |
| 40 | [`server.timeout`](40__TASK__SERVER_TIMEOUT.md) | 39 | implemented |
| 41 | [`server.threads`](41__TASK__SERVER_THREADS.md) | 40 | implemented |
| 42 | [`server.allow_update`](42__TASK__SERVER_ALLOW_UPDATE.md) | 36 | implemented |
| 43 | [`server.result_cache`](43__TASK__SERVER_RESULT_CACHE.md) | 36 | implemented |

## `[traversal]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 44 | [`traversal.min_depth`](44__TASK__TRAVERSAL_MIN_DEPTH.md) | 36 | implemented |
| 45 | [`traversal.max_depth`](45__TASK__TRAVERSAL_MAX_DEPTH.md) | 44 | implemented |
| 46 | [`traversal.follow_symlinks`](46__TASK__TRAVERSAL_FOLLOW_SYMLINKS.md) | 44, 9 | implemented |
| 47 | [`traversal.hidden`](47__TASK__TRAVERSAL_HIDDEN.md) | 44 | implemented |
| 48 | [`traversal.ignore_files`](48__TASK__TRAVERSAL_IGNORE_FILES.md) | 44 | implemented |
| 49 | [`traversal.git_excludes`](49__TASK__TRAVERSAL_GIT_EXCLUDES.md) | 48 | implemented |
| 50 | [`traversal.ignore_paths`](50__TASK__TRAVERSAL_IGNORE_PATHS.md) | 48 | implemented |
| 51 | [`traversal.exclude`](51__TASK__TRAVERSAL_EXCLUDE.md) | 44 | implemented |
| 52 | [`traversal.path_case`](52__TASK__TRAVERSAL_PATH_CASE.md) | 44 | implemented |
| 53 | [`traversal.binary`](53__TASK__TRAVERSAL_BINARY.md) | 20, 44 | implemented |
| 54 | [`traversal.max_file_size`](54__TASK__TRAVERSAL_MAX_FILE_SIZE.md) | 20, 44 | implemented |
| 55 | [`traversal.encoding`](55__TASK__TRAVERSAL_ENCODING.md) | 25, 44 | implemented |
| 56 | [`traversal.fallback_encoding`](56__TASK__TRAVERSAL_FALLBACK_ENCODING.md) | 55 | implemented |
| 57 | [`traversal.archives`: `none` and `compressed`](57__TASK__TRAVERSAL_ARCHIVES_COMPRESSED.md) | 53, 54 | implemented |
| 58 | [`traversal.archives`: `all`](58__TASK__TRAVERSAL_ARCHIVES_ALL.md) | 57 | implemented |
| 59 | [`traversal.max_archive_size`](59__TASK__TRAVERSAL_MAX_ARCHIVE_SIZE.md) | 58 | implemented |
| 60 | [`traversal.skip_generated`](60__TASK__TRAVERSAL_SKIP_GENERATED.md) | 16, 44 | implemented |

## `[escape]` and `[query]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 61 | [`escape.max_level`](61__TASK__ESCAPE_MAX_LEVEL.md) | 30, 36 | implemented |
| 62 | [`query.fuzzy_min_score`](62__TASK__QUERY_FUZZY_MIN_SCORE.md) | 32, 36 | implemented |

## `[kinds]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 63 | [`kinds.generated_markers`](63__TASK__KINDS_GENERATED_MARKERS.md) | 16, 36 | implemented |
| 64 | [`kinds.generated_marker_lines`](64__TASK__KINDS_GENERATED_MARKER_LINES.md) | 63 | implemented |
| 65 | [`kinds.generated`](65__TASK__KINDS_GENERATED.md) | 63 | implemented |
| 66 | [`kinds.test`](66__TASK__KINDS_TEST.md) | 63 | implemented |
| 67 | [`kinds.doc`](67__TASK__KINDS_DOC.md) | 63 | implemented |
| 68 | [`kinds.config`](68__TASK__KINDS_CONFIG.md) | 63 | implemented |

## `[git]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 69 | [`git.base`](69__TASK__GIT_BASE.md) | 18, 36 | implemented |

## `[languages]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 70 | [`languages.<name>.extensions`](70__TASK__LANGUAGES_EXTENSIONS.md) | 15, 36 | implemented |
| 71 | [`languages.<name>.names`](71__TASK__LANGUAGES_NAMES.md) | 70 | implemented |
| 72 | [`languages.<name>.shebangs`](72__TASK__LANGUAGES_SHEBANGS.md) | 70 | implemented |
| 73 | [`languages.<name>.line_comments`](73__TASK__LANGUAGES_LINE_COMMENTS.md) | 29, 70 | implemented |
| 74 | [`languages.<name>.block_comments`](74__TASK__LANGUAGES_BLOCK_COMMENTS.md) | 73 | implemented |
| 75 | [`languages.<name>.nested_block_comments`](75__TASK__LANGUAGES_NESTED_BLOCK_COMMENTS.md) | 74 | implemented |
| 76 | [`languages.<name>.strings`](76__TASK__LANGUAGES_STRINGS.md) | 73 | implemented |
| 77 | [`languages.<name>.source`](77__TASK__LANGUAGES_SOURCE.md) | 16, 70 | implemented |

## `[output]`

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 78 | [`output.format`](78__TASK__OUTPUT_FORMAT.md) | 36 | implemented |
| 79 | [`output.detail`](79__TASK__OUTPUT_DETAIL.md) | 78 | implemented |
| 80 | [`output.context`](80__TASK__OUTPUT_CONTEXT.md) | 79 | implemented |
| 81 | [`output.before`](81__TASK__OUTPUT_BEFORE.md) | 80 | implemented |
| 82 | [`output.after`](82__TASK__OUTPUT_AFTER.md) | 80 | implemented |
| 83 | [`output.blank_context`](83__TASK__OUTPUT_BLANK_CONTEXT.md) | 80 | implemented |
| 84 | [`output.dedent`](84__TASK__OUTPUT_DEDENT.md) | 80 | implemented |
| 85 | [`output.group_dirs`](85__TASK__OUTPUT_GROUP_DIRS.md) | 78 | implemented |
| 86 | [`output.group_repeated`](86__TASK__OUTPUT_GROUP_REPEATED.md) | 80 | implemented |
| 87 | [`output.highlight`](87__TASK__OUTPUT_HIGHLIGHT.md) | 78 | implemented |
| 88 | [`output.markers`](88__TASK__OUTPUT_MARKERS.md) | 87 | implemented |
| 89 | [`output.path_fragments`](89__TASK__OUTPUT_PATH_FRAGMENTS.md) | 78 | implemented |
| 90 | [`output.path_style`](90__TASK__OUTPUT_PATH_STYLE.md) | 39 | implemented |
| 91 | [`output.meta`](91__TASK__OUTPUT_META.md) | 79, 16 | implemented |
| 92 | [`output.sort`](92__TASK__OUTPUT_SORT.md) | 79 | implemented |
| 93 | [Relevance ranking](93__TASK__RELEVANCE.md) | 92, 91, 32 | implemented |
| 94 | [`output.max_files`](94__TASK__OUTPUT_MAX_FILES.md) | 92 | implemented |
| 95 | [`output.max_lines`](95__TASK__OUTPUT_MAX_LINES.md) | 94 | implemented |
| 96 | [`output.max_matches`](96__TASK__OUTPUT_MAX_MATCHES.md) | 94 | implemented |
| 97 | [`output.max_matches_per_file`](97__TASK__OUTPUT_MAX_MATCHES_PER_FILE.md) | 94 | implemented |
| 98 | [`output.max_line_length`](98__TASK__OUTPUT_MAX_LINE_LENGTH.md) | 80 | implemented |
| 99 | [`output.max_output_per_file`](99__TASK__OUTPUT_MAX_OUTPUT_PER_FILE.md) | 97 | implemented |
| 100 | [`output.max_output`](100__TASK__OUTPUT_MAX_OUTPUT.md) | 94 | implemented |
| 101 | [`output.cursor`: pagination](101__TASK__OUTPUT_CURSOR.md) | 94, 95, 96, 100 | implemented |
| 102 | [`output.funnel`](102__TASK__OUTPUT_FUNNEL.md) | 94 | implemented |
| 103 | [Hints for empty results](103__TASK__FUNNEL_HINTS_EMPTY.md) | 102 | implemented |
| 104 | [Hints for huge results](104__TASK__FUNNEL_HINTS_HUGE.md) | 103, 15 | implemented |
| 105 | [`output.facets`](105__TASK__OUTPUT_FACETS.md) | 79, 16, 17 | implemented |
| 106 | [`output.facet_depth`](106__TASK__OUTPUT_FACET_DEPTH.md) | 105 | implemented |
| 107 | [`output.facet_limit`](107__TASK__OUTPUT_FACET_LIMIT.md) | 105 | implemented |
| 108 | [`output.budget`](108__TASK__OUTPUT_BUDGET.md) | 101, 105, 98 | implemented |
| 109 | [`output.chars_per_token`](109__TASK__OUTPUT_CHARS_PER_TOKEN.md) | 108 | implemented |

## Request features

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 110 | [Request field `version`](110__TASK__VERSION_FIELD.md) | 78 | implemented |
| 111 | [Refine a previous result: `within`](111__TASK__WITHIN.md) | 43, 39 | implemented |
| 112 | [Changes since a previous result: `since`](112__TASK__SINCE.md) | 111 | implemented |
| 113 | [Several queries in one call: `queries`](113__TASK__BATCH.md) | 110, 108 | implemented |

## Protocol

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 114 | [Progress notifications](114__TASK__PROGRESS.md) | 40 | implemented |
| 115 | [Cancellation](115__TASK__CANCELLATION.md) | 114 | implemented |
| 116 | [Concurrent requests](116__TASK__CONCURRENCY.md) | 41, 36 | implemented |
| 117 | [Final tool descriptions and instructions](117__TASK__TOOL_DESCRIPTIONS.md) | 113, 115, 116 | implemented |

## Platforms

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 118 | [CI on Linux, macOS and Windows](118__TASK__CROSS_PLATFORM_CI.md) | 1 | implemented |
| 119 | [macOS specifics](119__TASK__MACOS.md) | 118, 47, 52 | implemented |
| 120 | [Windows specifics](120__TASK__WINDOWS.md) | 118, 47, 52, 9 | implemented |

## Conformance

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 121 | [Spec examples as tests](121__TASK__SPEC_CONFORMANCE.md) | 117, 119, 120 | implemented |

## Fixes from use

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 122 | [The cursor of a file list cut by the budget](122__TASK__BUDGET_CURSOR.md) | 108, 101 | not implemented |
| 123 | [Hints for an empty fuzzy result](123__TASK__FUZZY_HINTS.md) | 103, 62 | not implemented |
| 124 | [No line for a facet without values](124__TASK__EMPTY_FACET.md) | 105 | not implemented |

## Write tools

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 125 | [`delete` tool](125__TASK__DELETE_TOOL.md) | 111, 115, 120 | not implemented |
| 126 | [`rename` tool](126__TASK__RENAME_TOOL.md) | 125 | not implemented |
| 127 | [`copy` tool](127__TASK__COPY_TOOL.md) | 126 | not implemented |

## Groups and replace

| # | Task | Depends on | Status |
|---|------|------------|--------|
| 128 | [Query groups](128__TASK__QUERY_GROUPS.md) | 7, 22 | not implemented |
| 129 | [`replace` tool](129__TASK__REPLACE_TOOL.md) | 125, 128 | not implemented |
| 130 | [Output limited to groups](130__TASK__OUTPUT_GROUPS.md) | 128 | not implemented |
