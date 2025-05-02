---
"@biomejs/biome": patch
---

Updates the [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) rule to more closely match the behavior of the ESLint plugin (e.g. mark the whole fragment as incorrect when no key is present). This also adds the option to check shorthand fragments (`<></>`)
