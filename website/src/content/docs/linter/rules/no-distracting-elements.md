---
title: noDistractingElements (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noDistractingElements`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-distracting-elements.md" target="_blank"><code>no-distracting-elements</code></a>

Enforces that no distracting elements are used.

Elements that can be visually distracting can cause accessibility issues with visually impaired users.
Such elements are most likely deprecated, and should be avoided.
By default, the following elements are visually distracting: `<marquee>` and `<blink>`.

## Examples

### Invalid

```jsx
<marquee />
```

<pre class="language-text"><code class="language-text">a11y/noDistractingElements.js:1:1 <a href="https://biomejs.dev/linter/rules/no-distracting-elements">lint/a11y/noDistractingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the 'marquee' element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;marquee /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Visually distracting elements can cause accessibility issues and should be avoided.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the 'marquee' element.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">q</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">e</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

```jsx
<blink />
```

<pre class="language-text"><code class="language-text">a11y/noDistractingElements.js:1:1 <a href="https://biomejs.dev/linter/rules/no-distracting-elements">lint/a11y/noDistractingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the 'blink' element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;blink /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Visually distracting elements can cause accessibility issues and should be avoided.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the 'blink' element.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

### Valid

```jsx
<div />
```

## Accessibility guidelines

- [WCAG 2.2.2](https://www.w3.org/WAI/WCAG21/Understanding/pause-stop-hide)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
