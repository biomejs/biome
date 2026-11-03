---
"@biomejs/biome": patch
---

Added the nursery rule [`useTailwindVariablesShorthand`](https://biomejs.dev/linter/rules/use-tailwind-variables-shorthand/) for JavaScript and HTML. It reports Tailwind CSS classes that read a CSS variable with `[var()]`, and fixes them to the shorthand that Tailwind CSS v4 added.

```jsx
<div className="bg-[var(--brand)]" />; // use `bg-(--brand)`
```
