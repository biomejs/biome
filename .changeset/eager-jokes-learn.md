---
"@biomejs/biome": minor
---

Changed [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) to ignore specifiers that are neither a path nor a package name, such as `astro:content`, `virtual:config`, or `~/utils`, when Biome can't find all the modules that a framework provides, for example because Astro isn't installed. Unresolved paths and package names, such as `./missing.js`, are still reported.
