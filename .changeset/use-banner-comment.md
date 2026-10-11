---
"@biomejs/biome": patch
---

Added the nursery lint rule [`useBannerComment`](https://biomejs.dev/linter/rules/use-banner-comment/), inspired by [eslint-plugin-header](https://github.com/Stuk/eslint-plugin-header). Biome now reports JavaScript and CSS files that don't start with the `/* ... */` banner configured with the `content` option. An unsafe fix inserts the banner at the top of the file.

```json
{
    "options": {
       "content": "Copyright 2026 Acme"
    }
}
```

```js
const a = 1; // invalid, file does not start with banner comment
```
