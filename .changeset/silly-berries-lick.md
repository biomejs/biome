---
"@biomejs/biome": patch
---

Added the nursery rule [`useTailwindStaticClassStrings`](https://biomejs.dev/linter/rules/use-tailwind-static-class-strings/) to the Tailwind domain. The rule reports class names assembled from fragments, such as `className={"text-" + color}` in JSX and `class="bg-{color}"` in Svelte.
