---
title: noConfusingArrow (since v1.0.0)
---


Disallow arrow functions where they could be confused with comparisons.

Arrow functions (`=>`) are similar in syntax to some comparison operators (`>`, `<`, `<=`, and `>=`).
This rule warns against using the arrow function syntax in places where it could be confused with a comparison operator.

Source: https://eslint.org/docs/latest/rules/no-confusing-arrow

## Examples

### Invalid

```jsx
var x = a => 1 ? 2 : 3;
```

<pre class="language-text"><code class="language-text">nursery/noConfusingArrow.js:1:11 <a href="https://biomejs.dev/linter/rules/no-confusing-arrow">lint/nursery/noConfusingArrow</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Fat arrows can be confused with some comparison operators (</span><span style="color: Orange;"><strong>&lt;</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>&gt;</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>&lt;=</strong></span><span style="color: Orange;">, </span><span style="color: Orange;"><strong>&gt;=</strong></span><span style="color: Orange;">).</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var x = a =&gt; 1 ? 2 : 3;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

## Valid

```jsx
var x = (a) => 1 ? 2 : 3;

var x = a => (1 ? 2 : 3);

var x = (a) => (1 ? 2 : 3);

var x = a => { return 1 ? 2 : 3; };

var x = (a) => { return 1 ? 2 : 3; };
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
