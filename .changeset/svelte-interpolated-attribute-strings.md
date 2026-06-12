---
"@biomejs/biome": patch
---

Improved Svelte lint rule accuracy for quoted attribute values containing `{expression}` interpolations.

- [`noRedundantAlt`](https://biomejs.dev/linter/rules/no-redundant-alt/) no longer emits false positives when the alt text contains an interpolation, e.g. `alt="image of {person}"`.
- [`useButtonType`](https://biomejs.dev/linter/rules/use-button-type/) no longer emits false positives for dynamic button types written as `type="{dynamicType}"`.
- [`noScriptUrl`](https://biomejs.dev/linter/rules/no-script-url/) no longer emits false positives for dynamic hrefs such as `href="{url}"`.
