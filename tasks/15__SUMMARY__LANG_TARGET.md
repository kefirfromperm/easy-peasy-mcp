# Summary of task 15: `lang` target

Server version: `0.15.0`.

## What was built

- `lang` with `=`, `!=`, `in ( ... )`, values in any case (QUERY.md#lang), by the built-in table of
  CONFIG.md#built-in-languages (41 languages).
- Detection, now written into CONFIG.md#languages: the full file name, then the extension (after the last `.`,
  case-insensitive), then — only for files without an extension — the shebang of the first line: the program or
  the first argument of `env`, exact or without a trailing version (`python3.11` → `python`). Other files and
  directories are `unknown`.
- `unknown_value` for a value which is neither a language nor `unknown`, with `did you mean` and the full list.
- The `find` description shows `lang`.

## Architecture

- `languages.rs`: `Language`, `LanguageTable` (lookup maps by name, extension, interpreter) with `detect`, which
  reads the first line only when a shebang is needed; the comment syntax is left for `/c` (task 29).
- `Filter::Lang`, `FileValues.lang`, `Needs.lang`; `ParseOptions` and `SearchOptions` share the table through an
  `Arc` held by the engine (the configuration tasks 70–77 replace it).
- `compile_node` dispatches to `compile_size`, `compile_date`, `compile_text`.

## Deviations from the plan

- `compile_node` was split to stay under the function length lint.

## Tests

- examples E15.1–E15.9 (fixture `langs`)
- unit: detection by name, extension, shebang forms (`env`, `env -S`, versions, spaces after `#!`), files with an
  extension are not read; `Filter::Lang`; `unknown_value` for languages

## Limitations and known gaps

- the table can't be changed yet (tasks 70–77)

## Try it

`{ "query": "lang = rust" }`
