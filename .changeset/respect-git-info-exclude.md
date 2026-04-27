---
"@biomejs/biome": patch
---

Fixed [#4822](https://github.com/biomejs/biome/issues/4822): `vcs.useIgnoreFile` now respects repository-local Git ignore patterns from `.git/info/exclude`.

Because `store_root_ignore_patterns` now includes patterns read through `read_git_info_exclude`, the `NoIgnoreFileFound` diagnostic is only emitted when `.gitignore`, `.ignore`, and `.git/info/exclude` are all absent or empty.
