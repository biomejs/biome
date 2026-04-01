---
"@biomejs/biome": patch
---

noDrizzleDeleteWithoutWhere and noDrizzleUpdateWithoutWhere now support member expressions in the drizzleObjectName option. The option now matches any expression where the last identifier matches the configured name. For example, configuring `"db"` will match `db`, `nested.db`, `context.nested.db`, but not `database` or `nested.database`.
