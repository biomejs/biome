---
"@biomejs/biome": patch
---

The Tailwind CSS parser now understands container-query variants (`@sm:`, `@max-lg:`, `@min-[400px]:`) and child and descendant variants (`*:`, `**:`). These previously produced parse errors. This affects the [`noTailwindArbitraryValue`](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/) rule, which parses class strings through this parser.
