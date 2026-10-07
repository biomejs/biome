---
"@biomejs/biome": minor
---

Removed the undocumented `javascript.parser.gritMetavariables` option. It was only used by Biome's internal parser tests and didn't affect how Biome parses project files. Remove it from your configuration file if you set it.
