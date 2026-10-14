# Examples for task 22

Spec: `1`

The fixture `content` is described in [19__EXAMPLES__TEXT_HAS.md](19__EXAMPLES__TEXT_HAS.md).

## E22.1 A regex per line, `^` is the start of the line

Fixture: `content`

```json request
{ "query": "text =~ `^let \\w+ = \\w+\\.unwrap`" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text =~ `^let \w+ = \w+\.unwrap`

src/
a.rs  [#1]  lines: 2  matches: 2
2:⟦#1:let x = foo.unwrap⟧();
3:⟦#1:let y = bar.unwrap⟧().baz.unwrap();
```

## E22.2 Several matches on a line

Fixture: `content`

```json request
{ "query": "text =~ `\\bunwrap\\(\\)`" }
```

```text response
files: 2  lines: 3  matches: 4  result: r1
#1 text =~ `\bunwrap\(\)`

src/
a.rs  [#1]  lines: 2  matches: 3
2:let x = foo.⟦#1:unwrap()⟧;
3:let y = bar.⟦#1:unwrap()⟧.baz.⟦#1:unwrap()⟧;
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().⟦#1:unwrap()⟧;
```

## E22.3 `$` is the end of the line

Fixture: `content`

```json request
{ "query": "text =~ `\\);$`" }
```

```text response
files: 2  lines: 4  matches: 4  result: r1
#1 text =~ `\);$`

src/
a.rs  [#1]  lines: 3  matches: 3
2:let x = foo.unwrap(⟦#1:);⟧
3:let y = bar.unwrap().baz.unwrap(⟦#1:);⟧
4:println!("done"⟦#1:);⟧
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().unwrap(⟦#1:);⟧
```

## E22.4 On `name`

Fixture: `content`

```json request
{ "query": "name =~ `^[a-c]\\.rs$`" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name =~ `^[a-c]\.rs$`

src/
a.rs  [#1]
b.rs  [#1]
c.rs  [#1]
```

## E22.5 On `path`

Fixture: `content`

```json request
{ "query": "path =~ `^src/.*\\.txt$`" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 path =~ `^src/.*\.txt$`

src/
crlf.txt  [#1]
```

## E22.6 With `/i`

Fixture: `content`

```json request
{ "query": "text =~ `UNWRAP\\(\\)` /i" }
```

```text response
files: 2  lines: 3  matches: 4  result: r1
#1 text =~ `UNWRAP\(\)` /i

src/
a.rs  [#1]  lines: 2  matches: 3
2:let x = foo.⟦#1:unwrap()⟧;
3:let y = bar.⟦#1:unwrap()⟧.baz.⟦#1:unwrap()⟧;
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().⟦#1:unwrap()⟧;
```

## E22.7 A quoted string keeps unknown escapes

`'\b'` is not an escape of quoted strings, so the regex gets `\b`.

Fixture: `content`

```json request
{ "query": "text =~ '\\bfoo\\b'" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text =~ '\bfoo\b'

src/
a.rs  [#1]  lines: 1  matches: 1
2:let x = ⟦#1:foo⟧.unwrap();
```

## E22.8 Look-arounds are not supported

Fixture: `content`

```json request
{ "query": "text =~ `(?<=a)b`" }
```

```text error
error[invalid_regex]: invalid regex
  text =~ `(?<=a)b`
          ^^^^^^^^^
  hint: look-arounds are not supported
```

## E22.9 Backreferences are not supported

Fixture: `content`

```json request
{ "query": "text =~ `(a)\\1`" }
```

```text error
error[invalid_regex]: invalid regex
  text =~ `(a)\1`
          ^^^^^^^
  hint: backreferences are not supported
```

## E22.10 Other errors name the problem

Fixture: `content`

```json request
{ "query": "name =~ `(unclosed`" }
```

```text error
error[invalid_regex]: invalid regex
  name =~ `(unclosed`
          ^^^^^^^^^^^
  hint: unclosed group
```
