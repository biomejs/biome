---
"@biomejs/biome": patch
---

Fixed [#12622](https://github.com/biomejs/biome/issues/12622): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports `??` after an optional chain whose object has a type alias that includes `null` or `undefined`.

```ts
type Media = { items: string[] } | null;
const paths = (media: Media) => media?.items ?? [];
```
