# Examples for task 128

Spec: `1`

Groups of the query ([QUERY.md#groups](../QUERY.md#groups)): a label after a bracket, the capture groups of a `text =~`
condition as sub-groups in the JSON format.

No field of a request refers to groups yet: the [group references](../QUERY.md#group-references) and their error
`unknown_group` are shown by the examples of the tasks which add such fields.

## Fixtures

### query-groups

```text tree
Cargo.toml                 2026-10-01 10:00
lib.rs                     2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
src/c.rs                   2026-10-01 10:00
```

In `tests/fixtures/query-groups/`:

- `src/a.rs`: `use serde::Deserialize;` (line 1), `fn main() {` (line 3), `    let apple = 1;` (line 4)
- `src/b.rs`: `use serde_json::Value;` (line 1), `// apple pie` (line 2)
- `src/c.rs`: `use serde::Serialize;`, no `apple`
- `lib.rs`: `pub fn load_config(path: &Path) -> Config {` (line 1), `    parse(path)` (line 2),
  `fn parse(path: &Path) -> Config {` (line 5), `    todo!()` (line 6)
- `Cargo.toml`: a `[package]`

## E128.1 A group around the branches of `or`

The bracket with the label `@ser` is a group: it is in the legend in the order of its `(`, in the conditions of the
files and in the markers of the matches inside it, before their ids. `apple` is not in the group. In `b.rs` the
fragments of `#1` (`serde`) and `#2` (`serde_json`) overlap and are marked as one part.

Fixture: `query-groups`

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 4  matches: 5  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'

src/
a.rs  [@ser #1 #3]  lines: 2  matches: 2
1:use ⟦@ser,#1:serde⟧::Deserialize;
--
⇥4
4:let ⟦#3:apple⟧ = 1;
b.rs  [@ser #1 #2 #3]  lines: 2  matches: 3
1:use ⟦@ser,#1,#2:serde_json⟧::Value;
2:// ⟦#3:apple⟧ pie
```

## E128.2 The `condition` facet counts a group

Fixture: `query-groups`

```json request
{"query": "(text has 'serde' or text has 'serde_json') @ser and text has 'apple'", "output": {"detail": "facets", "facets": ["condition"]}}
```

```text response
files: 2  lines: 4  matches: 5  result: r1
@ser (text has 'serde' or text has 'serde_json')
#1 text has 'serde'
#2 text has 'serde_json'
#3 text has 'apple'
condition:  @ser 2 · #1 2 · #3 2 · #2 1
```

## E128.3 Nested groups

`@all` holds everything, `@lib` only the `serde` and `Value` matches; a fragment gets the ids of all the groups around
its condition.

Fixture: `query-groups`

```json request
{"query": "((text has 'serde' or text has 'Value') @lib and text has 'apple') @all", "output": {"sort": "path asc", "highlight": "markers"}}
```

```text response
files: 2  lines: 4  matches: 5  result: r1
@all ((text has 'serde' or text has 'Value') and text has 'apple')
@lib (text has 'serde' or text has 'Value')
#1 text has 'serde'
#2 text has 'Value'
#3 text has 'apple'

src/
a.rs  [@all @lib #1 #3]  lines: 2  matches: 2
1:use ⟦@all,@lib,#1:serde⟧::Deserialize;
--
⇥4
4:let ⟦@all,#3:apple⟧ = 1;
b.rs  [@all @lib #1 #2 #3]  lines: 2  matches: 3
1:use ⟦@all,@lib,#1:serde⟧_json::⟦@all,@lib,#2:Value⟧;
2:// ⟦@all,#3:apple⟧ pie
```

## E128.4 A group inside `line( ... )`

A group inside `line( ... )` is not listed per file, like the conditions inside it; its id is in the markers of the
line.

Fixture: `query-groups`

```json request
{"query": "line((text has 'load' or text has 'parse') @call and not text has 'fn')", "output": {"highlight": "markers"}}
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 line((text has 'load' or text has 'parse') and not text has 'fn')
@call (text has 'load' or text has 'parse')
#2 text has 'load'
#3 text has 'parse'
#4 text has 'fn'

./
lib.rs  [#1]  lines: 1  matches: 1
⇥4
2:⟦#1,@call,#3:parse⟧(path)
```

## E128.5 A group without a `text` condition

A group of metadata conditions is a condition id; it has no matched text.

Fixture: `query-groups`

```json request
{"query": "(name ~ '*.toml' or name = 'lib.rs') @top", "output": {"sort": "path asc"}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
@top (name ~ '*.toml' or name = 'lib.rs')
#1 name ~ '*.toml'
#2 name = 'lib.rs'

./
Cargo.toml  [@top #1]
lib.rs  [@top #2]
```

## E128.6 Regex groups in the JSON format

The capture groups of `@def` are listed in the `groups` of its fragments: group 1 (`pub `) only in line 1, where it
took part in the match; the named group `fname` by its number and its name.

Fixture: `query-groups`

```json request
{"query": "name = 'lib.rs' and text =~ `(pub\\s+)?fn\\s+(?<fname>\\w+)\\(` @def", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 2, "matches": 2 },
  "legend": {
    "#1": "name = 'lib.rs'",
    "@def": "text =~ `(pub\\s+)?fn\\s+(?<fname>\\w+)\\(`"
  },
  "files": [
    {
      "path": "lib.rs",
      "lines": 2,
      "matches": 2,
      "conditions": ["#1", "@def"],
      "blocks": [
        {
          "lines": [
            { "n": 1, "text": "pub fn load_config(path: &Path) -> Config {",
              "fragments": [ { "col": [1, 20], "text": "pub fn load_config(", "ids": ["@def"],
                "groups": [
                  { "col": [1, 5], "text": "pub ", "ids": ["@def.1"] },
                  { "col": [8, 19], "text": "load_config", "ids": ["@def.2", "@def.fname"] }
                ] } ] }
          ]
        },
        {
          "lines": [
            { "n": 5, "text": "fn parse(path: &Path) -> Config {",
              "fragments": [ { "col": [1, 10], "text": "fn parse(", "ids": ["@def"],
                "groups": [
                  { "col": [4, 9], "text": "parse", "ids": ["@def.2", "@def.fname"] }
                ] } ] }
          ]
        }
      ]
    }
  ]
}
```

## E128.7 Regex groups don't change the text format

The same query in the text format: the capture groups are not marked.

Fixture: `query-groups`

```json request
{"query": "name = 'lib.rs' and text =~ `(pub\\s+)?fn\\s+(?<fname>\\w+)\\(` @def"}
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'lib.rs'
@def text =~ `(pub\s+)?fn\s+(?<fname>\w+)\(`

./
lib.rs  [#1 @def]  lines: 2  matches: 2
1:⟦@def:pub fn load_config(⟧path: &Path) -> Config {
--
5:⟦@def:fn parse(⟧path: &Path) -> Config {
```

## E128.8 Sub-groups of a numbered regex inside `line( ... )`

Without a label the sub-groups are numbered after the id of the condition: `#2.1`. The fragment has the ids of
`line( ... )` and of the regex; only the regex has sub-groups.

Fixture: `query-groups`

```json request
{"query": "line(text =~ `^fn (p\\w+)`)", "output": {"format": "json"}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": {
    "#1": "line(text =~ `^fn (p\\w+)`)",
    "#2": "text =~ `^fn (p\\w+)`"
  },
  "files": [
    {
      "path": "lib.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        {
          "lines": [
            { "n": 5, "text": "fn parse(path: &Path) -> Config {",
              "fragments": [ { "col": [1, 9], "text": "fn parse", "ids": ["#1", "#2"],
                "groups": [ { "col": [4, 9], "text": "parse", "ids": ["#2.1"] } ] } ] }
          ]
        }
      ]
    }
  ]
}
```

## E128.9 `omit` leaves out the sub-groups

Fixture: `query-groups`

```json request
{"query": "line(text =~ `^fn (p\\w+)`)", "output": {"format": "json", "omit": ["groups"]}}
```

```json response
{
  "version": 1,
  "exists": true,
  "result": "r1",
  "counts": { "files": 1, "lines": 1, "matches": 1 },
  "legend": {
    "#1": "line(text =~ `^fn (p\\w+)`)",
    "#2": "text =~ `^fn (p\\w+)`"
  },
  "files": [
    {
      "path": "lib.rs",
      "lines": 1,
      "matches": 1,
      "conditions": ["#1"],
      "blocks": [
        {
          "lines": [
            { "n": 5, "text": "fn parse(path: &Path) -> Config {",
              "fragments": [ { "col": [1, 9], "text": "fn parse", "ids": ["#1", "#2"] } ] }
          ]
        }
      ]
    }
  ]
}
```

## E128.10 A group label used by a condition too

Labels are unique across conditions and groups: the later use is the duplicate.

Fixture: `query-groups`

```json request
{"query": "(text has 'a' @x or text has 'b') @x"}
```

```text error
error[duplicate_label]: duplicate label '@x'
  (text has 'a' @x or text has 'b') @x
                                    ^^
  hint: labels must be unique
```

## E128.11 A second label after a group

A group has one label, like a condition.

Fixture: `query-groups`

```json request
{"query": "(text has 'a') @x @y"}
```

```text error
error[unexpected_token]: unexpected '@y'
  (text has 'a') @x @y
                    ^^
  hint: missing 'and' / 'or' between conditions?
```
