---
"@biomejs/biome": patch
---

Fixed [#11872](https://github.com/biomejs/biome/issues/11872): [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) no longer reports an anchor whose only content is a custom component written with an explicit closing tag. Biome already accepted the self-closing spelling, and Astro, Vue and Svelte treat the two as the same component. Astro's `Image` is now checked the same way in both spellings, including its `alt` attribute.

```astro
<!-- No longer reported -->
<a href="/x"><Icon></Icon></a>

<!-- Now reported: Astro's Image does not render its children -->
<a href="/x"><Image alt="">Home</Image></a>
```
