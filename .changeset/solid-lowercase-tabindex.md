---
"@biomejs/biome": patch
---

Fixed [#13421](https://github.com/biomejs/biome/issues/13421): in Astro templates and in projects that depend on `solid-js`, the rules [`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive/), [`noPositiveTabindex`](https://biomejs.dev/linter/rules/no-positive-tabindex/), [`noNoninteractiveTabindex`](https://biomejs.dev/linter/rules/no-noninteractive-tabindex/), [`useAriaActivedescendantWithTabindex`](https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex/), and [`noAriaHiddenOnFocusable`](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable/) now recognize the lowercase `tabindex` attribute on JSX elements, and their diagnostics and fixes suggest `tabindex` instead of `tabIndex`.

The following code is no longer reported in a Solid project or inside an Astro template expression:

```jsx
<div role="button" tabindex={0} onClick={() => {}} />
```
