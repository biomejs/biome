---
title: noConstantCondition (since v1.0.0)
---

**Diagnostic Category: `lint/correctness/noConstantCondition`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-constant-condition" target="_blank"><code>no-constant-condition</code></a>

Disallow constant expressions in conditions

## Examples

### Invalid

```jsx
if (false) {
    doSomethingUnfinished();
}
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:5 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (false) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    doSomethingUnfinished();
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
if (Boolean(1)) {
    doSomethingAlways();
}
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:5 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (Boolean(1)) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    doSomethingAlways();
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
if (undefined) {
    doSomethingUnfinished();
}
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:5 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (undefined) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    doSomethingUnfinished();
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
for (;-2;) {
    doSomethingForever();
}
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:7 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (;-2;) {
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    doSomethingForever();
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
while (typeof x) {
    doSomethingForever();
}
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:8 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (typeof x) {
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    doSomethingForever();
    <strong>3 │ </strong>}
  
</code></pre>

```jsx
var result = 0 ? a : b;
```

<pre class="language-text"><code class="language-text">correctness/noConstantCondition.js:1:14 <a href="https://biomejs.dev/linter/rules/no-constant-condition">lint/correctness/noConstantCondition</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected constant condition.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var result = 0 ? a : b;
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
if (x === 0) {
    doSomething();
}

for (;;) {
    doSomethingForever();
}

while (typeof x === "undefined") {
    doSomething();
}

do {
    doSomething();
} while (x);

var result = x !== 0 ? a : b;

// Exception
while (true) {
    if (x) { break; }
    x = f();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
