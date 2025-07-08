---
"@biomejs/biome": patch
---

Type inference is now able to handle ternary conditions in type aliases.

Note that we don't attempt to evaluate the condition itself. The resulting type
is simply a union of both conditional outcomes.

**Example**

```ts
type MaybeResult<T> = T extends Function ? Promise<string> : undefined;

// We can now detect this function _might_ return a `Promise`:
function doStuff<T>(input: T): MaybeResult<T> { /* ... */ }
```
