---
"@biomejs/biome": patch
---

Fixed [#12882](https://github.com/biomejs/biome/issues/12882): the Tailwind class parser no longer reports a parse error for unknown utilities with a dashed name and an arbitrary value or a CSS variable, such as `foo-bar-[1px]` or `foo-bar-(--my-var)`. [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) sorts these classes first, like other unknown classes.
