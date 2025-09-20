---
"@biomejs/biome": patch
---

Fixed [#7517](https://github.com/biomejs/biome/issues/7517): the [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) rule no longer suggests changes for typeof checks on global objects.

```ts
// ok
typeof window !== 'undefined' && window.location;
```
