---
title: Lint Rule noChildrenProp
parent: lint/rules/index
---

# noChildrenProp (since v0.10.0)

> This rule is recommended by Biome.

Prevent passing of **children** as props.

When using JSX, the children should be nested between the opening and closing tags.
When not using JSX, the children should be passed as additional arguments to `React.createElement`.

## Examples

### Invalid

```jsx
<FirstComponent children={'foo'} />
```

<pre class="language-text"><code class="language-text">correctness/noChildrenProp.js:1:17 <a href="https://biomejs.dev/lint/rules/noChildrenProp">lint/correctness/noChildrenProp</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;FirstComponent children={'foo'} /&gt;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The canonical way to pass children in React is to use JSX elements</span>
  
</code></pre>

```jsx
React.createElement('div', { children: 'foo' });
```

<pre class="language-text"><code class="language-text">correctness/noChildrenProp.js:1:30 <a href="https://biomejs.dev/lint/rules/noChildrenProp">lint/correctness/noChildrenProp</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid passing </span><span style="color: Tomato;"><strong>children</strong></span><span style="color: Tomato;"> using a prop</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>React.createElement('div', { children: 'foo' });
   <strong>   │ </strong>                             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The canonical way to pass children in React is to use additional arguments to React.createElement</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
