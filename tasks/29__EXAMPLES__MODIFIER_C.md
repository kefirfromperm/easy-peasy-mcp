# Examples for task 29

Spec: `1`

## Fixtures

### comments

```text tree
src/app.py                 2026-10-01 10:00
src/lib.rs                 2026-10-01 10:00
src/notes.txt              2026-10-01 10:00
src/only.rs                2026-10-01 10:00
```

File contents are in `tests/fixtures/comments/`. `src/lib.rs`:

```text
// let x = foo.unwrap();
let y = bar.unwrap(); // ok
/* .unwrap() */
let s = "// .unwrap()";
/* outer /* inner */ still .unwrap() */
let z = baz.unwrap();
```

`src/app.py`:

```text
print("#not a comment")  # TODO later
# TODO first
x = "TODO in a string"
```

`src/only.rs` has `.unwrap()` only in a comment; `src/notes.txt` (no language) has `// unwrap here`. Times are UTC.

## E29.1 Matches in Rust comments are ignored

Line 2 matches before its comment; in line 4 `//` is inside a string; line 5 is one nested block comment.

Fixture: `comments`

```json request
{ "query": "name = 'lib.rs' and text has '.unwrap()' /c" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'lib.rs'
#2 text has '.unwrap()' /c

src/
lib.rs  [#1 #2]  lines: 3  matches: 3
2:let y = bar⟦#2:.unwrap()⟧; // ok
--
4:let s = "// ⟦#2:.unwrap()⟧";
--
6:let z = baz⟦#2:.unwrap()⟧;
```

## E29.2 A Python `#` comment and a `#` in a string

Fixture: `comments`

```json request
{ "query": "name = 'app.py' and text has 'TODO' /c" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'app.py'
#2 text has 'TODO' /c

src/
app.py  [#1 #2]  lines: 1  matches: 1
3:x = "⟦#2:TODO⟧ in a string"
```

## E29.3 The comment marker belongs to the comment

Fixture: `comments`

```json request
{ "query": "name = 'app.py' and text has '#' /c" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'app.py'
#2 text has '#' /c

src/
app.py  [#1 #2]  lines: 1  matches: 1
1:print("⟦#2:#⟧not a comment")  # TODO later
```

## E29.4 Files where a word appears only in comments

Fixture: `comments`

```json request
{ "query": "name ~ '*.rs' and not text has 'unwrap' /c" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ~ '*.rs'
#2 text has 'unwrap' /c

src/
only.rs  [#1]
```

## E29.5 A file of an unknown language is searched as is

Fixture: `comments`

```json request
{ "query": "name = 'notes.txt' and text has 'unwrap' /c" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 name = 'notes.txt'
#2 text has 'unwrap' /c

src/
notes.txt  [#1 #2]  lines: 1  matches: 1
1:// ⟦#2:unwrap⟧ here
```

## E29.6 `/c` inside a line group

Line 1 has `let` and `unwrap` only inside its comment.

Fixture: `comments`

```json request
{ "query": "line(text has 'let' and text has 'unwrap' /c)" }
```

```text response
files: 1  lines: 3  matches: 6  result: r1
#1 line(text has 'let' and text has 'unwrap' /c)
#2 text has 'let'
#3 text has 'unwrap' /c

src/
lib.rs  [#1]  lines: 3  matches: 6
2:⟦#1,#2:let⟧ y = bar.⟦#1,#3:unwrap⟧(); // ok
--
4:⟦#1,#2:let⟧ s = "// .⟦#1,#3:unwrap⟧()";
--
6:⟦#1,#2:let⟧ z = baz.⟦#1,#3:unwrap⟧();
```
