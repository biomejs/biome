---
title: useAriaActivedescendantTabindex (since vnext)
---

**Diagnostic Category: `lint/nursery/useAriaActivedescendantTabindex`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.

aria-activedescendant is used to manage focus within a [composite widget](https://www.w3.org/TR/wai-aria/#composite).
The element with the attribute aria-activedescendant retains the active document focus;
it indicates which of its child elements has secondary focus by assigning the ID of that
element to the value of aria-activedescendant. This pattern is used to build a widget
like a search typeahead select list. The search input box retains document focus
so that the user can type in the input. If the down arrow key is pressed and
a search suggestion is highlighted, the ID of the suggestion element will be applied
as the value of aria-activedescendant on the input element.

Because an element with aria-activedescendant must be tabbable,
it must either have an inherent tabIndex of zero or declare a tabIndex attribute.

## Examples

### Invalid

```jsx
<div aria-activedescendant={someID} />
```

<pre class="language-text"><code class="language-text">nursery/useAriaActivedescendantTabindex.js:1:1 <a href="https://biomejs.dev/lint/rules/use-aria-activedescendant-tabindex">lint/nursery/useAriaActivedescendantTabindex</a> ━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Enforce elements with aria-activedescendant are tabbable.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-activedescendant={someID} /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
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
