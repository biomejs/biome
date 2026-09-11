---
"@biomejs/biome": patch
---

Added support for `suite()` as an alias of `describe()` across test analysis rules and formatter. Rules now recognize `suite`, `fsuite`, `xsuite`, and `test.suite` blocks. The formatter recognises them as test declarations.
