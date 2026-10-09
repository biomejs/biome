---
"@biomejs/biome": patch
---

Improved type inference for template literals. Biome now infers untagged template literals as strings, so type-aware rules such as [`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/) now report code like ``String(`hello ${name}`)``.
