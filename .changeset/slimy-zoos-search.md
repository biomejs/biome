---
"@biomejs/biome": patch
---

Fixed [#7233](https://github.com/biomejs/biome/issues/7233): The useIndexOf rule now correctly suggests using indexOf() instead of findIndex().

The diagnostic message was incorrectly recommending Array#findIndex() over Array#indexOf(), when it should recommend the opposite for simple equality checks.
