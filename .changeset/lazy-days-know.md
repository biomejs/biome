---
"@biomejs/biome": patch
---

Fixed [#9810](https://github.com/biomejs/biome/issues/9810): [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) now flags union annotations whose extra variants are never returned, and suggests the narrower type (e.g. `string | null` → `string`).

```ts
function getUser(): string | null { return "hello"; }   // null is never returned
function getCode(): string | number { return "hello"; } // number is never returned
```
