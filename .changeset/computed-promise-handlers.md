---
"@biomejs/biome": patch
---

Fixed [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) reporting promises that are handled through a method named by a string literal. The following code is no longer reported:

```ts
declare const promise: Promise<number>;
promise["then"]((value) => value, () => {});
promise["catch"](() => {});
```
