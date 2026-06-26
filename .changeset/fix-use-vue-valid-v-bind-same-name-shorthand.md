---
"@biomejs/biome": patch
---

Fixed [#10754](https://github.com/biomejs/biome/issues/10754): [`useVueValidVBind`](https://biomejs.dev/linter/rules/use-vue-valid-v-bind/) no longer reports the Vue 3.4+ same-name shorthand as missing a value. `:foo` and `v-bind:foo` are now accepted as equivalent to `:foo="foo"`, while `v-bind`, `v-bind:[dynamicArg]`, and `:[dynamicArg]` without a value continue to be reported.
