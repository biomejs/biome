---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindLegacyImportant`](https://biomejs.dev/linter/rules/no-tailwind-legacy-important/) for JavaScript and HTML. It reports Tailwind CSS classes that put the `!` important modifier at the start, as Tailwind CSS v3 did, and moves the `!` to the end of the class name.

```jsx
<div className="!flex hover:!bg-red-500" />; // use `flex!` and `hover:bg-red-500!`
```
