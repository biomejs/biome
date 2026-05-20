---
"@biomejs/biome": patch
---

Fixed [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) sorting for Tailwind CSS v4 arbitrary values and named values with modifiers. The rule now classifies arbitrary colors, lengths, images, positions, background sizes, and related CSS value types before choosing the Tailwind utility sort key.
