---
"@biomejs/biome": patch
---

Fixed a stack overflow in type-aware lint rules, such as [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), when generic types in files that import each other reference one another in their type parameters.

```ts
// entity.ts
import type { Repository } from "./repository";
export interface Entity<R extends Repository<any> = Repository<any>> {}

// repository.ts
import type { Entity } from "./entity";
export interface Repository<E extends Entity<any> = Entity<any>> {}
```
