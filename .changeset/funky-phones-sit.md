---
"@biomejs/biome": patch
---

Fixed [`useTailwindShorthandClasses`](https://biomejs.dev/linter/rules/use-tailwind-shorthand-classes/) suggesting invalid or behavior-changing shorthands for rotations, dividers, content alignment, and custom width and height tokens. Shorthand fixes now preserve `!important` with either leading or trailing `!` markers.

