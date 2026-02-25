---
"@biomejs/biome": patch
---

Fixed an issue where the JSON reporter didn't write output to a file when `--reporter-file` was specified. The output is now correctly written to the specified file instead of always going to stdout.
