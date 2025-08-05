---
"@biomejs/biome": patch
---

Fixed [#7101](https://github.com/biomejs/biome/issues/7101)

1. If a class member defined in a constructor argument is only used within the constructor, remove the private modifier and make it a plain method argument.

2. If it is not used within the constructor, prefix it with an underscore, as with no_unused_function_parameter.
