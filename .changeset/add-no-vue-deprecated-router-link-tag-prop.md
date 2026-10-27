---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueDeprecatedRouterLinkTagProp`](https://biomejs.dev/linter/rules/no-vue-deprecated-router-link-tag-prop/), which disallows the `tag` prop on `RouterLink`. Vue Router 4 ignores this prop, so the link still renders as an `<a>` element.

```vue
<template>
  <RouterLink tag="li" to="/">Home</RouterLink>
</template>
```
