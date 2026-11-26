---
"@biomejs/biome": minor
---

Fixed [#12881](https://github.com/biomejs/biome/issues/12881): Added the `markdown.formatter.quoteStyle` option. It sets the preferred quote for link and image titles, like Prettier's `singleQuote`. Biome still picks the other quote, or parentheses, when that needs fewer escapes. Defaults to `double`.

With `"quoteStyle": "single"`:

```diff
- [Biome](https://biomejs.dev "Single quote here")
+ [Biome](https://biomejs.dev 'Single quote here')
```
