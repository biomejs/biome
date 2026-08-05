---
"@biomejs/biome": patch
---

Fixed [#10806](https://github.com/biomejs/biome/issues/10806): [`noUselessFragments`](https://biomejs.dev/linter/rules/no-useless-fragments/) no longer causes Biome to panic when its unsafe fix removes a fragment used as a JSX attribute value.
