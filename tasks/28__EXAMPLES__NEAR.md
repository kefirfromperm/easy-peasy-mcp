# Examples for task 28

## Fixtures

### near

```text tree
src/cache.rs               2026-10-01 10:00
src/files.py               2026-10-01 10:00
src/inline.py              2026-10-01 10:00
src/main.go                2026-10-01 10:00
src/tests.rs               2026-10-01 10:00
src/tie.txt                2026-10-01 10:00
```

File contents are in `tests/fixtures/near/`. `src/files.py`:

```text
f = open('a')
data = f.read()
close(f)
g = open('b')
h = open('c')
close(h)
close(g)
```

`src/inline.py` has `t = open('t'); close(t)` and `u = close(u); open('u')`; `src/tie.txt` has the lines `b`, `a`,
`b`. Times are UTC.

## E28.1 A lock and an `await` close to each other

The fragments carry the id of the inner condition and of `near`; the file lists `near`.

Fixture: `near`

```json request
{ "query": "near(text has '.lock()' @lock, text has '.await' @await, 3)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 near(text has '.lock()', text has '.await', 3)
@lock text has '.lock()'
@await text has '.await'

src/
cache.rs  [#1]  lines: 2  matches: 2
2:let mut guard = self.inner⟦#1,@lock:.lock()⟧;
--
4:let value = fetch(key)⟦#1,@await:.await⟧?;
```

## E28.2 `ordered, closest`: every `open(` with the nearest `close(` after it

Line 7 is not the nearest `close(` of any `open(`.

Fixture: `near`

```json request
{ "query": "name = 'files.py' and near(text has 'open(', text has 'close(', 20, ordered, closest)" }
```

```text response
files: 1  lines: 5  matches: 5  result: r1
#1 name = 'files.py'
#2 near(text has 'open(', text has 'close(', 20, ordered, closest)
#3 text has 'open('
#4 text has 'close('

src/
files.py  [#1 #2]  lines: 5  matches: 5
1:f = ⟦#2,#3:open(⟧'a')
--
3:⟦#2,#4:close(⟧f)
4:g = ⟦#2,#3:open(⟧'b')
5:h = ⟦#2,#3:open(⟧'c')
6:⟦#2,#4:close(⟧h)
```

## E28.3 `report = all`, the default

Fixture: `near`

```json request
{ "query": "name = 'files.py' and near(text has 'open(', text has 'close(', 20, ordered)" }
```

```text response
files: 1  lines: 6  matches: 6  result: r1
#1 name = 'files.py'
#2 near(text has 'open(', text has 'close(', 20, ordered)
#3 text has 'open('
#4 text has 'close('

src/
files.py  [#1 #2]  lines: 6  matches: 6
1:f = ⟦#2,#3:open(⟧'a')
--
3:⟦#2,#4:close(⟧f)
4:g = ⟦#2,#3:open(⟧'b')
5:h = ⟦#2,#3:open(⟧'c')
6:⟦#2,#4:close(⟧h)
7:⟦#2,#4:close(⟧g)
```

## E28.4 `first`: only the first pair

The options can be written in any order.

Fixture: `near`

```json request
{ "query": "name = 'files.py' and near(text has 'open(', text has 'close(', 20, first, ordered)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'files.py'
#2 near(text has 'open(', text has 'close(', 20, first, ordered)
#3 text has 'open('
#4 text has 'close('

src/
files.py  [#1 #2]  lines: 2  matches: 2
1:f = ⟦#2,#3:open(⟧'a')
--
3:⟦#2,#4:close(⟧f)
```

## E28.5 `any` order, the default

Fixture: `near`

```json request
{ "query": "name = 'files.py' and near(text has 'close(', text has 'open(', 1)" }
```

```text response
files: 1  lines: 4  matches: 4  result: r1
#1 name = 'files.py'
#2 near(text has 'close(', text has 'open(', 1)
#3 text has 'close('
#4 text has 'open('

src/
files.py  [#1 #2]  lines: 4  matches: 4
3:⟦#2,#3:close(⟧f)
4:g = ⟦#2,#4:open(⟧'b')
5:h = ⟦#2,#4:open(⟧'c')
6:⟦#2,#3:close(⟧h)
```

## E28.6 `ordered`: B after A

Fixture: `near`

```json request
{ "query": "name = 'files.py' and near(text has 'close(', text has 'open(', 1, ordered)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'files.py'
#2 near(text has 'close(', text has 'open(', 1, ordered)
#3 text has 'close('
#4 text has 'open('

src/
files.py  [#1 #2]  lines: 2  matches: 2
3:⟦#2,#3:close(⟧f)
4:g = ⟦#2,#4:open(⟧'b')
```

## E28.7 `N = 0`: the same line

Fixture: `near`

```json request
{ "query": "name = 'inline.py' and near(text has 'open(', text has 'close(', 0)" }
```

```text response
files: 1  lines: 2  matches: 4  result: r1
#1 name = 'inline.py'
#2 near(text has 'open(', text has 'close(', 0)
#3 text has 'open('
#4 text has 'close('

src/
inline.py  [#1 #2]  lines: 2  matches: 4
1:t = ⟦#2,#3:open(⟧'t'); ⟦#2,#4:close(⟧t)
2:u = ⟦#2,#4:close(⟧u); ⟦#2,#3:open(⟧'u')
```

## E28.8 `ordered` on the same line: the B match starts after the A match

Fixture: `near`

```json request
{ "query": "name = 'inline.py' and near(text has 'open(', text has 'close(', 0, ordered)" }
```

```text response
files: 1  lines: 1  matches: 2  result: r1
#1 name = 'inline.py'
#2 near(text has 'open(', text has 'close(', 0, ordered)
#3 text has 'open('
#4 text has 'close('

src/
inline.py  [#1 #2]  lines: 1  matches: 2
1:t = ⟦#2,#3:open(⟧'t'); ⟦#2,#4:close(⟧t)
```

## E28.9 Ignored tests

Fixture: `near`

```json request
{ "query": "near(text =~ `^\\s*#\\[test\\]`, text has '#[ignore]', 1)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 near(text =~ `^\s*#\[test\]`, text has '#[ignore]', 1)
#2 text =~ `^\s*#\[test\]`
#3 text has '#[ignore]'

src/
tests.rs  [#1]  lines: 2  matches: 2
3:⟦#1,#2:#[test]⟧
4:⟦#1,#3:#[ignore]⟧
```

## E28.10 A swallowed error in Go

Fixture: `near`

```json request
{ "query": "name ~ '*.go' and near(text has 'err != nil', text has 'return nil', 2)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name ~ '*.go'
#2 near(text has 'err != nil', text has 'return nil', 2)
#3 text has 'err != nil'
#4 text has 'return nil'

src/
main.go  [#1 #2]  lines: 2  matches: 2
2:if ⟦#2,#3:err != nil⟧ {
3:⟦#2,#4:return nil⟧
```

## E28.11 `closest` with a tie takes the earlier line

Fixture: `near`

```json request
{ "query": "name = 'tie.txt' and near(text = 'a', text = 'b', 1, closest)" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'tie.txt'
#2 near(text = 'a', text = 'b', 1, closest)
#3 text = 'a'
#4 text = 'b'

src/
tie.txt  [#1 #2]  lines: 2  matches: 2
1:⟦#2,#4:b⟧
2:⟦#2,#3:a⟧
```

## E28.12 Unknown option

Fixture: `near`

```json request
{ "query": "near(text has 'a', text has 'b', 3, nearest)" }
```

```text error
error[invalid_near_option]: unknown option 'nearest'
  near(text has 'a', text has 'b', 3, nearest)
                                      ^^^^^^^
  hint: options: any, ordered, all, closest, first
```

## E28.13 Conflicting options

Fixture: `near`

```json request
{ "query": "near(text has 'a', text has 'b', 3, any, ordered)" }
```

```text error
error[conflicting_options]: conflicting options 'any' and 'ordered'
  near(text has 'a', text has 'b', 3, any, ordered)
                                           ^^^^^^^
  hint: choose one of: any, ordered
```

## E28.14 `near` inside `line`

Fixture: `near`

```json request
{ "query": "line(near(text has 'a', text has 'b', 1))" }
```

```text error
error[nested_line]: near( ... ) inside line( ... )
  line(near(text has 'a', text has 'b', 1))
       ^^^^
  hint: line( ... ) can't contain near( ... )
```
