---
"@biomejs/js-api": minor
---

Added GritQL search to `@biomejs/js-api`. Parse a pattern with `parsePattern()`, search content with `searchContent()`, and release the pattern with `dropPattern()`. Matches are returned as UTF-8 byte offsets; use `spanInBytesToSpanInCodeUnits()` to convert them before slicing the content.

```js
const patternId = biome.parsePattern("`console.log($message)`", {
  defaultLanguage: "js",
});
const { matches } = biome.searchContent(projectKey, content, {
  filePath: "example.js",
  patternId,
});
biome.dropPattern(patternId);
```
