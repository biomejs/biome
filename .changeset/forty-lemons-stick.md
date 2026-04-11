---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxNamespace`](https://biomejs.dev/linter/rules/no-jsx-namespace), which disallows JSX namespace syntax.

**Invalid**:

```jsx
<ns:testcomponent />
```
