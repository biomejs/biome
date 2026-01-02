---
"@biomejs/biome": patch
---

Fixed [#8584](https://github.com/biomejs/biome/issues/8584): The HTML formatter will preserve whitespace after some elements and embedded expressions, which more closely aligns with Prettier's behavior.

```diff
- <h1>Hello, {framework}and Svelte!</h1>
+ <h1>Hello, {framework} and Svelte!</h1>
```
