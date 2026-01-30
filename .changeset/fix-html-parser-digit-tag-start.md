---
"@biomejs/biome": patch
---

Fixed [#8363](https://github.com/biomejs/biome/issues/8363): HTML parser no longer crashes when encountering a `<` character followed by a digit in text content (e.g., `<12 months`). The parser now correctly emits an "Unescaped `<` bracket character" error instead of treating `<12` as a tag name and crashing.
