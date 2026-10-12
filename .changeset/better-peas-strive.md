---
"@biomejs/biome": patch
---

Switched [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) to the Tailwind CSS v4 class order. The rule now checks the attributes and functions set in the top-level `tailwind` configuration, and its own `attributes` and `functions` options were removed. Run `biome migrate --write` to move them to `tailwind.attributes`, `tailwind.mergeFunctions`, and `tailwind.variantFunctions`.

The rule is now part of the `project` domain, so enabling it makes Biome scan your project.
