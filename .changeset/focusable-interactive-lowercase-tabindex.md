---
"@biomejs/biome": patch
---

The a11y rules that read an element's tab index now accept the HTML attribute name `tabindex` alongside the React one, `tabIndex`. Some JSX dialects mirror HTML attribute names, and both spellings set the tab index of an element, so [`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive/), [`noPositiveTabindex`](https://biomejs.dev/linter/rules/no-positive-tabindex/), [`noNoninteractiveTabindex`](https://biomejs.dev/linter/rules/no-noninteractive-tabindex/), [`useAriaActivedescendantWithTabindex`](https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex/) and [`noAriaHiddenOnFocusable`](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable/) all look up both.

For instance, `<div role="button" tabindex={0} />` is no longer reported as not focusable, `<div tabindex={1} />` is now reported as a positive tab index, and `<div aria-hidden="true" tabindex="0" />` is now reported as hidden while focusable.
