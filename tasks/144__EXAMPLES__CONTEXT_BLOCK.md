# Examples for task 144

`output.after = "block"`: the context after a matched line goes on to the end of its block by indentation
(OUTPUT.md#block-context). The examples are written for the current spec version 2.

## Fixtures

### block

```text tree
README.md          2026-10-01 10:00
app.py             2026-10-01 10:00
src/lib.rs         2026-10-01 10:00
```

File contents are in `tests/fixtures/block/`: `src/lib.rs` the Rust functions `load` (lines 3-9, with an `if` block
in lines 5-7), `read` (lines 11-13) and `parse` with its parameters on their own lines (lines 15-21); `app.py` the
Python functions `load` (lines 4-8, a blank line 7 in the body) and `main` (lines 11-12); `README.md` a Markdown
heading `## Usage` in line 3 followed by its text.

## E144.1 A Rust function

Fixture: `block`

The body of the function and its closing line.

```json request
{"query": "text has 'fn load('", "output": {"after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
3:pub ⟦fn load(⟧path: &str) -> String {
4-    let raw = read(path);
5-    if raw.is_empty() {
6-        return String::new();
7-    }
8-    raw
9-}
```

## E144.2 A Python function has no closing line

Fixture: `block`

The block ends with the body: `def main():` is not more indented and is no closing line; the blank lines before it
are not part of the block, the blank line 7 inside it is skipped as a blank context line.

```json request
{"query": "text has 'def load('", "output": {"after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

./
app.py  lines: 1  matches: 1
4:⟦def load(⟧path):
5-    with open(path) as f:
6-        data = f.read()
8-    return data.strip()
```

## E144.3 A Markdown section is not a block

Fixture: `block`

The text after the heading isn't indented: the heading opens no block and gets no context.

```json request
{"query": "text has '## Usage'", "output": {"after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

./
README.md  lines: 1  matches: 1
3:⟦## Usage⟧
```

## E144.4 Two matches in one block

Fixture: `block`

The block of the `if` is inside the block of the function: one block, every line once.

```json request
{"query": "text has 'fn load(' or text has 'is_empty'", "output": {"after": "block"}}
```
```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'fn load('
#2 text has 'is_empty'

src/
lib.rs  lines: 2  matches: 2
3:pub ⟦#1:fn load(⟧path: &str) -> String {
4-    let raw = read(path);
5:    if raw.⟦#2:is_empty⟧() {
6-        return String::new();
7-    }
8-    raw
9-}
```

## E144.5 A match in a nested block

Fixture: `block`

The block of the `if` ends with its closing line; the block is dedented as always.

```json request
{"query": "text has 'is_empty'", "output": {"after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
⇥4
5:if raw.⟦is_empty⟧() {
6-    return String::new();
7-}
```

## E144.6 Matches on the closing lines

Fixture: `block`

A closing line opens no block: no context after it. The blocks are separated by `--`, as with any context. The
markers would outgrow the one character of the lines, so `highlight = auto` writes them plain.

```json request
{"query": "line(text starts '}')", "output": {"after": "block"}}
```
```text response
files: 1  lines: 3  matches: 3  result: r1

src/
lib.rs  lines: 3  matches: 3
9:}
--
13:}
--
21:}
```

## E144.7 A closing line which opens the block again

Fixture: `block`

The parameters are a block; `) -> Vec<String> {` closes it and opens the body, so the block goes on to the `}` of
the function.

```json request
{"query": "text has 'fn parse('", "output": {"after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
15:pub ⟦fn parse(⟧
16-    text: &str,
17-) -> Vec<String> {
18-    text.lines()
19-        .map(str::to_string)
20-        .collect()
21-}
```

## E144.8 A block cut by `max_output_per_file`

Fixture: `block`

The lines 15-19 have 90 characters, line 20 would make 108: the lines 20 and 21 of the block are left out.

```json request
{"query": "text has 'fn parse('", "output": {"after": "block", "max_output_per_file": 100}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1  truncated: yes (max_output_per_file)

src/
lib.rs  lines: 1  matches: 1  (cut)
15:pub ⟦fn parse(⟧
16-    text: &str,
17-) -> Vec<String> {
18-    text.lines()
19-        .map(str::to_string)
… 2 more lines of the block
```

## E144.9 Without the line of the cut block

Fixture: `block`

```json request
{"query": "text has 'fn parse('", "output": {"after": "block", "max_output_per_file": 100, "omit": ["more_lines"]}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1  truncated: yes (max_output_per_file)

src/
lib.rs  lines: 1  matches: 1  (cut)
15:pub ⟦fn parse(⟧
16-    text: &str,
17-) -> Vec<String> {
18-    text.lines()
19-        .map(str::to_string)
```

## E144.10 A cut block in JSON

Fixture: `block`

```json request
{"query": "text has 'fn parse('", "output": {"format": "json", "after": "block", "max_output_per_file": 100}}
```
```json response
{
  "version": 2,
  "exists": true,
  "result": "r1",
  "truncated": true,
  "truncated_by": ["max_output_per_file"],
  "counts": {"files": 1, "lines": 1, "matches": 1},
  "files": [
    {"path": "src/lib.rs", "lines": 1, "matches": 1, "truncated": true, "blocks": [
      {"lines": [
        {"n": 15, "text": "pub fn parse(", "fragments": [{"col": [5, 14], "text": "fn parse("}]},
        {"n": 16, "text": "    text: &str,"},
        {"n": 17, "text": ") -> Vec<String> {"},
        {"n": 18, "text": "    text.lines()"},
        {"n": 19, "text": "        .map(str::to_string)"}
      ], "more_lines": 2}
    ]}
  ]
}
```

## E144.11 `max_lines` counts the matched lines only

Fixture: `block`

The first matched line is listed with its whole block, the next ones are left for the next page.

```json request
{"query": "text =~ `fn \\w+\\(`", "output": {"after": "block", "max_lines": 1}}
```
```text response
files: 1  lines: 3  matches: 3  result: r1  truncated: yes (max_lines)  next cursor: eyJ2IjoxLCJoIjoiYzdiZjc3YzQiLCJmIjowLCJtIjoxLCJ0IjpbMSwzLDNdfQ

src/
lib.rs  lines: 3  matches: 3
3:pub ⟦fn load(⟧path: &str) -> String {
4-    let raw = read(path);
5-    if raw.is_empty() {
6-        return String::new();
7-    }
8-    raw
9-}
```

## E144.12 With `before`

Fixture: `block`

Two lines before the matched line, the block after it.

```json request
{"query": "text has 'is_empty'", "output": {"before": 2, "after": "block"}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
3-pub fn load(path: &str) -> String {
4-    let raw = read(path);
5:    if raw.⟦is_empty⟧() {
6-        return String::new();
7-    }
```

## E144.13 Without dedent

Fixture: `block`

```json request
{"query": "text has 'is_empty'", "output": {"after": "block", "dedent": false}}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
5:    if raw.⟦is_empty⟧() {
6-        return String::new();
7-    }
```

## E144.14 The budget drops the block context

Fixture: `block`

The three functions with their blocks don't fit into 100 tokens (400 characters); without the context lines, the
level 2 of the budget, they do.

```json request
{"query": "text =~ `fn \\w+\\(`", "output": {"after": "block", "budget": 100}}
```
```text response
files: 1  lines: 3  matches: 3  result: r1  truncated: yes (budget)

src/
lib.rs  lines: 3  matches: 3
3:pub ⟦fn load(⟧path: &str) -> String {
--
11:⟦fn read(⟧path: &str) -> String {
--
15:pub ⟦fn parse(⟧
```

## E144.15 An invalid value in a request

Fixture: `block`

```json request
{"output": {"after": "body"}}
```
```text error
error[invalid_option]: 'output.after' must be a whole number or "block"
```

## E144.16 `update_config`

Fixture: `block`

1. the block as the configured default

```json request update_config
{"set": "output.after = \"block\""}
```
```text response
[output]
after = "block"    # was (not set)
```

2. `find`

```json request
{"query": "text has 'is_empty'"}
```
```text response
files: 1  lines: 1  matches: 1  result: r1

src/
lib.rs  lines: 1  matches: 1
⇥4
5:if raw.⟦is_empty⟧() {
6-    return String::new();
7-}
```

3. an invalid value

```json request update_config
{"set": "output.after = \"body\""}
```
```text error
error[invalid_value]: invalid value for 'output.after'
  hint: a whole number or "block" is expected
```
