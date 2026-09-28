---
"@biomejs/biome": patch
---

Added the nursery rule [`noAstroConflictingSetDirectives`](https://biomejs.dev/linter/rules/no-astro-conflicting-set-directives/), which reports Astro elements with multiple content sources, such as `set:html`, `set:text`, and child content.

For example, `<div set:html={html}>content</div>` triggers the rule.
