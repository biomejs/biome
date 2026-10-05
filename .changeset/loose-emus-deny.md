---
"@biomejs/biome": minor
---

Added the `tailwind.stylesheet` option, which points Biome at the CSS file that holds your Tailwind CSS configuration. [`useTailwindSortedClasses`](https://biomejs.dev/linter/rules/use-tailwind-sorted-classes/) reads the `@theme`, `@utility`, and `@custom-variant` rules in that file and in the local files it imports, so your own theme values, utilities, and variants sort correctly.

```json
{
  "tailwind": { "stylesheet": "./src/app.css" }
}
```
