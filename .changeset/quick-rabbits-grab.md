---
"@biomejs/biome": patch
---

Fixed [#11121](https://github.com/biomejs/biome/issues/11121): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports conditions based on an inapplicable function overload.

For example, the condition in the following code is no longer reported because `query({})` selects the overload that returns `boolean`:

```ts
declare function query(options: { initial: string }): { isPending: false };
declare function query(options: { initial?: string }): { isPending: boolean };

const { isPending } = query({});
isPending || fallback;
```
