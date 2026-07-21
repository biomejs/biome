---
"@biomejs/biome": patch
---

Fixed [#10980](https://github.com/biomejs/biome/issues/10980): [`useAriaPropsSupportedByRole`](https://biomejs.dev/linter/rules/use-aria-props-supported-by-role/) no longer reports false positives when the attribute that determines an element's implicit ARIA role is written as a shorthand attribute, such as `<a {href} aria-label="...">` in Astro and Svelte files.

Shorthand attributes are now taken into account when computing the implicit role, so the anchor above correctly resolves to the `link` role instead of `generic`.
