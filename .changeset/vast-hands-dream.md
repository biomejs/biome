---
"@biomejs/biome": minor
---

Added support for the modules that Astro provides to [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/): imports from `astro:content` and `astro:assets` now resolve, and the rule checks the names imported from them. The rule also finds the exports of packages whose type declarations use `declare module "package-name" { ... }`, which it previously reported as missing.
