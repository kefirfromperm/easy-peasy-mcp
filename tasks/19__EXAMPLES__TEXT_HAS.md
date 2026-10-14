# Examples for task 19

Spec: `1`

## Fixtures

### content

```text tree
docs/notes.md              2026-10-01 10:00
src/a.rs                   2026-10-01 10:00
src/b.rs                   2026-10-01 10:00
src/c.rs                   2026-10-01 10:00
src/crlf.txt               2026-10-01 10:00
```

File contents are in `tests/fixtures/content/`; `src/crlf.txt` has `\r\n` line breaks. Times are UTC. The lines
have no indentation. Where several files match, the file with more matches per line comes first in path order too,
as with relevance sorting.

## E19.1 Several matches on a line, matches in several files

Lines next to each other are one block, there is no `--` between them.

Fixture: `content`

```json request
{ "query": "text has 'unwrap('" }
```

```text response
files: 2  lines: 3  matches: 4  result: r1
#1 text has 'unwrap('

src/
a.rs  [#1]  lines: 2  matches: 3
2:let x = foo.⟦#1:unwrap(⟧);
3:let y = bar.⟦#1:unwrap(⟧).baz.⟦#1:unwrap(⟧);
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().⟦#1:unwrap(⟧);
```

## E19.2 Case-insensitive

Fixture: `content`

```json request
{ "query": "text has 'UNWRAP' /i" }
```

```text response
files: 3  lines: 4  matches: 5  result: r1
#1 text has 'UNWRAP' /i

src/
a.rs  [#1]  lines: 2  matches: 3
2:let x = foo.⟦#1:unwrap⟧();
3:let y = bar.⟦#1:unwrap⟧().baz.⟦#1:unwrap⟧();
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().⟦#1:unwrap⟧();
c.rs  [#1]  lines: 1  matches: 1
1:// ⟦#1:Unwrap⟧ is only mentioned in this comment
```

## E19.3 Files without a match, like `grep -L`

A condition under `not` produces no fragments; the file has no matched lines.

Fixture: `content`

```json request
{ "query": "name ~ '*.rs' and not text has 'unwrap('" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap('

src/
c.rs  [#1]
```

## E19.4 With a `name` condition

`name` conditions produce no fragments.

Fixture: `content`

```json request
{ "query": "name = 'b.rs' and text has 'parse()'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'b.rs'
#2 text has 'parse()'

src/
b.rs  [#1 #2]  lines: 1  matches: 1
2:let v = ⟦#2:parse()⟧.unwrap();
```

## E19.5 A value with a line break matches across lines

The fragment opens on its first line and closes on its last one; both lines are matched lines.

Fixture: `content`

```json request
{ "query": "text has '{\\nlet v'" }
```

```text response
files: 1  lines: 2  matches: 1  result: r1
#1 text has '{\nlet v'

src/
b.rs  [#1]  lines: 2  matches: 1
1:fn helper() -> Option<u8> ⟦#1:{
2:let v⟧ = parse().unwrap();
```

## E19.6 `\r\n` line breaks work like `\n`

Fixture: `content`

```json request
{ "query": "text has 'second Line\\nthird'" }
```

```text response
files: 1  lines: 2  matches: 1  result: r1
#1 text has 'second Line\nthird'

src/
crlf.txt  [#1]  lines: 2  matches: 1
2:⟦#1:second Line
3:third⟧
```

## E19.7 Only the true branches of `or` produce fragments

Fixture: `content`

```json request
{ "query": "name = 'a.rs' and (text has 'foo' or text has 'qux')" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'a.rs'
#2 text has 'foo'
#3 text has 'qux'

src/
a.rs  [#1 #2]  lines: 1  matches: 1
2:let x = ⟦#2:foo⟧.unwrap();
```

## E19.8 Overlapping fragments are one marked part

Both fragments are counted.

Fixture: `content`

```json request
{ "query": "text has 'unwrap().baz' and text has 'baz.unwrap'" }
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 text has 'unwrap().baz'
#2 text has 'baz.unwrap'

src/
a.rs  [#1 #2]  lines: 1  matches: 2
3:let y = bar.⟦#1,#2:unwrap().baz.unwrap⟧();
```

## E19.9 Identical fragments are one fragment

Fixture: `content`

```json request
{ "query": "text has 'foo' or text has 'FOO' /i" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text has 'foo'
#2 text has 'FOO' /i

src/
a.rs  [#1 #2]  lines: 1  matches: 1
2:let x = ⟦#1,#2:foo⟧.unwrap();
```

## E19.10 Separate lines are separate blocks

Fixture: `content`

```json request
{ "query": "name = 'a.rs' and (text has 'fn main' or text has 'done')" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'a.rs'
#2 text has 'fn main'
#3 text has 'done'

src/
a.rs  [#1 #2 #3]  lines: 2  matches: 2
1:⟦#2:fn main⟧() {
--
4:println!("⟦#3:done⟧");
```
