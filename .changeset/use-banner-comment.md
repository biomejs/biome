---
"@biomejs/biome": patch
---

Added the nursery lint rule [`useBannerComment`](https://biomejs.dev/linter/rules/use-banner-comment/).
Inspired by [eslint-plugin-header](https://github.com/Stuk/eslint-plugin-header), this rule enforces that every JavaScript and CSS file starts with a configured `/* ... */` banner comment. The `content` option accepts either a single string (one canonical banner) or an array of strings (any of which is acceptable). When the banner is missing or does not match, an unsafe fix inserts the canonical banner.

```json,options
{
    "options": {
       "content": "Copyright 2026 Acme"
    }
}
```

```js
const a = 1; // invalid, file does not start with banner comment
```
