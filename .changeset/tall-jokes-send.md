---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidVBind`](https://biomejs.dev/linter/rules/use-vue-valid-v-bind/), which enforces the validity of `v-bind` directives in Vue files.

Invalid `v-bind` usages include:
```vue
<Foo v-bind /> <!-- Missing argument -->
<Foo v-bind:foo /> <!-- Missing value -->
<Foo v-bind:foo.bar="baz" /> <!-- Invalid modifier -->
```
