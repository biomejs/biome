---
title: noSuspiciousSemicolonInJsx (not released)
---

**Diagnostic Category: `lint/nursery/noSuspiciousSemicolonInJsx`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

It detects possible "wrong" semicolons inside JSX elements.

Semicolons that appear after a self-closing element or a closing element are usually the result of a typo of a refactor gone wrong.

## Examples

### Invalid

```jsx
const Component = () => {
  return (
    <div>
      <div />;
    </div>
 );
}
```

<pre class="language-text"><code class="language-text">nursery/noSuspiciousSemicolonInJsx.js:4:14 <a href="https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx">lint/nursery/noSuspiciousSemicolonInJsx</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">There is a suspicious </span><span style="color: Tomato;"><strong>semicolon</strong></span><span style="color: Tomato;"> in the JSX element.</span>
  
    <strong>2 │ </strong>  return (
    <strong>3 │ </strong>    &lt;div&gt;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>      &lt;div /&gt;;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    &lt;/div&gt;
   <strong>   │ </strong>    
    <strong>6 │ </strong> );
    <strong>7 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This is usually the result of a typo or some refactor gone wrong.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>semicolon</strong></span><span style="color: lightgreen;">, or move it inside a JSX element.</span>
  
</code></pre>

### Valid

```jsx
const Component = () => {
  return (
    <div>
      <div />
      ;
    </div>
  );
}
const Component2 = () => {
  return (
    <div>
      <span>;</span>
    </div>
  );
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
