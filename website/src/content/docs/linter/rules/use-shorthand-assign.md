---
title: useShorthandAssign (since vnext)
---

**Diagnostic Category: `lint/nursery/useShorthandAssign`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Require assignment operator shorthand where possible

JavaScript provides shorthand operators that combine variable assignment and some simple mathematical operations

Source: https://eslint.org/docs/latest/rules/operator-assignment/

## Examples

### Invalid

```jsx
var a = 1;
a = a + 1;
```

<pre class="language-text"><code class="language-text">nursery/useShorthandAssign.js:2:1 <a href="https://biomejs.dev/lint/rules/use-shorthand-assign">lint/nursery/useShorthandAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Assignment (=) can be replaced with operator assignment +=</span>
  
    <strong>1 │ </strong>var a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = a + 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
</code></pre>

## Valid

```jsx
var a = 1;
a += 1;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
