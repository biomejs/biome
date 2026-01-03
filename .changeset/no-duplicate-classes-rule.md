---
"@biomejs/biome": patch
---

Added the new nursery rule [`noDuplicateClasses`](https://biomejs.dev/linter/rules/no-duplicate-classes/) to detect and remove duplicate CSS classes.

**For JSX files:** Available as an assist action at [`assist.source.noDuplicateClasses`](https://biomejs.dev/assist/actions/no-duplicate-classes/). Supports `class`, `className` attributes and utility functions like `clsx`, `cn`, `cva`.

**For HTML files:** Available as a lint rule at [`linter.nursery.noDuplicateClasses`](https://biomejs.dev/linter/rules/no-duplicate-classes/). Checks `class` attributes.

```jsx
// Before
<div class="flex p-4 flex" />;

// After
<div class="flex p-4" />;
```
