---
"@biomejs/biome": patch
---

Added the [`noInvalidFileInputAccept`](https://biomejs.dev/linter/rules/no-invalid-file-input-accept/) nursery rule. The rule reports invalid literal `accept` values on file inputs in JSX and HTML, and normalizes common mistakes.

```jsx
<input type="file" accept="image/jpg" />
```
