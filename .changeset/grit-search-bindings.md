---
"@biomejs/js-api": minor
---

Added GritQL pattern search bindings to the WASM API and `@biomejs/js-api` package. The new bindings enable parsing, searching, and managing GritQL patterns for code analysis and transformation through JavaScript.

Users can now:
- Parse GritQL patterns with `parsePattern(pattern, defaultLanguage)`
- Search for pattern matches in files with `searchPattern(projectKey, filePath, content, patternId)`
- Clean up parsed patterns with `dropPattern(patternId)`

```typescript
const biome = await Biome.create({ distribution: Distribution.BUNDLER });
const projectKey = biome.openProject().projectKey;

// Parse a GritQL pattern
const patternId = biome.parsePattern("`const $x = 1;`", "JavaScript");

// Search for matches
const matches = biome.searchPattern(projectKey, "test.js", fileContent, patternId);

// Clean up
biome.dropPattern(patternId);
```
