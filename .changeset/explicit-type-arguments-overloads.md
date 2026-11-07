---
"@biomejs/biome": patch
---

Fixed [#11446](https://github.com/biomejs/biome/issues/11446): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports a necessary condition on the result of an overloaded function called with explicit type arguments.

```ts
type Trooper = { armour: number };

declare function garrison<T>(unit: T): { current: T };
declare function garrison<T>(unit: T | null): { current: T | null };

const post = garrison<Trooper>(null);
if (!post.current) {
  // no longer reported
}
```
