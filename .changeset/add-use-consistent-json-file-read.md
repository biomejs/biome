---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentJsonFileRead`](https://biomejs.dev/linter/rules/use-consistent-json-file-read/), which enforces a consistent way of reading JSON files before passing them to `JSON.parse()`. By default, it reports files read as a `Buffer`; set `readAs` to `"buffer"` to report files read as a UTF-8 string instead.

```js
const packageJson = JSON.parse(await fs.readFile("./package.json"));
```
