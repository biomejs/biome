---
"@biomejs/biome": patch
---

Fixed [#9024](https://github.com/biomejs/biome/issues/9024). The `noInteractiveElementToNoninteractiveRole` rule no longer incorrectly flags `<hr>` elements with `role="presentation"` or `role="none"`. The `separator` role (implicit role of `<hr>`) is now treated as non-interactive, matching the WAI-ARIA spec where a non-focusable separator is a static structural element.
