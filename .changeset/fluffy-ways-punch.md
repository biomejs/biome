---
"@biomejs/biome": patch
---

Added the nursery rule [`useIframeSandbox`](https://biomejs.dev/linter/rules/use-iframe-sandbox), which enforces the `sandbox` attribute for `iframe` tags.

**Invalid**:

```html
<iframe></iframe>
```
