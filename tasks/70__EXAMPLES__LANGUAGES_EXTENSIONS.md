# Examples for task 70

Examples with a command line are startup examples (AGENTS.md).

## Fixtures

### languages

```text tree
app.tpl                    2026-10-01 10:00
main.tf                    2026-10-01 10:00
src/main.rs                2026-10-01 10:00
vars.tfvars                2026-10-01 10:00
```

File contents are in `tests/fixtures/languages/`: Terraform files, an HTML template and a Rust file, one or two lines
each. Times are UTC.

## E70.1 The built-in languages

`.tf`, `.tfvars` and `.tpl` are not in the built-in table.

Fixture: `languages`

1. `find`

```json request
{"query": "lang = unknown"}
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 lang = unknown

./
app.tpl  [#1]
main.tf  [#1]
vars.tfvars  [#1]
```

2. `get_config`

```json request get_config
{ "section": "languages" }
```

```text response
# easy-peasy-mcp <version>, spec 1, supported specs: 1
# read-only: server.allowed_roots, server.allow_update

[languages.c]
extensions = ['c', 'h']

[languages.cpp]
extensions = ['cpp', 'cc', 'cxx', 'hpp', 'hh', 'hxx']

[languages.csharp]
extensions = ['cs']

[languages.java]
extensions = ['java']

[languages.kotlin]
extensions = ['kt', 'kts']

[languages.scala]
extensions = ['scala', 'sc']

[languages.go]
extensions = ['go']

[languages.swift]
extensions = ['swift']

[languages.dart]
extensions = ['dart']

[languages.groovy]
extensions = ['groovy', 'gradle']

[languages.javascript]
extensions = ['js', 'jsx', 'mjs', 'cjs']

[languages.typescript]
extensions = ['ts', 'tsx', 'mts', 'cts']

[languages.rust]
extensions = ['rs']

[languages.php]
extensions = ['php']

[languages.css]
extensions = ['css']

[languages.scss]
extensions = ['scss', 'less']

[languages.python]
extensions = ['py', 'pyi']

[languages.ruby]
extensions = ['rb']

[languages.shell]
extensions = ['sh', 'bash', 'zsh']

[languages.perl]
extensions = ['pl', 'pm']

[languages.r]
extensions = ['r']

[languages.powershell]
extensions = ['ps1', 'psm1']

[languages.batch]
extensions = ['bat', 'cmd']

[languages.sql]
extensions = ['sql']

[languages.lua]
extensions = ['lua']

[languages.haskell]
extensions = ['hs']

[languages.clojure]
extensions = ['clj', 'cljs', 'cljc']

[languages.lisp]
extensions = ['lisp', 'el', 'scm']

[languages.html]
extensions = ['html', 'htm']

[languages.xml]
extensions = ['xml', 'svg', 'xsd', 'xsl']

[languages.vue]
extensions = ['vue']

[languages.markdown]
extensions = ['md', 'markdown']

[languages.yaml]
extensions = ['yml', 'yaml']

[languages.toml]
extensions = ['toml']

[languages.ini]
extensions = ['ini', 'cfg', 'conf']

[languages.properties]
extensions = ['properties']

[languages.dotenv]
extensions = ['env']

[languages.json]
extensions = ['json']

[languages.jsonc]
extensions = ['jsonc', 'json5']

[languages.dockerfile]
extensions = ['dockerfile']

[languages.makefile]
extensions = ['mk']
```

## E70.2 A new language in a request

Fixture: `languages`

```json request
{"query": "lang = terraform", "languages": {"terraform": {"extensions": ["tf", "tfvars"]}}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = terraform

./
main.tf  [#1]
vars.tfvars  [#1]
```

## E70.3 A built-in language replaced in a request

Fixture: `languages`

```json request
{"query": "lang = html", "languages": {"html": {"extensions": ["html", "htm", "tpl"]}}}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = html

./
app.tpl  [#1]
```

## E70.4 `kind = source` uses the languages

A new language is code unless it says otherwise.

Fixture: `languages`

```json request
{"query": "kind = source", "languages": {"terraform": {"extensions": ["TF"]}}}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 kind = source

./
main.tf  [#1]
src/
main.rs  [#1]
```

## E70.5 An extension of another language

Fixture: `languages`

```json request
{"query": "lang = terraform", "languages": {"terraform": {"extensions": ["tf", "RS"]}}}
```

```text error
error[invalid_option]: the extension 'RS' of 'languages.terraform.extensions' is also in 'languages.rust'
```

## E70.6 A language for the session, then reset

Fixture: `languages`

1. `update_config`

```json request update_config
{"set": "[languages.terraform]\nextensions = [\"tf\", \"tfvars\"]"}
```

```text response
[languages.terraform]
extensions = ['tf', 'tfvars']    # was (not set)
```

2. `find`

```json request
{"query": "lang = terraform"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = terraform

./
main.tf  [#1]
vars.tfvars  [#1]
```

3. `update_config`

```json request update_config
{"reset": ["languages.terraform"]}
```

```text response
[languages.terraform]
# extensions =  (not set)    # was ['tf', 'tfvars'], reset
```

## E70.7 A built-in language for the session

Fixture: `languages`

1. `update_config`

```json request update_config
{"set": "languages.html.extensions = [\"html\", \"htm\", \"tpl\"]"}
```

```text response
[languages.html]
extensions = ['html', 'htm', 'tpl']    # was ['html', 'htm']
```

2. `find`

```json request
{"query": "lang = html"}
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 lang = html

./
app.tpl  [#1]
```

## E70.8 An invalid value in a request

Fixture: `languages`

```json request
{"languages": {"terraform": {"extensions": "tf"}}}
```

```text error
error[invalid_option]: 'languages.terraform.extensions' must be a list of strings
```

## E70.9 An invalid language name in a request

Fixture: `languages`

```json request
{"languages": {"c++": {"extensions": ["cxx2"]}}}
```

```text error
error[invalid_option]: invalid language name 'c++'
  hint: a letter, then letters, digits, _ and - are expected
```

## E70.10 An invalid value for `update_config`

Fixture: `languages`

```json request update_config
{"set": "[languages.terraform]\nextensions = [\"tf\", \"rs\"]"}
```

```text error
error[invalid_value]: invalid value for 'languages.terraform.extensions'
  hint: the extension 'rs' is also in 'languages.rust'
```

## E70.11 From the config file

Fixture: `languages`

```toml conf/l.toml
[languages.terraform]
extensions = ["tf", "tfvars"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```json request
{"query": "lang = terraform"}
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 lang = terraform

./
main.tf  [#1]
vars.tfvars  [#1]
```

## E70.12 An invalid value in the config file

Fixture: `languages`

```toml conf/l.toml
[languages.terraform]
extensions = ["tf", "rs"]
```

```text command
easy-peasy-mcp --config conf/l.toml
```

```text stderr exit 1
easy-peasy-mcp: conf/l.toml:2: invalid value for 'languages.terraform.extensions': the extension 'rs' is also in 'languages.rust'
```
