---
"@biomejs/biome": patch
---

Fixed [#10244](https://github.com/biomejs/biome/issues/10244): The `useOptionalChain` rule now detects negated guard inequality chains like `!foo || foo.bar !== "x"`.
