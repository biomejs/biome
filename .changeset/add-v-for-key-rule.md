---
"@biomejs/biome": patch
---

Added the rule [`useVueVForKey`](https://biomejs.dev/linter/rules/use-vue-v-for-key/), which enforces that any element using `v-for` also specifies a `key`.

**Invalid**

```vue
<li v-for="item in items">{{ item }}</li>
```

**Valid**

```vue
<li v-for="item in items" :key="item.id">{{ item }}</li>
```
