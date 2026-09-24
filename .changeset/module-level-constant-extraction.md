---
"@biomejs/biome": minor
---

Added unsafe fixes for [`noMagicNumbers`](https://biomejs.dev/linter/rules/no-magic-numbers/) and [`useTopLevelRegex`](https://biomejs.dev/linter/rules/use-top-level-regex/) that extract literals into uniquely named module-level constants, reusing one declaration for repeated literal values in a file. The regex fix intentionally hoists evaluation and can change regex object identity or mutable state visibility; apply it only when that runtime change is acceptable.
