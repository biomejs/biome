---
"@biomejs/biome": patch
---

Biome no longer scans the whole project for the [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) and [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/) rules when [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) isn't set. These rules no longer belong to the `project` domain.
