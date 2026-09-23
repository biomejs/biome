---
"@biomejs/biome": patch
---

Fixed [#10531](https://github.com/biomejs/biome/issues/10531): the JavaScript formatter is now idempotent on member chains whose final call argument is an object (or array) literal that doesn't fit on its own line. Previously the first pass fully expanded the chain while the second collapsed it, so a single `biome format --write` left code that a subsequent `biome check` rejected. The chain now stays inline and breaks only that final argument on the first pass.
