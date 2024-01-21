---
title: useAriaActivedescendantWithTabindex (since v1.3.0)
---

**Diagnostic Category: `lint/a11y/useAriaActivedescendantWithTabindex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-activedescendant-has-tabindex.md" target="_blank"><code>aria-activedescendant-has-tabindex</code></a>

Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.

`aria-activedescendant` is used to manage to focus within a [composite widget](https://www.w3.org/TR/wai-aria/#composite).
The element with the attribute `aria-activedescendant` retains the active document focus.

It indicates which of its child elements has a secondary focus by assigning the ID of that
element to the value of `aria-activedescendant`. This pattern is used to build a widget
like a search typeahead select list. The search input box retains document focus
so that the user can type in the input. If the down arrow key is pressed and
a search suggestion is highlighted, the ID of the suggestion element will be applied
as the value of `aria-activedescendant` on the input element.

Because an element with `aria-activedescendant` must be tabbable,
it must either have an inherent tabIndex of zero or declare a tabIndex attribute.

## Examples

### Invalid

```jsx
<div aria-activedescendant={someID} />
```

<pre class="language-text"><code class="language-text">a11y/useAriaActivedescendantWithTabindex.js:1:1 <a href="https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex">lint/a11y/useAriaActivedescendantWithTabindex</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Enforce elements with aria-activedescendant are tabbable.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div aria-activedescendant={someID} /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">aria-activedescendant is used to manage focus within a composite widget.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">The element with the attribute aria-activedescendant retains the active document focus.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Add the tabIndex attribute to the element with a value greater than or equal to -1.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add the tabIndex attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;div<span style="opacity: 0.8;">·</span>aria-activedescendant={someID}<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">I</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">0</span><span style="color: MediumSeaGreen;">&quot;</span><span style="opacity: 0.8;">·</span>/&gt;
<strong>  </strong><strong>    │ </strong>                                   <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>   
</code></pre>

### Valid

```jsx
<div aria-activedescendant={someID} tabIndex={0} />
```

```jsx
<input aria-activedescendant={someID} />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
