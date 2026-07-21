---
"@biomejs/biome": patch
---

Improved `GlobalsResolver` by automatically extracting global types from TypeScript's official `.d.ts` files. This modernization enhances type inference for built-ins like `Array`, `Promise`, `Map`, `Set`, and `Error`, including support for generic interfaces and static members.
