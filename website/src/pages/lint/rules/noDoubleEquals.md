---
title: Lint Rule noDoubleEquals
parent: lint/rules/index
---

# noDoubleEquals (since v1.0.0)

> This rule is recommended by Biome.

Require the use of `===` and `!==`

It is generally bad practice to use `==` for comparison instead of
`===`. Double operators will triger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)
and are thus not prefered. Using strict equality operators is almost
always best practice.

For ergonomic reasons, this rule makes an exception for `== null` for
comparing to both `null` and `undefined`.

## Examples

### Invalid

```jsx
foo == bar
```

<pre class="language-text"><code class="language-text">suspicious/noDoubleEquals.js:1:5 <a href="https://biomejs.dev/lint/rules/noDoubleEquals">lint/suspicious/noDoubleEquals</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>===</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>==</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo == bar
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>==</strong></span><span style="color: rgb(38, 148, 255);"> is only allowed when comparing against </span><span style="color: rgb(38, 148, 255);"><strong>null</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo == bar
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Using </span><span style="color: rgb(38, 148, 255);"><strong>===</strong></span><span style="color: rgb(38, 148, 255);"> may be unsafe if you are relying on type coercion</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>===</strong></span>
  
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
