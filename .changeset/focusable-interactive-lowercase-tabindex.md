---
"@biomejs/biome": patch
---

[`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive/) no longer reports elements that use the HTML attribute name `tabindex`, such as `<div role="button" tabindex={0} />`. Some JSX dialects mirror HTML attribute names instead of the React ones, and both spellings are now accepted.
