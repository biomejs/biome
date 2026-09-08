---
"@biomejs/biome": patch
---

Fixed [#7076](https://github.com/biomejs/biome/issues/7076): [`useAriaPropsForRole`](https://biomejs.dev/linter/rules/use-aria-props-for-role) and [`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive) no longer report non-focusable elements with `role="separator"`. A separator with an explicit `tabIndex` or `tabindex` still requires `aria-valuenow`.
