---
title: noSemicolonInJsx (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noSemicolonInJsx`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Inspired from: <a href="https://eslint.org/docs/latest/rules/no-semicolons-in-jsx" target="_blank"><code>no-semicolons-in-jsx</code></a>

Remove semicolons from JSX elements.

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

<pre class="language-text"><code class="language-text">suspicious/noSemicolonInJsx.js:4:14 <a href="https://biomejs.dev/linter/rules/no-semicolons-in-jsx">lint/suspicious/noSemicolonInJsx</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">There is suspicious </span><span style="color: Tomato;"><strong>Semicolon</strong></span><span style="color: Tomato;"> in the JSX element.</span>
  
    <strong>2 │ </strong>  return (
    <strong>3 │ </strong>    &lt;div&gt;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>      &lt;div /&gt;;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    &lt;/div&gt;
   <strong>   │ </strong>    
    <strong>6 │ </strong> );
    <strong>7 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>Semicolon</strong></span><span style="color: lightgreen;"> from the JSX element.</span>
  
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
