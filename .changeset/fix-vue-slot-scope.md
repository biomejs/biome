---
"@biomejs/biome": patch
---

Fixed [#11230](https://github.com/biomejs/biome/issues/11230): Vue `v-slot` bindings are no longer reported as undeclared variables.

```vue
<RouterView v-slot="{ Component }">
  <component :is="Component" />
</RouterView>
```

Slot bindings are only visible inside the element's content, so a reference to `Component` outside the `RouterView` above, or in another attribute on its opening tag, is still reported. A `v-slot` value that is not a binding pattern, such as `v-slot="{ item }: SlotProps"`, now reports a parse error instead of crashing.
