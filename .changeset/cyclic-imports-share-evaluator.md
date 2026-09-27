---
"@biomejs/biome": patch
---

Fixed [#11610](https://github.com/biomejs/biome/issues/11610), [#11611](https://github.com/biomejs/biome/issues/11611), [#11612](https://github.com/biomejs/biome/issues/11612), [#11615](https://github.com/biomejs/biome/issues/11615), and [#11616](https://github.com/biomejs/biome/issues/11616): type-aware lint rules were several times slower in Biome 2.5.12 when inferring types that come from packages with import cycles between their declaration files, such as Zod. Type inference now builds a single declaration graph per lookup instead of one for every cyclic import it encounters.
