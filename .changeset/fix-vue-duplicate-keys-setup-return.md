---
"@biomejs/biome": patch
---

[`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/) now checks the properties returned by a `setup()` method of an options object, so `return { foo }` conflicts with a prop, data property, method or computed property named `foo`.
