---
"@biomejs/biome": patch
---

Added 6 new nursery lint rules for Tailwind CSS utility classes:

- Added the rule [`noDuplicateTailwindClasses`](https://biomejs.dev/linter/rules/no-duplicate-tailwind-classes/). Detects and removes duplicate CSS utility classes in JSX attributes and utility function calls.

- Added the rule [`noUnnecessaryTailwindWhitespace`](https://biomejs.dev/linter/rules/no-unnecessary-tailwind-whitespace/). Removes extra whitespace in CSS utility class strings.

- Added the rule [`noEmptyTailwindArbitraryValue`](https://biomejs.dev/linter/rules/no-empty-tailwind-arbitrary-value/). Flags empty arbitrary values in CSS utility classes like `w-[]` or `text-[]`.

- Added the rule [`noInvalidTailwindVariantCombination`](https://biomejs.dev/linter/rules/no-invalid-tailwind-variant-combination/). Detects conflicting or redundant variant combinations like `hover:hover:` or `hover:focus:hover:`.

- Added the rule [`useTailwindLogicalProperties`](https://biomejs.dev/linter/rules/use-tailwind-logical-properties/). Suggests using logical properties (`ms-*`, `me-*`, `ps-*`, `pe-*`) instead of physical properties (`ml-*`, `mr-*`, `pl-*`, `pr-*`) for better internationalization support.

- Added the rule [`useTailwindColorOpacityModifier`](https://biomejs.dev/linter/rules/use-tailwind-color-opacity-modifier/). Suggests using Tailwind's opacity modifier syntax (`text-red-500/50`) instead of separate opacity utilities.
