---
"@biomejs/biome": patch
---

Added the rule [`noDrizzleUpdateWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-update-without-where/) to prevent accidental full-table updates when using Drizzle ORM without a `.where()` clause.
