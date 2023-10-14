---
title: useAriaActivedescendantWithTabindex (since vnext)
---

**Diagnostic Category: `lint/nursery/useAriaActivedescendantWithTabindex`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.

`aria-activedescendant` is used to manage to focus within a [composite widget].
The element with the attribute `aria-activedescendant` retains the active document focus.

It indicates which of its child elements has a secondary focus by assigning the ID of that
element to the value of `aria-activedescendant`. This pattern is used to build a widget
like a search typeahead select list. The search input box retains document focus
so that the user can type in the input. If the down arrow key is pressed and
a search suggestion is highlighted, the ID of the suggestion element will be applied
as the value of `aria-activedescendant` on the input element.

Because an element with `aria-activedescendant` must be tabbable,
it must either have an inherent tabIndex of zero or declare a tabIndex attribute.

Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-activedescendant-has-tabindex.md

[Composite widget](https://www.w3.org/TR/wai-aria/#composite)

## Examples

### Invalid

```jsx
<div aria-activedescendant={someID} />
```

<pre class="language-text"><code class="language-text">nursery/useAriaActivedescendantWithTabindex.js:1:1 <a href="https://biomejs.dev/lint/rules/use-aria-activedescendant-with-tabindex">lint/nursery/useAriaActivedescendantWithTabindex</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Enforce elements with aria-activedescendant are tabbable.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-activedescendant={someID} /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Add the tabIndex attribute to the element with a value greater than or equal to -1.</span>
  
</code></pre>

## Valid

```jsx
<div aria-activedescendant={someID} tabIndex={0} />
```

```jsx
<input aria-activedescendant={someID} />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
