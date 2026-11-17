---
"@biomejs/biome": patch
---

Fixed [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) treating some valid classes as unknown and moving them to the front: gradient interpolation modifiers such as `bg-linear-to-r/oklch`, and the `@container-normal` and `@container-size` utilities. Biome now also treats `shadow-inherit/5`, which Tailwind doesn't generate, as unknown.
