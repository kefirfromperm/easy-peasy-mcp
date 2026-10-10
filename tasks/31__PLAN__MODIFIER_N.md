# Plan for task 31: `/n` any naming style

## Design

- `query/naming.rs` (new): `words(value)` splits the value at spaces, `_`, `-`, `.` and case changes (a lower case
  letter or a digit before an upper case one; the last letter of an upper case run before a lower case one starts a
  new word), lower case; `pattern(value)` is the regex of the words joined by nothing or by one separator, the same
  between all words: `(?:parsequery|parse_query|parse-query|parse\.query|parse query)`; no anchors, no flags.
- `query/mod.rs` `compile_text`: with `/n` the escaped value is replaced by the naming pattern and the regex is
  case-insensitive, on `text` as well as on `path` and `name`; the anchors of `=`, `starts`, `ends` and `/w` work as
  before around the whole alternation.
- `query/subset.rs`: `/n` is supported; `description.rs`: `/n`.
- the validation (`/n` only with `=`, `has`, `starts`, `ends`, not with `/e`) exists since task 1.

No new dependencies.

## Tests

- examples E31.1–E31.8 (fixture `naming`)
- unit: `words` for every style, `HTTPServer`, `utf8Decode`, digits, an empty value; `/n` on `text`, `name`, `path`
  with each operator, `/n/w`, `/n` in line scope

## Steps

1. failing unit tests
2. naming module, compile, subset
3. description

## Risks

- a value without words (`'_'`) gives an empty pattern, which matches like `has ''`
