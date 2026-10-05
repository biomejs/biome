---
"@biomejs/biome": patch
---

[`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) now sorts classes in the same order as Tailwind CSS v4, and checks the attributes and functions set in the top-level `tailwind` configuration. Its own `attributes` and `functions` options were removed: use `tailwind.attributes` and `tailwind.mergeFunctions` instead.
