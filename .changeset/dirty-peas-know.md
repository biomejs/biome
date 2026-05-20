---
"@biomejs/biome": patch
---

Fixed [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) sorting for Tailwind CSS v4 named values with modifiers. The rule now keeps modifiers such as `bg-red-500/50` and `text-lg/8` in the named utility sort bucket instead of treating them as unknown classes.
