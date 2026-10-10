# Examples for task 26

The fixture `content` is described in [19__EXAMPLES__TEXT_HAS.md](19__EXAMPLES__TEXT_HAS.md).

## E26.1 Two conditions on the same line

The fragments carry the id of the group and of the condition; the file lists only the group. `src/c.rs` has lines
with `let`, but none with `unwrap` too.

Fixture: `content`

```json request
{ "query": "line(text has 'let' and text has 'unwrap')" }
```

```text response
files: 2  lines: 3  matches: 7  result: r1
#1 line(text has 'let' and text has 'unwrap')
#2 text has 'let'
#3 text has 'unwrap'

src/
a.rs  [#1]  lines: 2  matches: 5
2:⟦#1,#2:let⟧ x = foo.⟦#1,#3:unwrap⟧();
3:⟦#1,#2:let⟧ y = bar.⟦#1,#3:unwrap⟧().baz.⟦#1,#3:unwrap⟧();
b.rs  [#1]  lines: 1  matches: 2
2:⟦#1,#2:let⟧ v = parse().⟦#1,#3:unwrap⟧();
```

## E26.2 The same conditions in file scope may match different lines

Fixture: `content`

```json request
{ "query": "text has 'fn' and text has 'println'" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text has 'fn'
#2 text has 'println'

src/
a.rs  [#1 #2]  lines: 2  matches: 2
1:⟦#1:fn⟧ main() {
--
4:⟦#2:println⟧!("done");
```

## E26.3 `not` inside a group: lines without a match

The group has no positive text condition, so every matched line is one fragment with the id of the group.

Fixture: `content`

```json request
{ "query": "name = 'c.rs' and line(not text has 'let')" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'c.rs'
#2 line(not text has 'let')
#3 text has 'let'

src/
c.rs  [#1 #2]  lines: 3  matches: 3
1:⟦#2:// Unwrap is only mentioned in this comment⟧
2:⟦#2:fn c() {⟧
--
8:⟦#2:}⟧
```

## E26.4 `not` before a group: no line matches

Fixture: `content`

```json request
{ "query": "name ~ '*.rs' and not line(text has 'unwrap')" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 line(text has 'unwrap')
#3 text has 'unwrap'

src/
c.rs  [#1]
```

## E26.5 Two independent groups

Fixture: `content`

```json request
{ "query": "line(text has 'fn') and line(text has 'println')" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 line(text has 'fn')
#2 text has 'fn'
#3 line(text has 'println')
#4 text has 'println'

src/
a.rs  [#1 #3]  lines: 2  matches: 2
1:⟦#1,#2:fn⟧ main() {
--
4:⟦#3,#4:println⟧!("done");
```

## E26.6 Labels inside a group

Fixture: `content`

```json request
{ "query": "line(text has 'unwrap(' @unwrap or text has 'parse(' @parse) @calls" }
```

```text response
files: 2  lines: 3  matches: 5  result: r1
@calls line(text has 'unwrap(' or text has 'parse(')
@unwrap text has 'unwrap('
@parse text has 'parse('

src/
a.rs  [@calls]  lines: 2  matches: 3
2:let x = foo.⟦@calls,@unwrap:unwrap(⟧);
3:let y = bar.⟦@calls,@unwrap:unwrap(⟧).baz.⟦@calls,@unwrap:unwrap(⟧);
b.rs  [@calls]  lines: 1  matches: 2
2:let v = ⟦@calls,@parse:parse(⟧).⟦@calls,@unwrap:unwrap(⟧);
```

## E26.7 A file target inside a group

Fixture: `content`

```json request
{ "query": "line(name ~ '*.rs')" }
```

```text error
error[file_target_inside]: 'name' inside line( ... )
  line(name ~ '*.rs')
       ^^^^
  hint: move 'name' out of line( ... )
```

## E26.8 Nested groups

Fixture: `content`

```json request
{ "query": "line(line(text has 'a'))" }
```

```text error
error[nested_line]: nested line( ... )
  line(line(text has 'a'))
       ^^^^
  hint: line( ... ) can't be nested
```
