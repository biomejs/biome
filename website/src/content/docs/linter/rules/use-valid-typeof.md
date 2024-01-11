---
title: useValidTypeof (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/useValidTypeof`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/valid-typeof" target="_blank"><code>valid-typeof</code></a>

This rule verifies the result of `typeof $expr` unary expressions is being compared to valid values, either string literals containing valid type names or other `typeof` expressions

## Examples

### Invalid

```jsx
typeof foo === "strnig"
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:16 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo === &quot;strnig&quot;
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a valid type name</span>
  
</code></pre>

```jsx
typeof foo == "undefimed"
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:15 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo == &quot;undefimed&quot;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a valid type name</span>
  
</code></pre>

```jsx
typeof bar != "nunber"
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:15 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof bar != &quot;nunber&quot;
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a valid type name</span>
  
</code></pre>

```jsx
typeof bar !== "fucntion"
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:16 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof bar !== &quot;fucntion&quot;
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a valid type name</span>
  
</code></pre>

```jsx
typeof foo === undefined
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:16 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo === undefined
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a string literal</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Compare the result of `typeof` with a valid type name</span>
  
<strong>  </strong><strong>  1 │ </strong>typeof<span style="opacity: 0.8;">·</span>foo<span style="opacity: 0.8;">·</span>===<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">&quot;</span>undefined<span style="color: MediumSeaGreen;">&quot;</span>
<strong>  </strong><strong>    │ </strong>               <span style="color: MediumSeaGreen;">+</span>         <span style="color: MediumSeaGreen;">+</span>
</code></pre>

```jsx
typeof bar == Object
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:15 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof bar == Object
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a string literal</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Compare the result of `typeof` with a valid type name</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">t</span><span style="color: Tomato;">y</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">o</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>j</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">y</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>j</strong></span><span style="color: MediumSeaGreen;"><strong>e</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
typeof foo === baz
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:16 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo === baz
   <strong>   │ </strong>               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a string literal</span>
  
</code></pre>

```jsx
typeof foo == 5
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:15 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo == 5
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a string literal</span>
  
</code></pre>

```jsx
typeof foo == -5
```

<pre class="language-text"><code class="language-text">suspicious/useValidTypeof.js:1:15 <a href="https://biomejs.dev/linter/rules/use-valid-typeof">lint/suspicious/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>typeof foo == -5
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">not a string literal</span>
  
</code></pre>

### Valid

```jsx
typeof foo === "string"
```

```jsx
typeof bar == "undefined"
```

```jsx
typeof bar === typeof qux
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
