---
"@biomejs/biome": patch
---

Added the rule [`noDrizzleDeleteWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-delete-without-where/) to prevent accidental full-table deletes when using Drizzle ORM without a `.where()` clause.
