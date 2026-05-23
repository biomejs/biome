---
"@biomejs/biome": patch
---

Fixed [#10297](https://github.com/biomejs/biome/issues/10297): the [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) unsafe fix no longer removes a leading or trailing space from a string literal inside a template literal substitution, which previously merged the class with the surrounding text.
