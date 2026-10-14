# Task 140: The write tools say how to select directories and symlinks

Status: in progress

Depends on: task 125

## Goal

An agent which wants to delete a symlink or a directory finds out how from the description of the tool. The query of
a write tool selects files only, like `find`: without a `type` condition an implicit `type = file` is added
([QUERY.md#files-and-directories](../QUERY.md#files-and-directories)), so `name ~ '*.tmp'` doesn't select a symlink
`link.tmp`, and `type = link` does. The [description of `delete`](../TOOLS.md#delete) says what happens to a directory
and to a symlink, but not how to select them:

```
A directory is deleted with everything in it. Symlinks are deleted themselves, never followed.
```

Found by trying the server: `delete` with `query = "name ~ '*.tmp'"` left out the symlink `link.tmp`, which
`find -name '*.tmp'` lists; the agent turned on `traversal.follow_symlinks` to select it, the wrong tool for the job
(it also reads the targets of every other symlink), instead of `type = link`.

## Scope

A spec change first (its own commit; TOOLS.md, the description of `delete`), then the code.

- the description of `delete` says that the query selects files only, and names `type = dir` and `type = link`, in
  the line about directories and symlinks; the description stays as short as it is now give or take a line
- the wording serves the next write tools (`replace`, task 129: it acts on files only, the line may differ)
- maybe: a hint in the response of a write tool whose selection is empty but whose query without the implicit
  `type = file` would select entries (`no files selected; 1 symlink matches: add type = link`); decided in the spec
  step, with its cost (a second evaluation) and its `omit` part (**every part of the output is optional**)

## Out of scope

- the implicit `type = file` itself
- the description of `find`
- what a write tool does with a symlink or a directory

## Spec

- [TOOLS.md#delete](../TOOLS.md#delete), [TOOLS.md#selection](../TOOLS.md#selection)
- [QUERY.md#files-and-directories](../QUERY.md#files-and-directories)

## Examples

`140__EXAMPLES__WRITE_SELECTION_TYPES.md` covers:

- the description of `delete` in `tools/list`
- a dry run with `name ~ '*.tmp'` over a file and a symlink: the file only
- the same with `type = link`, and with `type = link or type = file`: the symlink is deleted, its target stays
- the hint, if the spec step keeps it: present, and left out with `omit`

## Acceptance criteria

- the spec fixes the description before the code is written
- every example in `140__EXAMPLES__WRITE_SELECTION_TYPES.md` passes as a test; the examples of all earlier tasks
  still pass
- the server version gets a patch bump (a minor one with the hint)
- the definition of done of [AGENTS.md](../AGENTS.md#definition-of-done) is met
