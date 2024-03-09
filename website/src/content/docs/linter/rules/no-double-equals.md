---
title: noDoubleEquals (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noDoubleEquals`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/eqeqeq" target="_blank"><code>eqeqeq</code></a>

Require the use of `===` and `!==`

It is generally bad practice to use `==` for comparison instead of
`===`. Double operators will trigger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)
and are thus not prefered. Using strict equality operators is almost
always best practice.

For ergonomic reasons, this rule makes an exception for `== null` for
comparing to both `null` and `undefined`.

## Examples

### Invalid

```jsx
foo == bar
```

<pre class="language-text"><code class="language-text">suspicious/noDoubleEquals.js:1:5 <a href="https://biomejs.dev/linter/rules/no-double-equals">lint/suspicious/noDoubleEquals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>==</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo == bar
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>==</strong></span><span style="color: lightgreen;"> is only allowed when comparing against </span><span style="color: lightgreen;"><strong>null</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo == bar
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Using </span><span style="color: lightgreen;"><strong>==</strong></span><span style="color: lightgreen;"> may be unsafe if you are relying on type coercion</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use </span><span style="color: lightgreen;"><strong>===</strong></span>
  
<strong>  </strong><strong>  1 │ </strong>foo<span style="opacity: 0.8;">·</span>==<span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;">·</span>bar
<strong>  </strong><strong>    │ </strong>      <span style="color: MediumSeaGreen;">+</span>    
</code></pre>

### Valid

```jsx
foo == null
```

```jsx
foo != null
```

```jsx
null == foo
```

```jsx
null != foo
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
