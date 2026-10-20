# Plan of task 157: Paths relative to the only working directory in the write tools

## Design

The rule of task 148 (`engine::in_dirs`) decides, for the write tools too, whether the working directory is written
once: spec 2, `path_style = relative`, one working directory given as an absolute path, no `dir` in `show`. The
working directories are the ones of the selection: of the query run, or of the kept result for `result = "rN"`.

- `engine::Selected` gets the working directories of the selection; `try_replace` takes them from the run or the
  kept result the same way.
- `write::Outcome` gets `in_dir: Option<PathBuf>` (the working directory as given) and `in_line: bool` (`false` with
  the configured `omit = ["dir"]`). A new `Outcome::relative_to(dir)` is called by `write_output` after the tool acted:
  it strips the directory from `Acted::shown`, writes `Acted::to` relative to it (`..` for a target outside, a lexical
  difference of the normalized paths) and strips the warnings (`Warnings::strip_prefix`, as `find` does).
- The entries themselves keep their shown paths while the tool acts: `replace` finds the matches of a file by its
  shown path, the conflicts error (`rename_conflict`) writes the paths as before.
- `to_text`: the line `in: <dir>/` (`output::dir_name`) right after the header when there are entries or warnings
  with paths. `to_json`: `"dirs": ["<dir>/"]` before `files` under the same condition.

Rejected: stripping the prefix when the entries are built (`found_entries`, `result_entries`): `replace` keys its
matches by the shown path, the rename conflicts and the trash use them, so it touches more code for the same output.

## Tests

- examples E157.1–E157.9 (startup examples, fixture `one-dir` of task 148)
- unit `write::tests::one_working_directory_is_written_once`: text and JSON, a target outside with `..`, the warnings,
  no line for an empty outcome, `in_line = false`
- unit `write::tests::relative_targets` for the lexical difference helper

## Steps

1. red: the examples and the unit tests
2. `Outcome::relative_to`, the `in:` line and JSON `dirs`
3. the engine: the working directories of the selection, `in_dirs` for the write tools
4. clippy, fmt, the definition of done, summary, version `0.154.0`

## Risks

- the order of the entries with several working directories is unchanged (E157.8 pins it)
- Windows: `..` and the separator in the targets follow `display_path`
