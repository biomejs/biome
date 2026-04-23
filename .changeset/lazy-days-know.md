---
"@biomejs/biome": patch
---

Improved [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/): it now flags union annotations whose extra variants are never returned, and suggests the narrower type (e.g. `string | null` → `string`).

```ts
function getUser(): string | null { return "hello"; }   // null is never returned
function getCode(): string | number { return "hello"; } // number is never returned
```
