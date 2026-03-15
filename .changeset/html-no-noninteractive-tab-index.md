---
"@biomejs/biome": minor
---

Added the [`noNoninteractiveTabindex`](https://biomejs.dev/linter/rules/no-noninteractive-tabindex/) lint rule for HTML. This rule enforces that `tabindex` is not used on non-interactive elements, as it can cause usability issues for keyboard users.

`<div tabindex="0">Invalid: non-interactive element</div>`
