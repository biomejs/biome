---
"@biomejs/biome": patch
---

Added the new nursery rule [`noDuplicateClasses`](https://biomejs.dev/linter/rules/no-duplicate-classes/).

Detects and removes duplicate CSS classes in:
- JSX `class` and `className` attributes
- HTML `class` attributes
- Utility function calls like `clsx`, `cn`, `cva`

```html
<!-- Before -->
<div class="flex p-4 flex"></div>

<!-- After -->
<div class="flex p-4"></div>
```
