---
"@biomejs/biome": patch
---

Added the new nursery rule [`useConsistentImportPaths`](https://biomejs.dev/linter/rules/use-consistent-import-paths/). The rule prefers configured `tsconfig.json` path aliases for distant imports and keeps nearby imports relative, with `package.json#imports` used as a fallback when no TypeScript alias matches.
