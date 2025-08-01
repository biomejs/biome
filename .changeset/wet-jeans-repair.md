---
"@biomejs/biome": patch
---

Fix #6675: Only flag noAccumulatingSpread on Object.Assign when a new object is being allocated on each iteration. Ex: foo.reduce((acc, bar) => Object.Assign({}, accum, bar), {})
