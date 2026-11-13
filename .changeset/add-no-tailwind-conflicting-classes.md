---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindConflictingClasses`](https://biomejs.dev/linter/rules/no-tailwind-conflicting-classes/), which reports Tailwind CSS classes that set the same CSS properties on an element. Only one of the conflicting classes takes effect, so the other one does nothing.

```jsx
<div className="flex grid" />;
```
