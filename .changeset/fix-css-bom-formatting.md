---
"@biomejs/biome": patch
---

Fixed CSS formatter incorrectly collapsing selectors when a BOM (Byte Order Mark) character is present at the start of the file. The formatter now correctly preserves line breaks between comments and selectors in BOM-prefixed CSS files, matching Prettier's behavior.
