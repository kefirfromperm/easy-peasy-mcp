# Examples for task 3

## Fixtures

### names

```text tree
config/settings.toml       2026-10-01 10:00
docs/Readme.md             2026-10-01 10:00
docs/äpfel.md              2026-10-01 10:00
lib/README.md              2026-10-01 10:00
lib/Äpfel.md               2026-10-01 10:00
lib/app_config.rs          2026-10-01 10:00
lib/config.rs              2026-10-01 10:00
lib/config_loader.rs       2026-10-01 10:00
lib/utils.test.ts          2026-10-01 10:00
lib/what's_new.rs          2026-10-01 10:00
```

File contents are in `tests/fixtures/names/`. Times are UTC. No two names differ only in case, so the fixture
works on case-insensitive file systems.

## E3.1 `has`: the value at the start, in the middle and at the end of the name

The directory `config/` matches too, but directories are not returned.

Fixture: `names`

```json request
{ "query": "name has 'config'" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name has 'config'

lib/
app_config.rs  [#1]
config.rs  [#1]
config_loader.rs  [#1]
```

## E3.2 `has` at the end of the name

Fixture: `names`

```json request
{ "query": "name has '.rs'" }
```

```text response
files: 4  lines: 0  matches: 0  result: r1
#1 name has '.rs'

lib/
app_config.rs  [#1]
config.rs  [#1]
config_loader.rs  [#1]
what's_new.rs  [#1]
```

## E3.3 `starts`

Fixture: `names`

```json request
{ "query": "name starts 'config'" }
```

```text response
files: 2  lines: 0  matches: 0  result: r1
#1 name starts 'config'

lib/
config.rs  [#1]
config_loader.rs  [#1]
```

## E3.4 `ends`

Fixture: `names`

```json request
{ "query": "name ends '_config.rs'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ends '_config.rs'

lib/
app_config.rs  [#1]
```

## E3.5 `ends` with several dots

Fixture: `names`

```json request
{ "query": "name ends '.test.ts'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name ends '.test.ts'

lib/
utils.test.ts  [#1]
```

## E3.6 A value with an escape

Fixture: `names`

```json request
{ "query": "name starts 'what\\'s'" }
```

```text response
files: 1  lines: 0  matches: 0  result: r1
#1 name starts 'what\'s'

lib/
what's_new.rs  [#1]
```
