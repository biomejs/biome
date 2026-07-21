---
"@biomejs/biome": patch
---

Improved overload selection for [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/). Biome now handles overloaded calls, overloaded constructors, rest parameters, union arguments, and generic constraints without selecting an incompatible signature. For example, `noMisusedPromises` now reports the async callback passed to the synchronous overload:

```ts
declare function consume(kind: "async", callback: () => Promise<void>): void;
declare function consume(kind: "sync", callback: () => void): void;
consume("sync", async () => {});
```
