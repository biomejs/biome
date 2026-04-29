---
"@biomejs/biome": patch
---

In HTML, `style` attributes are now considered to contain embedded CSS in their values. Rules like [`noUnknownProperty`](https://biomejs.dev/linter/rules/no-unknown-property/) will now work in these areas.
