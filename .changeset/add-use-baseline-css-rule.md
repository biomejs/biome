---
"@biomejs/biome": patch
---

Added new nursery lint rule `useBaseline` for CSS. The rule reports when CSS properties, property values, at-rules, media conditions, functions, or pseudo-selectors are not part of the configured [Baseline](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility) tier.

For example, *at the time of writing*, the rule will trigger for the use of `accent-color` because it has limited availability:

```css
a { accent-color: bar; }
```
