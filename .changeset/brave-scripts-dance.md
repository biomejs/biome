---
"@biomejs/biome": patch
---

Added the rule [`useRequiredScripts`](https://biomejs.dev/linter/rules/use-required-scripts/), which enforces presence of configurable entries in the `scripts` section of `package.json` files.

Fixed [`noDuplicateDependencies`](https://biomejs.dev/linter/rules/no-duplicate-dependencies/) incorrectly triggering on files like `_package.json`.
