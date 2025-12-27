---
"@biomejs/biome": patch
---

Fixed [#8584](https://github.com/biomejs/biome/issues/8584): The HTML formatter will preserve whitespace after expressions, which now matches Svelte's Prettier plugin.

```diff
- <h1>Hello, {framework}and Svelte!</h1>
+ <h1>Hello, {framework} and Svelte!</h1>
```
