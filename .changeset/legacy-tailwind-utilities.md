---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindLegacyUtilities`](https://biomejs.dev/linter/rules/no-tailwind-legacy-utilities/) for JavaScript and HTML. It reports Tailwind CSS classes that Tailwind CSS only keeps for backward compatibility, and fixes them to their current names.

```jsx
<div className="flex-grow bg-gradient-to-r" />; // use `grow` and `bg-linear-to-r`
```
