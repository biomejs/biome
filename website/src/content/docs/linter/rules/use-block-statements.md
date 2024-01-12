---
title: useBlockStatements (since v1.0.0)
---

**Diagnostic Category: `lint/style/useBlockStatements`**

Source: <a href="https://eslint.org/docs/latest/rules/curly" target="_blank"><code>curly</code></a>

Requires following curly brace conventions.

JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.

## Examples

### Invalid

```jsx
 if (x) x;
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:2 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> if (x) x;
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="opacity: 0.8;">·</span>if<span style="opacity: 0.8;">·</span>(x)<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">{</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>x;<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">}</span>
<strong>  </strong><strong>    │ </strong>        <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>  <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>
</code></pre>

```jsx
 if (x) {
   x;
 } else y;
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:3:4 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
    <strong>1 │ </strong> if (x) {
    <strong>2 │ </strong>   x;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong> } else y;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
<strong>  </strong><strong>  3 │ </strong><span style="opacity: 0.8;">·</span>}<span style="opacity: 0.8;">·</span>else<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">{</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>y;<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">}</span>
<strong>  </strong><strong>    │ </strong>        <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>  <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>
</code></pre>

```jsx
if (x) {
  x;
} else if (y) y;
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:3:8 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
    <strong>1 │ </strong>if (x) {
    <strong>2 │ </strong>  x;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} else if (y) y;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
<strong>  </strong><strong>  3 │ </strong>}<span style="opacity: 0.8;">·</span>else<span style="opacity: 0.8;">·</span>if<span style="opacity: 0.8;">·</span>(y)<span style="opacity: 0.8;">·</span><span style="color: MediumSeaGreen;">{</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>y;<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">}</span>
<strong>  </strong><strong>    │ </strong>              <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>  <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>
</code></pre>

```jsx
   for (;;);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:4 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>   for (;;);
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">;</span><span style="color: Tomato;">;</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><strong>;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
   for (p in obj);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:4 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>   for (p in obj);
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">p</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">o</span><span style="color: Tomato;">b</span><span style="color: Tomato;">j</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><strong>;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">j</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
  for (x of xs);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:3 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>  for (x of xs);
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">o</span><span style="color: Tomato;">f</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;">s</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><strong>;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
  do;
  while (x);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:3 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>  do;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  while (x);
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">d</span><span style="color: Tomato;">o</span><span style="color: Tomato;"><strong>;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>    while (x);
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

```jsx
   while (x);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:4 <a href="https://biomejs.dev/linter/rules/use-block-statements">lint/style/useBlockStatements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Block statements are preferred in this position.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>   while (x);
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Wrap the statement with a `JsBlockStatement`</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">w</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;"><strong>;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">w</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: MediumSeaGreen;"><strong>{</strong></span><span style="color: MediumSeaGreen;"><strong>}</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
  with (x);
```

<pre class="language-text"><code class="language-text">style/useBlockStatements.js:1:3 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">`with` statements are not allowed in strict mode</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>  with (x);
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
