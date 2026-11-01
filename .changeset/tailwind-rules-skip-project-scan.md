---
"@biomejs/biome": patch
---

Biome no longer scans the whole project for the [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/), [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/) and [`noTailwindRestrictedClasses`](https://biomejs.dev/linter/rules/no-tailwind-restricted-classes/) rules when [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) isn't set. These rules no longer belong to the `project` domain.
