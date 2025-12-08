---
"@biomejs/biome": patch
---

Improved the performance of the CLI when processing big projects. Biome now reduces system calls when indexing manifest files such as
`package.json`, `tsconfig.json` and `biome.json`.
