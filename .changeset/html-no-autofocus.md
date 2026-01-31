---
"@biomejs/biome": minor
---

Added the [`noAutofocus`](https://biomejs.dev/linter/rules/no-autofocus/) lint rule for HTML. This rule enforces that the `autofocus` attribute is not used on elements, as it can cause usability issues for sighted and non-sighted users. The rule allows `autofocus` inside `dialog` elements or elements with the `popover` attribute, as these are modal contexts where autofocus is expected.
