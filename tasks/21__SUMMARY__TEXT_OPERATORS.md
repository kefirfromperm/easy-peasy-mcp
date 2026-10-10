# Summary of task 21: `text` with `=`, `starts`, `ends`

Server version: `0.21.0`.

## What was built

- `text =`, `text starts`, `text ends` in file scope on the whole content (QUERY.md#text-operators), with `/i`; the
  fragment is the matched part and may span lines; `\r\n` files work like `\n` ones (`text ends '\n'`).
- `operator_not_allowed` for `~` / `~~` on `text` with the spec hint.
- The `find` description shows the operators.

## Architecture

The content regex of `Filter::Content` gets the anchors `\A` / `\z` of the regex crate for the whole text.

## Deviations from the plan

None.

## Tests

- examples E21.1–E21.6 (fixture `scripts`)
- unit: whole content, start, end with and without a final line break, CRLF, `/i`

## Limitations and known gaps

None new.

## Try it

`{ "query": "text starts '#!/bin/bash'" }`
