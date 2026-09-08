---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnsafeIframeSandbox`](https://biomejs.dev/linter/rules/no-unsafe-iframe-sandbox/), which reports `iframe` elements whose `sandbox` attribute combines `allow-scripts` and `allow-same-origin`, since that combination lets the embedded document remove its own sandboxing.

```jsx
<iframe src="https://example.com" sandbox="allow-scripts allow-same-origin" />
```
