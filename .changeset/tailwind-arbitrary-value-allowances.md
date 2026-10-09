---
"@biomejs/biome": patch
---

Added the `allowedCategories` and `allowedClasses` options to the nursery rule [`noTailwindArbitraryValue`](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/). They allow arbitrary values in categories of utilities, such as `layout`, or in specific classes, such as `p-[13px]`.

The rule no longer reports arbitrary modifiers, so `bg-black/[0.5]` and `text-sm/[18px]` are now valid.
