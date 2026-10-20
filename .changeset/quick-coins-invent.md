---
"@biomejs/biome": patch
---

Fixed an issue where the language server kept using the previous parser options for open files after the configuration changed. Open files are now parsed again with the new options, for example `json.parser.allowComments`.
