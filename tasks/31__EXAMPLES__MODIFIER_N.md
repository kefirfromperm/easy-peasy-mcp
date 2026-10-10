# Examples for task 31

## Fixtures

### naming

```text tree
src/codec.rs               2026-10-01 10:00
src/ConfigLoader.java      2026-10-01 10:00
src/config-loader.ts       2026-10-01 10:00
src/config_loader.rs       2026-10-01 10:00
src/loader_config.rs       2026-10-01 10:00
src/server.go              2026-10-01 10:00
src/styles.txt             2026-10-01 10:00
src/user.ts                2026-10-01 10:00
```

File contents are in `tests/fixtures/naming/`. `src/styles.txt`:

```text
parseQuery
ParseQuery
parse_query
PARSE_QUERY
parse-query
parse.query
parse query
parsequery
parse__query
reparse_query_all
```

`src/server.go` (the second line starts with a tab):

```text
func NewHTTPServer() *HttpServer {
	return &http_server{}
}
```

`src/codec.rs`:

```text
fn utf8_decode(b: &[u8]) {}
let s = Utf8Decode::new();
let t = utf8decode(x);
let u = utf_8_decode(x);
```

`src/user.ts`:

```text
const userId = 1;
const user_id = 2;
const userIdentity = 3;
const myUserId = 4;
```

`src/ConfigLoader.java`, `src/config-loader.ts`, `src/config_loader.rs` and `src/loader_config.rs` have one line each.
Times are UTC.

## E31.1 Every naming style

Line 9 has two separators between the words; line 10 has `parse_query` inside a longer identifier.

Fixture: `naming`

```json request
{ "query": "name = 'styles.txt' and text has 'parse query' /n" }
```

```text response
files: 1  lines: 9  matches: 9  result: r1
#1 name = 'styles.txt'
#2 text has 'parse query' /n

src/
styles.txt  [#1 #2]  lines: 9  matches: 9
1:⟦#2:parseQuery⟧
2:⟦#2:ParseQuery⟧
3:⟦#2:parse_query⟧
4:⟦#2:PARSE_QUERY⟧
5:⟦#2:parse-query⟧
6:⟦#2:parse.query⟧
7:⟦#2:parse query⟧
8:⟦#2:parsequery⟧
--
10:re⟦#2:parse_query⟧_all
```

## E31.2 The value in another style, a whole line

Fixture: `naming`

```json request
{ "query": "name = 'styles.txt' and line(text = 'PARSE_QUERY' /n)" }
```

```text response
files: 1  lines: 8  matches: 8  result: r1
#1 name = 'styles.txt'
#2 line(text = 'PARSE_QUERY' /n)
#3 text = 'PARSE_QUERY' /n

src/
styles.txt  [#1 #2]  lines: 8  matches: 8
1:⟦#2,#3:parseQuery⟧
2:⟦#2,#3:ParseQuery⟧
3:⟦#2,#3:parse_query⟧
4:⟦#2,#3:PARSE_QUERY⟧
5:⟦#2,#3:parse-query⟧
6:⟦#2,#3:parse.query⟧
7:⟦#2,#3:parse query⟧
8:⟦#2,#3:parsequery⟧
```

## E31.3 An uppercase run is one word

`HTTPServer` is the words `http` and `server`.

Fixture: `naming`

```json request
{ "query": "name = 'server.go' and text has 'HTTPServer' /n" }
```

```text response
files: 1  lines: 2  matches: 3  result: r1
#1 name = 'server.go'
#2 text has 'HTTPServer' /n

src/
server.go  [#1 #2]  lines: 2  matches: 3
1:func New⟦#2:HTTPServer⟧() *⟦#2:HttpServer⟧ {
2:	return &⟦#2:http_server⟧{}
```

## E31.4 Digits stay with the preceding word

`utf8Decode` is the words `utf8` and `decode`, so `utf_8_decode` is three words.

Fixture: `naming`

```json request
{ "query": "name = 'codec.rs' and text has 'utf8Decode' /n" }
```

```text response
files: 1  lines: 3  matches: 3  result: r1
#1 name = 'codec.rs'
#2 text has 'utf8Decode' /n

src/
codec.rs  [#1 #2]  lines: 3  matches: 3
1:fn ⟦#2:utf8_decode⟧(b: &[u8]) {}
2:let s = ⟦#2:Utf8Decode⟧::new();
3:let t = ⟦#2:utf8decode⟧(x);
```

## E31.5 `/n/w` matches the whole identifier

Fixture: `naming`

```json request
{ "query": "name = 'user.ts' and text has 'user id' /n/w" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 name = 'user.ts'
#2 text has 'user id' /n/w

src/
user.ts  [#1 #2]  lines: 2  matches: 2
1:const ⟦#2:userId⟧ = 1;
2:const ⟦#2:user_id⟧ = 2;
```

## E31.6 File names in any style

Fixture: `naming`

```json request
{ "query": "name has 'config loader' /n" }
```

```text response
files: 3  lines: 0  matches: 0  result: r1
#1 name has 'config loader' /n

src/
ConfigLoader.java  [#1]
config-loader.ts  [#1]
config_loader.rs  [#1]
```

## E31.7 `/n` with a wildcard

Fixture: `naming`

```json request
{ "query": "name ~ '*loader*' /n" }
```

```text error
error[modifier_not_allowed]: modifier '/n' is not allowed here
  name ~ '*loader*' /n
                    ^^
  hint: '/n' works with '=', 'has', 'starts', 'ends'
```

## E31.8 `/n` with `/e`

Fixture: `naming`

```json request
{ "query": "text has 'a b' /n /e(xml, 1)" }
```

```text error
error[modifier_not_allowed]: modifier '/e' is not allowed here
  text has 'a b' /n /e(xml, 1)
                    ^^
  hint: '/e' can't be combined with '/n'
```
