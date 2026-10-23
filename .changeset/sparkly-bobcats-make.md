---
"@biomejs/biome": patch
---

Fixed GraphQL parsing and formatting for executable descriptions and legacy fragment variables. Biome now formats inputs such as `"Description" query { field }` without panicking and removes insignificant commas consistently.

