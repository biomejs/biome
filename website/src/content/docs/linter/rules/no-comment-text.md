---
title: noCommentText (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noCommentText`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-comment-textnodes.md" target="_blank"><code>jsx-no-comment-textnodes</code></a>

Prevent comments from being inserted as text nodes

## Examples

### Invalid

```jsx
const a3 = <div>// comment</div>;
```

<pre class="language-text"><code class="language-text">suspicious/noCommentText.js:1:17 <a href="https://biomejs.dev/linter/rules/no-comment-text">lint/suspicious/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a3 = &lt;div&gt;// comment&lt;/div&gt;;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the comments with braces</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">3</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;">/</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">3</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><strong>*</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><strong>*</strong></span><span style="color: MediumSeaGreen;"><strong>/</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
const a4 = <div>/* comment */</div>;
```

<pre class="language-text"><code class="language-text">suspicious/noCommentText.js:1:17 <a href="https://biomejs.dev/linter/rules/no-comment-text">lint/suspicious/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a4 = &lt;div&gt;/* comment */&lt;/div&gt;;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the comments with braces</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>a4<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span>&lt;div&gt;<span style="color: MediumSeaGreen;">{</span>/*<span style="opacity: 0.8;">·</span>comment<span style="opacity: 0.8;">·</span>*/<span style="color: MediumSeaGreen;">}</span>&lt;/div&gt;;
<strong>  </strong><strong>    │ </strong>                <span style="color: MediumSeaGreen;">+</span>             <span style="color: MediumSeaGreen;">+</span>       
</code></pre>

```jsx
const a5 = <div>/** comment */</div>;
```

<pre class="language-text"><code class="language-text">suspicious/noCommentText.js:1:17 <a href="https://biomejs.dev/linter/rules/no-comment-text">lint/suspicious/noCommentText</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Wrap </span><span style="color: Tomato;"><strong>comments</strong></span><span style="color: Tomato;"> inside children within </span><span style="color: Tomato;"><strong>braces</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a5 = &lt;div&gt;/** comment */&lt;/div&gt;;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the comments with braces</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">5</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;"><strong>*</strong></span><span style="color: Tomato;">*</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">*</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">5</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">*</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">*</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><strong>}</strong></span><span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
const a = <div>{/* comment */}</div>;
const a1 = <div>{/** comment */}</div>;
const a2 = <div className={"cls" /* comment */}></div>;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
