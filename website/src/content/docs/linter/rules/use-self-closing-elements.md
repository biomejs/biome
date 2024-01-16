---
title: useSelfClosingElements (since v1.0.0)
---

**Diagnostic Category: `lint/style/useSelfClosingElements`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.style/rules/default/jsx-self-closing-comp" target="_blank"><code>jsx-self-closing-comp</code></a>

Prevent extra closing tags for components without children

## Examples

### Invalid

```jsx
<div></div>
```

<pre class="language-text"><code class="language-text">style/useSelfClosingElements.js:1:1 <a href="https://biomejs.dev/linter/rules/use-self-closing-elements">lint/style/useSelfClosingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div&gt;&lt;/div&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a SelfClosingElement instead</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;"><strong>&gt;</strong></span><span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;">/</span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>i</strong></span><span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;">&gt;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">&gt;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
<Component></Component>
```

<pre class="language-text"><code class="language-text">style/useSelfClosingElements.js:1:1 <a href="https://biomejs.dev/linter/rules/use-self-closing-elements">lint/style/useSelfClosingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;Component&gt;&lt;/Component&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a SelfClosingElement instead</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">C</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><strong>&gt;</strong></span><span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;">/</span><span style="color: Tomato;"><strong>C</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;">&gt;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">C</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">&gt;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
<Foo.bar></Foo.bar>
```

<pre class="language-text"><code class="language-text">style/useSelfClosingElements.js:1:1 <a href="https://biomejs.dev/linter/rules/use-self-closing-elements">lint/style/useSelfClosingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;Foo.bar&gt;&lt;/Foo.bar&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Use a SelfClosingElement instead</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">.</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><strong>&gt;</strong></span><span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;">/</span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;">&gt;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">F</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">&gt;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
<div />
```

```jsx
<div>child</div>
```

```jsx
<Component />
```

```jsx
<Component>child</Component>
```

```jsx
<Foo.bar />
```

```jsx
<Foo.bar>child</Foo.bar>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
