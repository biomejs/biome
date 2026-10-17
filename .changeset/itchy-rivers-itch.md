---
"@biomejs/biome": minor
---

Removed the `javascript.parser.gritMetavariables` option. It was intended for internal use only and had no effect on how Biome parses JavaScript and TypeScript files. Biome now reports it as an unknown key and exits with a configuration error, so remove it from your configuration file.
