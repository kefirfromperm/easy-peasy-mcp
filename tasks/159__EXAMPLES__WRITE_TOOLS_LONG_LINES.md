# Examples for task 159

The changed lines of `replace` longer than `output.max_line_length` are cut around the change
([TOOLS.md#result-6](../TOOLS.md#result-6)), like the long lines of `find`
([OUTPUT.md#long-lines](../OUTPUT.md#long-lines)). The examples are written for the current spec version 2. They are
startup examples with `--allow-write`, like the examples of `replace` of task 129.

## Fixtures

### long-lines

```text tree
data/tools.json      2026-10-01 10:00
notes/para.md        2026-10-01 10:00
```

In `tests/fixtures/long-lines/`, the words `w000`, `w001`, ... are filler, each of 4 characters and a space between:

- `data/tools.json`: one line of 677 characters, `{"description": "Search files. w000 … w059 Leftovers in files
  with uncommitted changes. w060 … w119"}`
- `notes/para.md`: line 1 of 572 characters, `alpha w000 … w019 beta w020 … w089 gamma w090 … w109 omega`; line 2
  empty; line 3 `A short line with beta in it.`

## E159.1 One match in a long line: the dry run

The one-line JSON of `data/tools.json` has 677 characters, over the default `max_line_length` of 300. The old line
is cut around the match, the new line around its new text: `(300 − 19) / 2 = 140` and `(300 − 7) / 2 = 146`
characters on both sides.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'uncommitted changes' @old", "replace": {"@old": "changes"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

data/
tools.json  matches: 1
-1:… w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with uncommitted changes. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w08…
+1:…5 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with changes. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088…
```

## E159.2 One match in a long line: the real call

The real call writes the same cut lines; the file gets the whole new line, which the JSON dry run of the next
change shows: JSON writes the lines whole.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

1. the real call

```json request replace
{"query": "text has 'uncommitted changes' @old", "replace": {"@old": "changes"}, "expect_matches": 1}
```
```text response
replaced: 1  matches: 1

data/
tools.json  matches: 1
-1:… w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with uncommitted changes. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w08…
+1:…5 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with changes. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088…
```

2. JSON: the lines whole

```json request replace
{"query": "text has 'changes.' @old", "replace": {"@old": "edits."}, "dry_run": true, "output": {"format": "json"}}
```
```json response
{
  "version": 2,
  "replaced": 1,
  "matches": 1,
  "dry_run": true,
  "files": [
    {
      "path": "data/tools.json",
      "matches": 1,
      "changes": [
        {
          "line": 1,
          "old": [
            "{\"description\": \"Search files. w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with changes. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 w110 w111 w112 w113 w114 w115 w116 w117 w118 w119\"}"
          ],
          "new_line": 1,
          "new": [
            "{\"description\": \"Search files. w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 Leftovers in files with edits. w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 w110 w111 w112 w113 w114 w115 w116 w117 w118 w119\"}"
          ]
        }
      ]
    }
  ]
}
```

## E159.3 A match near the start, a line under the limit

In line 1 of `notes/para.md` (572 characters) `beta` starts at character 106, closer to the start than
`(300 − 4) / 2 = 148`: its window starts at the start of the line, no `…` before it. Line 3 is under the limit
and written whole.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'beta' @old", "replace": {"@old": "BETA"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 2

notes/
para.md  matches: 2
-1:alpha w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 beta w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w0…
+1:alpha w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 BETA w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w0…
-3:A short line with beta in it.
+3:A short line with BETA in it.
```

## E159.4 A match near the end

`gamma` ends 106 characters before the end of the line, closer than `(300 − 5) / 2 = 147`: no `…` after it.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'gamma' @old", "replace": {"@old": "GAMMA"}, "dry_run": true}
```
```text response
dry run  would replace: 1  matches: 1

notes/
para.md  matches: 1
-1:…0 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 gamma w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 omega
+1:…0 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 GAMMA w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 omega
```

## E159.5 `max_line_length` of the request, a match at the end

`output.max_line_length` of the request replaces the configured 300: `(60 − 5) / 2 = 27` characters before
`omega`, which ends the line: no `…` after it.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has 'omega' @old", "replace": {"@old": "OMEGA"}, "dry_run": true, "output": {"max_line_length": 60}}
```
```text response
dry run  would replace: 1  matches: 1

notes/
para.md  matches: 1
-1:…4 w105 w106 w107 w108 w109 omega
+1:…4 w105 w106 w107 w108 w109 OMEGA
```

## E159.6 Two matches far apart

Two matches in one line get a window each, `(60 − 14) / 4 = 11` characters on both sides, with `…` between them;
in the new line `(60 − 16) / 4 = 11`.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text =~ `beta w020|gamma` @old", "replace": {"@old": "{@old}!"}, "dry_run": true, "output": {"max_line_length": 60}}
```
```text response
dry run  would replace: 1  matches: 2

notes/
para.md  matches: 2
-1:… w018 w019 beta w020 w021 w022 … w088 w089 gamma w090 w091 …
+1:… w018 w019 beta w020! w021 w022 … w088 w089 gamma! w090 w091 …
```

## E159.7 A match deleted

An empty new text is a part of no characters at its place: the new line keeps `40 / 2 = 20` characters on both
sides of it.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

```json request replace
{"query": "text has ' gamma' @old", "replace": {"@old": ""}, "dry_run": true, "output": {"max_line_length": 40}}
```
```text response
dry run  would replace: 1  matches: 1

notes/
para.md  matches: 1
-1:…86 w087 w088 w089 gamma w090 w091 w092 w…
+1:… w086 w087 w088 w089 w090 w091 w092 w093…
```

## E159.8 Spec 1

Spec 1 writes the lines whole unless the request sets `output.max_line_length`.

Fixture: `long-lines`

```text command
easy-peasy-mcp --allow-write
```

1. the lines whole

```json request replace
{"version": 1, "query": "text has 'omega' @old", "replace": {"@old": "OMEGA"}, "dry_run": true}
```
```text response
replaced: 1  matches: 1  dry run

notes/
para.md  matches: 1
-1:alpha w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 beta w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 gamma w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 omega
+1:alpha w000 w001 w002 w003 w004 w005 w006 w007 w008 w009 w010 w011 w012 w013 w014 w015 w016 w017 w018 w019 beta w020 w021 w022 w023 w024 w025 w026 w027 w028 w029 w030 w031 w032 w033 w034 w035 w036 w037 w038 w039 w040 w041 w042 w043 w044 w045 w046 w047 w048 w049 w050 w051 w052 w053 w054 w055 w056 w057 w058 w059 w060 w061 w062 w063 w064 w065 w066 w067 w068 w069 w070 w071 w072 w073 w074 w075 w076 w077 w078 w079 w080 w081 w082 w083 w084 w085 w086 w087 w088 w089 gamma w090 w091 w092 w093 w094 w095 w096 w097 w098 w099 w100 w101 w102 w103 w104 w105 w106 w107 w108 w109 OMEGA
```

2. `max_line_length` of the request

```json request replace
{"version": 1, "query": "text has 'omega' @old", "replace": {"@old": "OMEGA"}, "dry_run": true, "output": {"max_line_length": 60}}
```
```text response
replaced: 1  matches: 1  dry run

notes/
para.md  matches: 1
-1:…4 w105 w106 w107 w108 w109 omega
+1:…4 w105 w106 w107 w108 w109 OMEGA
```
