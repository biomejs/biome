---
"@biomejs/biome": patch
---

Added the nursery rule [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/), which reports references to custom properties that are not defined in available CSS, static HTML-like `style` attributes, or JSX string `style` attributes.

For example, the following snippet triggers the rule:

```css
a { color: var(--undefined-color); }
```
