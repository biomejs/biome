---
"@biomejs/biome": patch
---

Added the rule [`noDuplicateClasses`](https://biomejs.dev/linter/rules/no-duplicate-classes/).

Detects and removes duplicate CSS classes in:
- JSX `class` and `className` attributes
- HTML `class` attributes
- Utility function calls like `clsx`, `cn`, `cva`

Inspired by [`eslint-plugin-better-tailwindcss/no-duplicate-classes`](https://github.com/schoero/eslint-plugin-better-tailwindcss).
