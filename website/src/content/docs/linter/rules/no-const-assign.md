---
title: noConstAssign (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noConstAssign`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Prevents from having `const` variables being re-assigned.

Trying to assign a value to a `const` will cause an `TypeError` when the code is executed.

## Examples

### Invalid

```jsx
const a = 1;
a = 4;
```

<pre class="language-text"><code class="language-text">correctness/noConstAssign.js:2:1 <a href="https://biomejs.dev/linter/rules/no-const-assign">lint/correctness/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Can't assign </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = 4;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variable is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 1;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a = 4;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
const a = 2;
a += 1;
```

<pre class="language-text"><code class="language-text">correctness/noConstAssign.js:2:1 <a href="https://biomejs.dev/linter/rules/no-const-assign">lint/correctness/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Can't assign </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 2;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a += 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variable is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 2;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a += 1;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
const a = 1;
++a;
```

<pre class="language-text"><code class="language-text">correctness/noConstAssign.js:2:3 <a href="https://biomejs.dev/linter/rules/no-const-assign">lint/correctness/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Can't assign </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>++a;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variable is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 1;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>++a;
    <strong>3 │ </strong>
  
</code></pre>

```jsx
const a = 1, b = 2;

a = 2;
```

<pre class="language-text"><code class="language-text">correctness/noConstAssign.js:3:1 <a href="https://biomejs.dev/linter/rules/no-const-assign">lint/correctness/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Can't assign </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 1, b = 2;
    <strong>2 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>a = 2;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variable is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 1, b = 2;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
    <strong>3 │ </strong>a = 2;
  
</code></pre>

### Valid

```jsx
const a = 10;
let b = 10;
b = 20;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
