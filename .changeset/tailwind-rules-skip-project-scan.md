---
"@biomejs/biome": patch
---

The Tailwind CSS rules no longer make Biome scan the whole project. When [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) is set, Biome reads only that stylesheet and the files it imports. [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) and [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/) no longer belong to the `project` domain.
