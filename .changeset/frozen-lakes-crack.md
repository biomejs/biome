---
"@biomejs/biome": minor
---

[noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) now accepts more known CSS properties ([#4549](https://github.com/biomejs/biome/issues/4549)).

```diff
- ['anchor-default', 'anchor-scroll', 'inset-area', 'position-animation', 'position-fallback', 'position-fallback-bounds', 'position-try-options']
+ ['anchor-scope', 'interpolate-size', 'line-fit-edge', 'masonry', 'masonry-auto-tracks', 'masonry-direction', 'masonry-fill', 'masonry-flow', 'masonry-slack', 'masonry-template-areas', 'masonry-template-tracks', 'position-anchor', 'position-area', 'position-try-fallbacks', 'position-visibility', 'scroll-start-target', 'text-box', 'view-transition-class', 'view-transition-group']
```

This change replaces deprecated properties, improving CSS validation.
