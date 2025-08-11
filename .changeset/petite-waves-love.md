---
"@biomejs/biome": patch
---

The resolver can now correctly resolve `.ts`, `.tsx`, `.d.ts`, `.js` files by `.js` extension if exists, based on [the file extension substitution in TypeScript](https://www.typescriptlang.org/docs/handbook/modules/reference.html#file-extension-substitution).

For example, the linter can now detect the floating promise in the following situation, if you have enabled the `noFloatingPromises` rule.

**`foo.ts`**
```ts
export async function doSomething(): Promise<void> {}
```

**`bar.ts`**
```ts
import { doSomething } from "./foo.js"; // doesn't exist actually, but it is resolved to `foo.ts`

doSomething(); // floating promise!
```
