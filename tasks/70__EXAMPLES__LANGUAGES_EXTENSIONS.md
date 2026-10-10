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
names = []

[languages.cpp]
extensions = ['cpp', 'cc', 'cxx', 'hpp', 'hh', 'hxx']
names = []

[languages.csharp]
extensions = ['cs']
names = []

[languages.java]
extensions = ['java']
names = []

[languages.kotlin]
extensions = ['kt', 'kts']
names = []

[languages.scala]
extensions = ['scala', 'sc']
names = []

[languages.go]
extensions = ['go']
names = []

[languages.swift]
extensions = ['swift']
names = []

[languages.dart]
extensions = ['dart']
names = []

[languages.groovy]
extensions = ['groovy', 'gradle']
names = []

[languages.javascript]
extensions = ['js', 'jsx', 'mjs', 'cjs']
names = []

[languages.typescript]
extensions = ['ts', 'tsx', 'mts', 'cts']
names = []

[languages.rust]
extensions = ['rs']
names = []

[languages.php]
extensions = ['php']
names = []

[languages.css]
extensions = ['css']
names = []

[languages.scss]
extensions = ['scss', 'less']
names = []

[languages.python]
extensions = ['py', 'pyi']
names = []

[languages.ruby]
extensions = ['rb']
names = ['Gemfile', 'Rakefile']

[languages.shell]
extensions = ['sh', 'bash', 'zsh']
names = []

[languages.perl]
extensions = ['pl', 'pm']
names = []

[languages.r]
extensions = ['r']
names = []

[languages.powershell]
extensions = ['ps1', 'psm1']
names = []

[languages.batch]
extensions = ['bat', 'cmd']
names = []

[languages.sql]
extensions = ['sql']
names = []

[languages.lua]
extensions = ['lua']
names = []

[languages.haskell]
extensions = ['hs']
names = []

[languages.clojure]
extensions = ['clj', 'cljs', 'cljc']
names = []

[languages.lisp]
extensions = ['lisp', 'el', 'scm']
names = []

[languages.html]
extensions = ['html', 'htm']
names = []

[languages.xml]
extensions = ['xml', 'svg', 'xsd', 'xsl']
names = []

[languages.vue]
extensions = ['vue']
names = []

[languages.markdown]
extensions = ['md', 'markdown']
names = []

[languages.yaml]
extensions = ['yml', 'yaml']
names = []

[languages.toml]
extensions = ['toml']
names = []

[languages.ini]
extensions = ['ini', 'cfg', 'conf']
names = []

[languages.properties]
extensions = ['properties']
names = []

[languages.dotenv]
extensions = ['env']
names = ['.env']

[languages.json]
extensions = ['json']
names = []

[languages.jsonc]
extensions = ['jsonc', 'json5']
names = []

[languages.dockerfile]
extensions = ['dockerfile']
names = ['Dockerfile']

[languages.makefile]
extensions = ['mk']
names = ['Makefile', 'GNUmakefile']
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
names = []    # was (not set)
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
# names =  (not set)    # was [], reset
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
