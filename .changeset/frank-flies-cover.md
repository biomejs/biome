---
"@biomejs/biome": patch
---

Fixed CSS package imports that use the `style` export condition, such as `tw-animate-css`. With `tailwind.stylesheet` configured, [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) now reads utilities and theme values from imported packages, including their transitive CSS imports.
