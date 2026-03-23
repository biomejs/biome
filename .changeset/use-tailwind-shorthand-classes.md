---
"@biomejs/biome": patch
---

Added the rule [`useTailwindShorthandClasses`](https://biomejs.dev/linter/rules/use-tailwind-shorthand-classes/). This rule detects verbose Tailwind class combinations and suggests shorthand equivalents when they are semantically identical.

```diff
-<div class="px-2 py-2">
+<div class="p-2">
```
