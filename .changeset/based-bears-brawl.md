---
"@biomejs/biome": minor
---

Biome's resolver now supports `baseUrl` if specified in `tsconfig.json`. This
means that the following now resolves:

**`tsconfig.json`**
```json
{
    "compilerOptions": {
        "baseUrl": "./src",
    }
}
```

**`index.ts`**
```ts
import { foo } from "foo"; // This will now work.
```

**`src/foo.ts`**
```ts
export function foo() {}
```

Fixes [#6432](https://github.com/biomejs/biome/issues/6432).
