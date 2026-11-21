---
"@biomejs/biome": patch
---

Fixed [#13251](https://github.com/biomejs/biome/issues/13251): parse diagnostics are now reported in source order. Previously, a diagnostic that the parser discovered late, such as a mismatched JSX closing tag, could be reported after diagnostics that appear later in the file.
