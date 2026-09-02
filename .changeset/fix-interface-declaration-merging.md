---
"@biomejs/biome": patch
---

Fixed [#6644](https://github.com/biomejs/biome/issues/6644): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now recognizes all interface declarations in a TypeScript declaration-merging group when the interface is referenced.

The following snippet no longer triggers the rule.

```ts
interface Things {
    foo: string;
}

interface Things {
    bar: string;
}

export type Key = keyof Things;

interface Things {
    baz: string;
}
```
