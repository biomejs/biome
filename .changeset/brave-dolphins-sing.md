---
"@biomejs/biome": patch
---

Fixed [#9810](https://github.com/biomejs/biome/issues/9810): [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) no longer reports false positives on a getter with a matching setter in the same namespace.

```ts
class Store {
    get status(): string {
        if (Math.random() > 0.5) return "loading";
        return "idle";
    }
    set status(v: string) {}
}
```
