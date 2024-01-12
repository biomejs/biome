---
title: noChildrenProp (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noChildrenProp`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-children-prop.md" target="_blank"><code>no-children-prop</code></a>

Prevent passing of **children** as props.

When using JSX, the children should be nested between the opening and closing tags.
When not using JSX, the children should be passed as additional arguments to `React.createElement`.

## Examples

### Invalid

```jsx
<FirstComponent children={'foo'} />
```

<pre class="language-text"><code class="language-text">correctness/noChildrenProp.js:1:17 <a href="https://biomejs.dev/linter/rules/no-children-prop">lint/correctness/noChildrenProp</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;FirstComponent children={'foo'} /&gt;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The canonical way to pass children in React is to use JSX elements</span>
  
</code></pre>

```jsx
React.createElement('div', { children: 'foo' });
```

<pre class="language-text"><code class="language-text">correctness/noChildrenProp.js:1:30 <a href="https://biomejs.dev/linter/rules/no-children-prop">lint/correctness/noChildrenProp</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.createElement('div', { children: 'foo' });
   <strong>   │ </strong>                             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The canonical way to pass children in React is to use additional arguments to React.createElement</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
