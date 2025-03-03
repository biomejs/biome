---
"@biomejs/biome": minor
---

Add the new rule `noReactDeps`, which disallow usage of dependency arrays in `createEffect` and `createMemo`.

In Solid, `createEffect` and `createMemo` track dependencies automatically, it's no need to add dependency arrays.