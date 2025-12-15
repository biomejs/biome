---
"@biomejs/biome": patch
---

Fix [#8435](https://github.com/biomejs/biome/issues/8435): resolved false positive in `noUnusedVariables` for generic type parameters in construct signature type members (`new <T>(): T`).
