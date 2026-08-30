---
"@biomejs/biome": patch
---

Fixed [#11439](https://github.com/biomejs/biome/issues/11439): overload selection no longer reports the first declaration's return type when the checker cannot tell the overloads apart.

Argument compatibility is a viability check: beyond arity it only narrows on whether a callback returns a promise. When several overloads survive it and disagree about what they return, the call's type is now unknown instead of the first declaration's return type, so the result no longer depends on declaration order.

```ts
declare function q(fn: () => 1): string;
declare function q(fn: () => 0): string | undefined;

// no longer reported as an unnecessary optional chain
export const a1 = q(() => 0)?.length;
```
