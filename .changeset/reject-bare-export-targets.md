---
"@biomejs/biome": patch
---

Fixed a crash when a dependency's `package.json` maps an `exports` entry to its own package name, such as `"exports": { "sass": "my-package" }`. Like Node.js, Biome now rejects `exports` targets that don't start with `./`.
