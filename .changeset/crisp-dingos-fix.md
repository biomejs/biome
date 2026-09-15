---
"@biomejs/biome": patch
---

Fixed [#10248](https://github.com/biomejs/biome/issues/10248): [`noUselessFragments`](https://biomejs.dev/linter/rules/no-useless-fragments/) now allows fragments with props in Astro files, such as `<Fragment slot="name">{text}</Fragment>` inside template expressions.
