---
"@biomejs/biome": patch
---

Fixed [#11933](https://github.com/biomejs/biome/issues/11933): [`useUnicodeRegex`](https://biomejs.dev/linter/rules/use-unicode-regex/) no longer offers to add the `u` flag to patterns that become invalid or change meaning in Unicode mode, such as `/{/`, `/\-/` or `/[\d-z]/`. The rule still reports them.
