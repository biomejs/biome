---
"@biomejs/biome": patch
---

[`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) now detects Promises used as conditions when the type comes from a generic type alias.

```ts
type MaybePromise<T> = T | Promise<T>;
declare const cached: MaybePromise<string>;
if (cached) {
}
```
