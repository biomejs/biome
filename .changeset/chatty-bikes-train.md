---
"@biomejs/biome": patch
---

Fixed [#6933](https://github.com/biomejs/biome/issues/6933) and [#6994](https://github.com/biomejs/biome/issues/6994).

When the values of private member assignment expressions, increment expressions, etc. are used, those private members are no longer marked as unused.
