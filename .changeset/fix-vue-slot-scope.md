---
"@biomejs/biome": patch
---

Fixed [#11230](https://github.com/biomejs/biome/issues/11230): Vue `v-slot` bindings are no longer reported as undeclared variables.

```vue
<RouterView v-slot="{ Component }">
  <component :is="Component" />
</RouterView>
```

A `v-slot` value that is not a binding pattern, such as `v-slot="{ item }: SlotProps"`, now reports a parse error instead of crashing.
