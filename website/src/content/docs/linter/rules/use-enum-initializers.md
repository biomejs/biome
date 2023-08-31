---
title: useEnumInitializers (since v1.0.0)
---


:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Require that each enum member value be explicitly initialized.

_TypeScript_ enums are a practical way to organize semantically related constant values.
Members of enums that don't have explicit values are by default given sequentially increasing numbers.

When the value of enum members are important,
allowing implicit values for enum members can cause bugs if enum declarations are modified over time.

Source: https://typescript-eslint.io/rules/prefer-enum-initializers

## Examples

### Invalid

```ts
enum Version {
    V1,
}
```

<pre class="language-text"><code class="language-text">style/useEnumInitializers.js:1:6 <a href="https://biomejs.dev/lint/rules/useEnumInitializers">lint/style/useEnumInitializers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>enum declaration</strong></span><span style="color: Tomato;"> contains members that are implicitly initialized.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>enum Version {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    V1,
    <strong>3 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This </span><span style="color: rgb(38, 148, 255);"><strong>enum member</strong></span><span style="color: rgb(38, 148, 255);"> should be explicitly initialized.</span>
  
    <strong>1 │ </strong>enum Version {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    V1,
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowing implicit initializations for enum members can cause bugs if enum declarations are modified over time.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Initialize all enum members.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>V1<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">0</span>,
<strong>  </strong><strong>    │ </strong>      <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span> 
</code></pre>

```ts
enum Status {
    Open = 1,
    Close,
}
```

<pre class="language-text"><code class="language-text">style/useEnumInitializers.js:1:6 <a href="https://biomejs.dev/lint/rules/useEnumInitializers">lint/style/useEnumInitializers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>enum declaration</strong></span><span style="color: Tomato;"> contains members that are implicitly initialized.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>enum Status {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    Open = 1,
    <strong>3 │ </strong>    Close,
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This </span><span style="color: rgb(38, 148, 255);"><strong>enum member</strong></span><span style="color: rgb(38, 148, 255);"> should be explicitly initialized.</span>
  
    <strong>1 │ </strong>enum Status {
    <strong>2 │ </strong>    Open = 1,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    Close,
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowing implicit initializations for enum members can cause bugs if enum declarations are modified over time.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Initialize all enum members.</span>
  
<strong>  </strong><strong>  3 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>Close<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">2</span>,
<strong>  </strong><strong>    │ </strong>         <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span> 
</code></pre>

```ts
enum Color {
    Red = "Red",
    Green = "Green",
    Blue,
}
```

<pre class="language-text"><code class="language-text">style/useEnumInitializers.js:1:6 <a href="https://biomejs.dev/lint/rules/useEnumInitializers">lint/style/useEnumInitializers</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>enum declaration</strong></span><span style="color: Tomato;"> contains members that are implicitly initialized.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>enum Color {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    Red = &quot;Red&quot;,
    <strong>3 │ </strong>    Green = &quot;Green&quot;,
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This </span><span style="color: rgb(38, 148, 255);"><strong>enum member</strong></span><span style="color: rgb(38, 148, 255);"> should be explicitly initialized.</span>
  
    <strong>2 │ </strong>    Red = &quot;Red&quot;,
    <strong>3 │ </strong>    Green = &quot;Green&quot;,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    Blue,
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Allowing implicit initializations for enum members can cause bugs if enum declarations are modified over time.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Initialize all enum members.</span>
  
<strong>  </strong><strong>  4 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>Blue<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">B</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">&quot;</span>,
<strong>  </strong><strong>    │ </strong>        <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span> 
</code></pre>

### Valid

```ts
enum Status {
    Open = 1,
    Close = 2,
}
```

```ts
enum Color {
    Red = "Red",
    Green = "Green",
    Blue = "Blue",
}
```

```ts
declare enum Weather {
    Rainy,
    Sunny,
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
