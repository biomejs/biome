---
"@biomejs/biome": patch
---

Added the `allowBlackAndWhite` option to [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/). It defaults to `true`. When set to `false`, the rule also reports classes that use `black` or `white`, such as `bg-white` and `text-black/50`, and suggests similar colors from your theme when `tailwind.stylesheet` is set.

```json
{ "options": { "allowBlackAndWhite": false } }
```
