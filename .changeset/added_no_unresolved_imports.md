---
"@biomejs/biome": minor
---

The nursery rule [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) has been added.

Importing a non-existing export is an error at runtime or build time. With this
rule, Biome can detect such incorrect imports and report errors for them.

Note that if you use TypeScript, you probably don't want to use this rule, since
TypeScript already performs such checks for you.
