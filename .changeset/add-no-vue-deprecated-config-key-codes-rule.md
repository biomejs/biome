---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueDeprecatedConfigKeyCodes`](https://biomejs.dev/linter/rules/no-vue-deprecated-config-key-codes/), which reports uses of `Vue.config.keyCodes`. Vue 3 removed this option, so custom key names defined with it no longer work.

```js
Vue.config.keyCodes = { f1: 112 };
```
