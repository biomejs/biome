---
"@biomejs/biome": patch
---

Fixed [#11647](https://github.com/biomejs/biome/issues/11647): false positives in [`noNoninteractiveElementInteractions`](https://biomejs.dev/linter/rules/no-noninteractive-element-interactions/) for resource `load` and `error` handlers in HTML, Svelte, Vue, Astro, and JSX. Non-interactive elements can handle resource events without requiring an interactive role.
