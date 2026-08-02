---
"@biomejs/biome": patch
---

Added the nursery rule [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/), which reports references to custom properties that are not defined in available CSS or HTML-like files.

For example, the following snippet triggers the rule:

```css
a { color: var(--undefined-color); }
```
