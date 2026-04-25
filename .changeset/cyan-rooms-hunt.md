---
"@biomejs/biome": patch
---

Fixed [#6201](https://github.com/biomejs/biome/issues/6201): [`noUselessEscapeInRegex`](https://biomejs.dev/linter/rules/no-useless-escape-in-regex/) no longer flags an escaped backslash followed by `-` as a useless escape. Patterns like `/[\\-]/` are now considered valid because the second `\` is the escaped backslash, not an unnecessary escape of the trailing dash.
