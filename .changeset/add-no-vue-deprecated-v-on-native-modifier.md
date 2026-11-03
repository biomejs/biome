---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueDeprecatedVOnNativeModifier`](https://biomejs.dev/linter/rules/no-vue-deprecated-v-on-native-modifier/), which disallows the `.native` modifier on Vue's `v-on` directive. Vue 3 removed this modifier.

```vue
<MyComponent @click.native="onClick" />
```
