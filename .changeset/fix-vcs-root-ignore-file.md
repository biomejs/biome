---
"@biomejs/biome": patch
---

Fixed [#6964](https://github.com/biomejs/biome/issues/6964): Biome now correctly resolves the `.gitignore` file relative to `vcs.root` when configured. Previously, the `vcs.root` setting was ignored and Biome always looked for the ignore file in the workspace directory.
