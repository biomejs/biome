---
"@biomejs/biome": patch
---

Fixed a false positive in [`useTailwindShorthandClasses`](https://biomejs.dev/linter/rules/use-tailwind-shorthand-classes/) for strings in conditional tests, such as `cn(m === "w-2 h-2" ? "bg-red-800" : "bg-red-400")`.
