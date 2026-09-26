---
"@biomejs/biome": patch
---

Added [`useTailwindStaticClassStrings`](https://biomejs.dev/linter/rules/use-tailwind-static-class-strings/) to the Tailwind domain. The nursery rule reports class names assembled from fragments, such as `className={"text-" + color}` in JSX and `class="bg-{color}"` in Svelte.
