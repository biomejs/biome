---
"@biomejs/biome": patch
---

Recognized Astro `<style define:vars>` keys as custom-property declarations. The [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/) rule now accepts statically supplied variables and avoids diagnostics when dynamic keys may provide a declaration.
