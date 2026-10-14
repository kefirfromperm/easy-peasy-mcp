# Examples for task 24

Spec: `1`

## Fixtures

### tests_attr

```text tree
src/eval.rs                2026-10-01 10:00
```

`src/eval.rs`:

```text
#[test]
fn eval_fast() {
}

#[test]
fn eval_big_input_slow() {
}
```

## E24.1 A test attribute and the function after it

Every line of a multiline match is a matched line.

Fixture: `tests_attr`

```json request
{ "query": "text =~ `#\\[test\\]\\s*\\n\\s*fn \\w+_slow` /m" }
```

```text response
files: 1  lines: 2  matches: 1  result: r1
#1 text =~ `#\[test\]\s*\n\s*fn \w+_slow` /m

src/
eval.rs  [#1]  lines: 2  matches: 1
5:⟦#1:#[test]
6:fn eval_big_input_slow⟧() {
```

## E24.2 `(?s)`: `.` matches line breaks too

Fixture: `tests_attr`

```json request
{ "query": "text =~ `(?s)fast.*big` /m" }
```

```text response
files: 1  lines: 5  matches: 1  result: r1
#1 text =~ `(?s)fast.*big` /m

src/
eval.rs  [#1]  lines: 5  matches: 1
2:fn eval_⟦#1:fast() {
3:}
4:
5:#[test]
6:fn eval_big⟧_input_slow() {
```

## E24.3 `^` and `$` are line starts and ends

Fixture: `tests_attr`

```json request
{ "query": "text =~ `^fn \\w+\\(\\) \\{$` /m" }
```

```text response
files: 1  lines: 2  matches: 2  result: r1
#1 text =~ `^fn \w+\(\) \{$` /m

src/
eval.rs  [#1]  lines: 2  matches: 2
2:⟦#1:fn eval_fast() {⟧
--
6:⟦#1:fn eval_big_input_slow() {⟧
```

## E24.4 `/m` works only with `=~` on `text`

Fixture: `tests_attr`

```json request
{ "query": "name =~ `eval` /m" }
```

```text error
error[modifier_not_allowed]: modifier '/m' is not allowed here
  name =~ `eval` /m
                 ^^
  hint: '/m' works with '=~' on 'text' in file scope
```
