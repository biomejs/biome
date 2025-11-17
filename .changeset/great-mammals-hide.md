---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueHyphenatedAttributes`](https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes/), which encourages using kebab case for attribute names, per the Vue style guide's recommendations.

```vue
<!-- Invalid -->
<MyComponent myProp="value" />

<!-- Valid -->
<MyComponent my-prop="value" />
```
