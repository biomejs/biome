---
title: useJsxKeyInIterable (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/useJsxKeyInIterable`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-key.md" target="_blank"><code>jsx-key</code></a>

Disallow missing key props in iterators/collection literals.

Warn if an element that likely requires a key prop--namely, one present in an array literal or an arrow function expression.
Check out React documentation for [explanation on the why does React need keys.](https://react.dev/learn/rendering-lists#why-does-react-need-keys)

## Examples

### Invalid

```jsx
[<Hello />];
```

<pre class="language-text"><code class="language-text">nursery/useJsxKeyInIterable.js:1:2 <a href="https://biomejs.dev/linter/rules/use-jsx-key-in-iterable">lint/nursery/useJsxKeyInIterable</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Missing </span><span style="color: Orange;"><strong>key</strong></span><span style="color: Orange;"> property for this element in iterable.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>[&lt;Hello /&gt;];
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The order of the items may change, and having a key can help React identify which item was moved.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check the </span><span style="color: lightgreen;"><a href="https://react.dev/learn/rendering-lists#why-does-react-need-keys">React documentation</a></span><span style="color: lightgreen;">. </span>
  
</code></pre>

```jsx
data.map((x) => <Hello>{x}</Hello>);
```

<pre class="language-text"><code class="language-text">nursery/useJsxKeyInIterable.js:1:17 <a href="https://biomejs.dev/linter/rules/use-jsx-key-in-iterable">lint/nursery/useJsxKeyInIterable</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Missing </span><span style="color: Orange;"><strong>key</strong></span><span style="color: Orange;"> property for this element in iterable.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>data.map((x) =&gt; &lt;Hello&gt;{x}&lt;/Hello&gt;);
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The order of the items may change, and having a key can help React identify which item was moved.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Check the </span><span style="color: lightgreen;"><a href="https://react.dev/learn/rendering-lists#why-does-react-need-keys">React documentation</a></span><span style="color: lightgreen;">. </span>
  
</code></pre>

### Valid

```jsx
[<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];
data.map((x) => <Hello key={x.id}>{x}</Hello>);
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
