---
"@biomejs/biome": minor
---

The rule [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) now resolves the modules that Astro provides, such as `astro:content` and `astro:assets`, and checks the names imported from them. Biome reads these modules from the type declarations of the installed `astro` package.

The rule also finds the exports of a package whose type declarations describe the package with `declare module "package-name" { ... }`. Previously, importing such an export was reported as an error.
