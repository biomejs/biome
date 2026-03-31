---
"@biomejs/biome": patch
---

noDrizzleDeleteWithoutWhere and noDrizzleUpdateWithoutWhere now support member expression paths (e.g. ctx.db) in the drizzleObjectName option, in addition to simple identifiers.
