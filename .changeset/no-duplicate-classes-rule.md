---
"@biomejs/biome": minor
---

Added the [`noDuplicateClasses`](https://biomejs.dev/assist/actions/no-duplicate-classes/) assist action to detect and remove duplicate CSS classes.

**For JSX files:** Supports `class`, `className` attributes and utility functions like `clsx`, `cn`, `cva`.

**For HTML files:** Checks `class` attributes. This is the first assist action for HTML.

```jsx
// Before
<div class="flex p-4 flex" />;

// After
<div class="flex p-4" />;
```
