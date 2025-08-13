---
"@biomejs/biome": patch
---

Fixed [#7162](https://github.com/biomejs/biome/issues/7162): The `noUndeclaredDependencies` rule now considers a type-only import as a dev dependency.

For example, the following code is no longer reported:

**`package.json`**:
```json
{
  "devDependencies": {
    "type-fest": "*"
  }
}
```

**`foo.ts`**:
```ts
import type { SetRequired } from "type-fest";
```

Note that you still need to declare the package in the `devDependencies` section in `package.json`.
