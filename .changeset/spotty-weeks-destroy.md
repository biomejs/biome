---
"@biomejs/biome": patch
---

The [`useKeyWithClickEvents`](https://biomejs.dev/linter/rules/use-key-with-click-events/) rule has been improved with better support for ARIA roles.

Key improvements:

1. **Accessibility checks**:

Now the rule correctly handles the following cases:

- If an element is hidden from screen readers
- If an element has the presentation role
- If an element is interactive

```jsx
// No errors
<div aria-hidden="true" onClick={() => {}} /> // hidden from screen reader
<div role="presentation" onClick={() => {}} /> // presentation role
<button onClick={() => {}} /> // interactive role
```

This change ensures the rule is more accurate and helpful.

2. **Checks spread syntax**:

Spread syntax used to be ignored, but has been changed to be pointed out for more stringent checking.

```jsx
// Errors
<div {...props} onClick={() => {}} />
// No errors
<div {...props} onClick={() => {}} onKeyDown={foo} />;
```

3. **Refactor**:

Now the rule uses the aria roles to determine if an element is interactive.


The changes shown here are meant to be closer to the original [jsx-eslint's `click-events-have-key-events` rule](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/click-events-have-key-events.md).