# Examples for task 7

The fixture `names` is described in [3__EXAMPLES__NAME_OPERATORS.md](3__EXAMPLES__NAME_OPERATORS.md).

## E7.1 Labels show which branch of `or` matched

Fixture: `names`

```json request
{ "query": "name = 'config.rs' @cfg or name = 'app_config.rs' @app" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
@cfg name = 'config.rs'
@app name = 'app_config.rs'

lib/
app_config.rs  [@app]
config.rs  [@cfg]
```

## E7.2 Labelled and numbered conditions

Only conditions without a label are numbered; the legend is in the order of appearance.

Fixture: `names`

```json request
{ "query": "name has 'config' @c and (name starts 'app' or name ends 'loader.rs')" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
@c name has 'config'
#1 name starts 'app'
#2 name ends 'loader.rs'

lib/
app_config.rs  [@c #1]
config_loader.rs  [@c #2]
```

## E7.3 Numbers have no gaps

Fixture: `names`

```json request
{ "query": "name = 'x.rs' @x or name = 'config.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
@x name = 'x.rs'
#1 name = 'config.rs'

lib/
config.rs  [#1]
```

## E7.4 Label characters: letters, digits, `_` and `-`

The label comes after the modifiers.

Fixture: `names`

```json request
{ "query": "name = 'config.rs' /i @_cfg-2" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
@_cfg-2 name = 'config.rs' /i

lib/
config.rs  [@_cfg-2]
```

## E7.5 Duplicate label

Fixture: `names`

```json request
{ "query": "name = 'a' @x or name = 'b' @x" }
```

```text error
error[duplicate_label]: duplicate label '@x'
  name = 'a' @x or name = 'b' @x
                              ^^
  hint: labels must be unique
```

## E7.6 Invalid label

Fixture: `names`

```json request
{ "query": "name = 'a' @1x" }
```

```text error
error[unexpected_token]: unexpected '@'
  name = 'a' @1x
             ^
  hint: a label starts with a letter or '_', e.g. @name
```
