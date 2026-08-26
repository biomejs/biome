---
"@biomejs/biome": patch
---

Fixed [`noVueRefAsOperand`](https://biomejs.dev/linter/rules/no-vue-ref-as-operand/) so it no longer reports a callback parameter (e.g. from `.find()`, `.map()`) as an unwrapped ref value just because it's nested inside a `ref()`, `computed()`, or similar call.

```js
const result = computed(() => list.find((item) => item.label === "a"));
```

Previously, `item` here was incorrectly treated as a ref value because the rule attributed it to the outer `computed()` call.
