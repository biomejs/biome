---
"@biomejs/biome": patch
---

Fixed [#7899](https://github.com/biomejs/biome/issues/7899): [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules/) no longer reports `@tailwind` at-rules (such as `@tailwind base;`) when Tailwind directives are enabled.

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```
