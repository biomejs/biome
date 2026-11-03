---
"@biomejs/biome": patch
---

Fixed [#12873](https://github.com/biomejs/biome/issues/12873): [`useFlatMap`](https://biomejs.dev/linter/rules/use-flat-map/) no longer reports `.map().flat()` chains where `flat` is reached through optional chaining, such as `obj.map(f)?.flat()` or `obj.map(f).flat?.()`. Its safe fix used to drop the `?.`, which changed when the expression short-circuits.
