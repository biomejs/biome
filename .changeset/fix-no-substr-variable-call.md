---
"@biomejs/biome": patch
---

Fixed `noSubstr` rule not detecting `substr()`/`substring()` method calls in variable declarations (e.g., `const y = x.substring(0)`). The rule now correctly handles both direct member expressions and call expressions in variable initializers.
