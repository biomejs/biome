---
"@biomejs/biome": patch
---

Added 2 new nursery lint rules for Tailwind CSS utility classes:

- Added the rule [`noDuplicateTailwindClasses`](https://biomejs.dev/linter/rules/no-duplicate-tailwind-classes/). Detects and removes duplicate CSS utility classes in JSX attributes and utility function calls.

- Added the rule [`noUnnecessaryTailwindWhitespace`](https://biomejs.dev/linter/rules/no-unnecessary-tailwind-whitespace/). Removes extra whitespace in CSS utility class strings.
