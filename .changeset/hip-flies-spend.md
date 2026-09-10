---
"@biomejs/biome": patch
---

Fixed [#7795](https://github.com/biomejs/biome/issues/7795). The [`noJsxLiterals`](https://biomejs.dev/linter/rules/no-jsx-literals/) rule now ignores surrounding whitespace when matching literals against `allowedStrings`.
