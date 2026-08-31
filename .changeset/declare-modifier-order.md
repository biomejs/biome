---
"@biomejs/biome": patch
---

Fixed [#11500](https://github.com/biomejs/biome/issues/11500): the formatter now prints the `declare` modifier before accessibility modifiers on class properties. `private declare readonly name: string` is now formatted as `declare private readonly name: string`, matching Prettier and TypeScript's canonical modifier order.
