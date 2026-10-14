# Examples for task 23

Spec: `1`

## Fixtures

### words

```text tree
src/ids.rs                 2026-10-01 10:00
```

`src/ids.rs`:

```text
id
let valid = id_2;
let id2 = id;
user_id(id)
```

The fixture `content` is described in [19__EXAMPLES__TEXT_HAS.md](19__EXAMPLES__TEXT_HAS.md).

## E23.1 Whole words: at the line start and end, next to punctuation

`valid`, `id_2`, `id2` and `user_id` don't match: a letter, a digit or `_` is right before or after `id`.

Fixture: `words`

```json request
{ "query": "text has 'id' /w" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 text has 'id' /w

src/
ids.rs  [#1]  lines: 3  matches: 3
1:id
--
3:let id2 = ⟦#1:id⟧;
4:user_id(⟦#1:id⟧)
```

## E23.2 With `/i`

Fixture: `words`

```json request
{ "query": "text has 'ID' /i/w" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 text has 'ID' /i/w

src/
ids.rs  [#1]  lines: 3  matches: 3
1:id
--
3:let id2 = ⟦#1:id⟧;
4:user_id(⟦#1:id⟧)
```

## E23.3 With `=~`

Fixture: `words`

```json request
{ "query": "text =~ `id\\d?` /w" }
```

```text response
files: 1  lines: 3  matches: 4  result: r1
#1 text =~ `id\d?` /w

src/
ids.rs  [#1]  lines: 3  matches: 4
1:id
--
3:let ⟦#1:id2⟧ = ⟦#1:id⟧;
4:user_id(⟦#1:id⟧)
```

## E23.4 With `starts`

A line break after the match is not a word character.

Fixture: `words`

```json request
{ "query": "text starts 'id' /w" }
```

```text response
files: 1  lines: 1  matches: 1  result: r1
#1 text starts 'id' /w

src/
ids.rs  [#1]  lines: 1  matches: 1
1:id
```

## E23.5 Next to punctuation

Fixture: `content`

```json request
{ "query": "text has 'unwrap' /w" }
```

```text response
files: 2  lines: 3  matches: 4  result: r1
#1 text has 'unwrap' /w

src/
a.rs  [#1]  lines: 2  matches: 3
2:let x = foo.⟦#1:unwrap⟧();
3:let y = bar.⟦#1:unwrap⟧().baz.⟦#1:unwrap⟧();
b.rs  [#1]  lines: 1  matches: 1
2:let v = parse().⟦#1:unwrap⟧();
```

## E23.6 `/w` works only on `text`

Fixture: `words`

```json request
{ "query": "name has 'id' /w" }
```

```text error
error[modifier_not_allowed]: modifier '/w' is not allowed here
  name has 'id' /w
                ^^
  hint: '/w' works with '=', 'has', 'starts', 'ends', '=~' on 'text'
```
