---
"@biomejs/biome": patch
---

Fixed [#4565](https://github.com/biomejs/biome/issues/4565): [noControlCharactersInRegex](https://biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics when it encounters an unterminated unicode escape sequence.
