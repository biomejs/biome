---
"@biomejs/biome": patch
---

Fix #6675: Only flag noAccumulatingSpread on Object.assign when a new object is being allocated on each iteration. Ex: foo.reduce((acc, bar) => Object.assign({}, acc, bar), {})
