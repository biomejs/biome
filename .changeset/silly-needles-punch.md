---
"@biomejs/biome": patch
---

Fixed [#12936](https://github.com/biomejs/biome/issues/12936): nested configurations that enable the `project` or `types` domain, or a rule from these domains, now make `biome lint`, `biome check` and `biome ci` scan the project. Type-aware rules such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now report diagnostics, and [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) now reads the `tailwind.stylesheet` of the nested configuration.
