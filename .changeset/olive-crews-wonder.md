---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnknownAtRuleDescriptor`](https://biomejs.dev/linter/rules/no-unknown-at-rule-descriptor/), which disallows unknown descriptors in CSS at-rules such as `@font-face`, `@property`, and `@counter-style`.

```css
@font-face {
  font-famliy: "Example";
}
```
