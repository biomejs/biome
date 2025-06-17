---
"@biomejs/biome": major
---

Previously the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` checked both regular expression literals like `/regex/` and dynamically built regular expressions like `new RegExp("regex")`.

Checking dynamically built regular expressions has many limitations, edge cases, and complexities.
In addition, other rules that lint regular expressions don't check dynamically built regular expressions.

Rather than add support for other rules and have half-baked checking, we decided to remove support for dynamically built regular expressions.

Now the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` only check literals of regular expressions.
