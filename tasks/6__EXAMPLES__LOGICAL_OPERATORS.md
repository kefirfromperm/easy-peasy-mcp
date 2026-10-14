# Examples for task 6

Spec: `1`

The fixture `names` is described in [3__EXAMPLES__NAME_OPERATORS.md](3__EXAMPLES__NAME_OPERATORS.md).

## E6.1 `and`: all parts contribute

Fixture: `names`

```json request
{ "query": "name has 'config' and name ends '.rs'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name has 'config'
#2 name ends '.rs'

lib/
app_config.rs  [#1 #2]
config.rs  [#1 #2]
config_loader.rs  [#1 #2]
```

## E6.2 `or`: only the true branch contributes

Fixture: `names`

```json request
{ "query": "name = 'config.rs' or name = 'app_config.rs'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'config.rs'
#2 name = 'app_config.rs'

lib/
app_config.rs  [#2]
config.rs  [#1]
```

## E6.3 `not`: nothing under it contributes

The legend lists every condition, `#2` too.

Fixture: `names`

```json request
{ "query": "name ends '.rs' and not name has 'config'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ends '.rs'
#2 name has 'config'

lib/
what's_new.rs  [#1]
```

## E6.4 `and` binds tighter than `or`

The query is `name = 'config.rs' or (name has 'config' and name starts 'app')`; `lib/config_loader.rs` matches only `#2`.

Fixture: `names`

```json request
{ "query": "name = 'config.rs' or name has 'config' and name starts 'app'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'config.rs'
#2 name has 'config'
#3 name starts 'app'

lib/
app_config.rs  [#2 #3]
config.rs  [#1]
```

## E6.5 Brackets

Fixture: `names`

```json request
{ "query": "(name = 'config.rs' or name has 'loader') and name ends '.rs'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name = 'config.rs'
#2 name has 'loader'
#3 name ends '.rs'

lib/
config.rs  [#1 #3]
config_loader.rs  [#2 #3]
```

## E6.6 `not not`

The file matches only through `not`, so no condition is listed.

Fixture: `names`

```json request
{ "query": "not not name = 'config.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name = 'config.rs'

lib/
config.rs
```

## E6.7 Chained `xor`: an odd number of true parts

`lib/app_config.rs` has three true parts, `lib/what's_new.rs` one; `lib/config.rs` has two and doesn't match.

Fixture: `names`

```json request
{ "query": "name has 'config' xor name ends '.rs' xor name starts 'app'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name has 'config'
#2 name ends '.rs'
#3 name starts 'app'

lib/
app_config.rs  [#1 #2 #3]
what's_new.rs  [#2]
```

## E6.8 Keywords in any case, line breaks between conditions

Fixture: `names`

```json request
{ "query": "name ends '.rs'\nAND NOT\nname has '_'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ends '.rs'
#2 name has '_'

lib/
config.rs  [#1]
```

## E6.9 Missing operand

Fixture: `names`

```json request
{ "query": "name = 'a' and" }
```

```text error
error[unexpected_end]: unexpected end of the query
  name = 'a' and
                ^
  hint: a condition is expected after 'and'
```

## E6.10 Missing `)`

Fixture: `names`

```json request
{ "query": "(name = 'a' or name = 'b'" }
```

```text error
error[unexpected_end]: unexpected end of the query
  (name = 'a' or name = 'b'
                           ^
  hint: closing ')' is missing
```

## E6.11 Two conditions without an operator

Fixture: `names`

```json request
{ "query": "name = 'a' name = 'b'" }
```

```text error
error[unexpected_token]: unexpected 'name'
  name = 'a' name = 'b'
             ^^^^
  hint: missing 'and' / 'or' between conditions?
```

## E6.12 `not` without a condition

Fixture: `names`

```json request
{ "query": "name = 'a' or not" }
```

```text error
error[unexpected_end]: unexpected end of the query
  name = 'a' or not
                   ^
  hint: a condition is expected after 'not'
```
