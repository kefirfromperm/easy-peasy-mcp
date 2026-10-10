# Examples for task 30

## Fixtures

### escaped

```text tree
data/links.txt             2026-10-01 10:00
data/page.html             2026-10-01 10:00
data/query.sql             2026-10-01 10:00
data/strings.js            2026-10-01 10:00
data/users.json            2026-10-01 10:00
```

File contents are in `tests/fixtures/escaped/`. `data/users.json`:

```text
{"users": [{ "name": "root", "role": "admin" }],
  "audit": {
    "level1": "{\"role\": \"admin\"}",
    "level2": "{\"inner\": \"{\\\"role\\\": \\\"admin\\\"}\"}",
    "other": "{\"role\": \"viewer\"}"
  }
}
```

`data/strings.js`:

```text
const a = 'it\'s';
const b = "it's";
const c = "line one\nline two";
```

`data/query.sql`:

```text
SELECT * FROM users WHERE name = 'O''Brien';
-- O'Brien
```

`data/page.html`:

```text
<p>Use &lt;script&gt; tags carefully</p>
<p title="it&apos;s">it&#39;s</p>
<script>run()</script>
```

`data/links.txt`:

```text
https://example.com/search?q%3Dhello%20world
https://example.com/search?q%3Dhello+world
https://example.com/search?q=hello world
```

Times are UTC.

## E30.1 JSON inside JSON on every level

Line 1 is raw JSON, line 3 a stringified object, line 4 one inside another.

Fixture: `escaped`

```json request
{ "query": "name = 'users.json' and text has `\"role\": \"admin\"` /e(backslash, any)" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'users.json'
#2 text has `"role": "admin"` /e(backslash, any)

data/
users.json  [#1 #2]  lines: 3  matches: 3
1:{"users": [{ "name": "root", ⟦#2/e0:"role": "admin"⟧ }],
--
3:    "level1": "{⟦#2/e1:\"role\": \"admin\"⟧}",
4:    "level2": "{\"inner\": \"{⟦#2/e2:\\\"role\\\": \\\"admin\\\"⟧}\"}",
```

## E30.2 A range of levels

Fixture: `escaped`

```json request
{ "query": "name = 'users.json' and text has `\"role\": \"admin\"` /e(backslash, 1..2)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'users.json'
#2 text has `"role": "admin"` /e(backslash, 1..2)

data/
users.json  [#1 #2]  lines: 2  matches: 2
3:    "level1": "{⟦#2/e1:\"role\": \"admin\"⟧}",
4:    "level2": "{\"inner\": \"{⟦#2/e2:\\\"role\\\": \\\"admin\\\"⟧}\"}",
```

## E30.3 `'` with and without a backslash

JSON doesn't escape `'`, so level 1 accepts both forms.

Fixture: `escaped`

```json request
{ "query": "name = 'strings.js' and text has \"it's\" /e(backslash, 1)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'strings.js'
#2 text has "it's" /e(backslash, 1)

data/
strings.js  [#1 #2]  lines: 2  matches: 2
1:const a = '⟦#2/e1:it\'s⟧';
2:const b = "⟦#2/e1:it's⟧";
```

## E30.4 A line break escaped as `\n`

Fixture: `escaped`

```json request
{ "query": "name = 'strings.js' and text has 'one\\nline' /e(backslash, 1)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'strings.js'
#2 text has 'one\nline' /e(backslash, 1)

data/
strings.js  [#1 #2]  lines: 1  matches: 1
3:const c = "line ⟦#2/e1:one\nline⟧ two";
```

## E30.5 A name inside an SQL string literal

The raw name in the comment is level 0, which the query leaves out.

Fixture: `escaped`

```json request
{ "query": "name = 'query.sql' and text has `O'Brien` /e(quote_doubling, 1)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'query.sql'
#2 text has `O'Brien` /e(quote_doubling, 1)

data/
query.sql  [#1 #2]  lines: 1  matches: 1
1:SELECT * FROM users WHERE name = '⟦#2/e1:O''Brien⟧';
```

## E30.6 An escaped tag in HTML

Fixture: `escaped`

```json request
{ "query": "name = 'page.html' and text has '<script>' /e(xml, 1)" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'page.html'
#2 text has '<script>' /e(xml, 1)

data/
page.html  [#1 #2]  lines: 1  matches: 1
1:<p>Use ⟦#2/e1:&lt;script&gt;⟧ tags carefully</p>
```

## E30.7 `&apos;` and `&#39;`

Fixture: `escaped`

```json request
{ "query": "name = 'page.html' and text has \"it's\" /e(xml, any)" }
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 name = 'page.html'
#2 text has "it's" /e(xml, any)

data/
page.html  [#1 #2]  lines: 1  matches: 2
2:<p title="⟦#2/e1:it&apos;s⟧">⟦#2/e1:it&#39;s⟧</p>
```

## E30.8 `/i` on the escaped variants

The style is written in any case.

Fixture: `escaped`

```json request
{ "query": "name = 'page.html' and text has '<SCRIPT>' /e(XML, 1) /i" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'page.html'
#2 text has '<SCRIPT>' /e(XML, 1) /i

data/
page.html  [#1 #2]  lines: 1  matches: 1
1:<p>Use ⟦#2/e1:&lt;script&gt;⟧ tags carefully</p>
```

## E30.9 A query string with `%20` and `+`

Fixture: `escaped`

```json request
{ "query": "name = 'links.txt' and text has 'q=hello world' /e(url, 1)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'links.txt'
#2 text has 'q=hello world' /e(url, 1)

data/
links.txt  [#1 #2]  lines: 2  matches: 2
1:https://example.com/search?⟦#2/e1:q%3Dhello%20world⟧
2:https://example.com/search?⟦#2/e1:q%3Dhello+world⟧
```

## E30.10 Level 0 is marked too

Fixture: `escaped`

```json request
{ "query": "name = 'links.txt' and text has 'q=hello world' /e(url, any)" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'links.txt'
#2 text has 'q=hello world' /e(url, any)

data/
links.txt  [#1 #2]  lines: 3  matches: 3
1:https://example.com/search?⟦#2/e1:q%3Dhello%20world⟧
2:https://example.com/search?⟦#2/e1:q%3Dhello+world⟧
3:https://example.com/search?⟦#2/e0:q=hello world⟧
```

## E30.11 A level above the max level

Fixture: `escaped`

```json request
{ "query": "text has 'a' /e(backslash, 9)" }
```

```text error
error[invalid_escape_level]: invalid escape level 9
  text has 'a' /e(backslash, 9)
                             ^
  hint: max level is 3 (configuration escape.max_level)
```

## E30.12 A range above the max level

Fixture: `escaped`

```json request
{ "query": "text has 'a' /e(url, 0..5)" }
```

```text error
error[invalid_escape_level]: invalid escape level 5
  text has 'a' /e(url, 0..5)
                          ^
  hint: max level is 3 (configuration escape.max_level)
```

## E30.13 Levels which are not a number, a range or `any`

Fixture: `escaped`

```json request
{ "query": "text has 'a' /e(xml, all)" }
```

```text error
error[invalid_escape_level]: invalid escape level 'all'
  text has 'a' /e(xml, all)
                       ^^^
  hint: levels: a number, a range like 0..2, or any
```

## E30.14 A reversed range

Fixture: `escaped`

```json request
{ "query": "text has 'a' /e(url, 2..1)" }
```

```text error
error[invalid_range]: invalid range '2..1'
  text has 'a' /e(url, 2..1)
                       ^^^^
  hint: the start must not be greater than the end
```

## E30.15 An unknown style

Fixture: `escaped`

```json request
{ "query": "text has 'a' /e(json, 1)" }
```

```text error
error[unknown_value]: unknown value 'json'
  text has 'a' /e(json, 1)
                  ^^^^
  hint: expected one of: backslash, quote_doubling, xml, url
```
