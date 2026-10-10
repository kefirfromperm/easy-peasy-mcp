# Examples for task 5

The fixture `names` is described in [3__EXAMPLES__NAME_OPERATORS.md](3__EXAMPLES__NAME_OPERATORS.md).

## E5.1 `=` with `/i`

Fixture: `names`

```json request
{ "query": "name = 'readme.md' /i" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'readme.md' /i

docs/
Readme.md  [#1]
lib/
README.md  [#1]
```

## E5.2 `has` with `/i` and non-ASCII letters

`/i` uses Unicode simple case folding: `Ä` and `ä` are the same letter.

Fixture: `names`

```json request
{ "query": "name has 'äpfel' /i" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'äpfel' /i

docs/
äpfel.md  [#1]
lib/
Äpfel.md  [#1]
```

## E5.3 `~` with `/i`

Fixture: `names`

```json request
{ "query": "name ~ 'READ*' /i" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name ~ 'READ*' /i

docs/
Readme.md  [#1]
lib/
README.md  [#1]
```

## E5.4 `path ends` with `/i`

Fixture: `names`

```json request
{ "query": "path ends '/ÄPFEL.MD' /i" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 path ends '/ÄPFEL.MD' /i

docs/
äpfel.md  [#1]
lib/
Äpfel.md  [#1]
```

## E5.5 A space between the value and the modifier is optional

Fixture: `names`

```json request
{ "query": "name = 'README.MD'/i" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'README.MD'/i

docs/
Readme.md  [#1]
lib/
README.md  [#1]
```

## E5.6 Unknown modifier

Fixture: `names`

```json request
{ "query": "name = 'a' /x" }
```

```text error
error[unexpected_token]: unexpected '/x'
  name = 'a' /x
             ^^
  hint: modifiers: /i, /w, /m, /c, /n, /e(style, levels)
```

## E5.7 Modifier not allowed

Fixture: `names`

```json request
{ "query": "name = 'a' /m" }
```

```text error
error[modifier_not_allowed]: modifier '/m' is not allowed here
  name = 'a' /m
             ^^
  hint: '/m' works with '=~' on 'text' in file scope
```
