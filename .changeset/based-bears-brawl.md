---
"@biomejs/biome": minor
---

Biome's resolver now supports `baseUrl` if specified in `tsconfig.json`.

#### Example

Given the following file structure:

**`tsconfig.json`**
```json
{
    "compilerOptions": {
        "baseUrl": "./src",
    }
}
```

**`src/foo.ts`**
```ts
export function foo() {}
```

In this scenario, `import { foo } from "foo";` should work regardless of the
location of the file containing the `import` statement.

Fixes [#6432](https://github.com/biomejs/biome/issues/6432).
