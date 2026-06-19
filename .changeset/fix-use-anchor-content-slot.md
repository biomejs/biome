---
"@biomejs/biome": patch
---

Fixed [#10663](https://github.com/biomejs/biome/issues/10663): the `useAnchorContent` rule no longer reports anchors whose only content is a `<slot>` element.

`<slot>` (in Vue, Svelte, and Astro components, as well as web components) renders content provided by the parent or assigned nodes, which the linter cannot see. Such anchors are therefore no longer flagged as empty, matching how the rule already treats custom (PascalCase) components.

```vue
<!-- No longer reported -->
<a><slot /></a>
```
