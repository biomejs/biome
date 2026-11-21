---
"@biomejs/biome": patch
---

The Tailwind CSS rules no longer make Biome scan the whole project. When [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) is set, Biome reads only that stylesheet and the files it imports, and the language server watches the project so that changes to them update the diagnostics. [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/), [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/) and [`noTailwindRestrictedClasses`](https://biomejs.dev/linter/rules/no-tailwind-restricted-classes/) no longer belong to the `project` domain.
