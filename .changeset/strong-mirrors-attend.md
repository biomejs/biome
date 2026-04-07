---
"@biomejs/biome": patch
---

Added the nursery rule [`noInlineStyles`](https://biomejs.dev/linter/rules/no-inline-styles/). The rule disallows the use of inline `style` attributes in HTML and the `style` prop in JSX, including `React.createElement` calls. Inline styles make code harder to maintain and can interfere with Content Security Policy.
