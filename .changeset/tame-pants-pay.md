---
"@biomejs/biome": patch
---

Fixed [#9081](https://github.com/biomejs/biome/issues/9081): The `noUnknownPseudoElement` rule no longer reports false positives for any known pseudo elements in CSS modules. This was a regression introduced in v2.4.0.
