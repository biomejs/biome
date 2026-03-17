---
"@biomejs/biome": patch
---

Added the rules [`noDrizzleDeleteWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-delete-without-where/) and [`noDrizzleUpdateWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-update-without-where/) to prevent accidental full-table deletes and updates when using Drizzle ORM without a `.where()` clause.
