---
title: noFunctionAssign (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noFunctionAssign`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-func-assign" target="_blank"><code>no-func-assign</code></a>

Disallow reassigning function declarations.

## Examples

### Invalid

```jsx
function foo() { };
foo = bar;
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:1:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo() { };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>foo = bar;
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
    <strong>1 │ </strong>function foo() { };
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>foo = bar;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
function foo() {
    foo = bar;
 }
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:1:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo() {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    foo = bar;
    <strong>3 │ </strong> }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
    <strong>1 │ </strong>function foo() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    foo = bar;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong> }
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
foo = bar;
function foo() { };
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:2:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
    <strong>1 │ </strong>foo = bar;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>function foo() { };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foo = bar;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>function foo() { };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassignment happens here because the function declaration is hoisted.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
[foo] = bar;
function foo() { };
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:2:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
    <strong>1 │ </strong>[foo] = bar;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>function foo() { };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>[foo] = bar;
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>function foo() { };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassignment happens here because the function declaration is hoisted.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
({ x: foo = 0 } = bar);
function foo() { };
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:2:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
    <strong>1 │ </strong>({ x: foo = 0 } = bar);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>function foo() { };
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ x: foo = 0 } = bar);
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>function foo() { };
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassignment happens here because the function declaration is hoisted.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
function foo() {
    [foo] = bar;
 }
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:1:10 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo() {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    [foo] = bar;
    <strong>3 │ </strong> }
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
    <strong>1 │ </strong>function foo() {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    [foo] = bar;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong> }
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

```jsx
(function () {
    ({ x: foo = 0 } = bar);
    function foo() { };
 })();
```

<pre class="language-text"><code class="language-text">suspicious/noFunctionAssign.js:3:14 <a href="https://biomejs.dev/linter/rules/no-function-assign">lint/suspicious/noFunctionAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not reassign a function declaration.</span>
  
    <strong>1 │ </strong>(function () {
    <strong>2 │ </strong>    ({ x: foo = 0 } = bar);
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>    function foo() { };
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong> })();
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassigned here.</span>
  
    <strong>1 │ </strong>(function () {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    ({ x: foo = 0 } = bar);
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    function foo() { };
    <strong>4 │ </strong> })();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Reassignment happens here because the function declaration is hoisted.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Use a local variable instead.</span>
  
</code></pre>

### Valid

```jsx
function foo() {
    var foo = bar;
 }
```

```jsx
function foo(foo) {
    foo = bar;
 }
```

```jsx
function foo() {
    var foo;
    foo = bar;
 }
```

```jsx
var foo = () => {};
foo = bar;
```

```jsx
var foo = function() {};
foo = bar;
```

```jsx
var foo = function() {
    foo = bar;
 };
```

```jsx
import bar from 'bar';
function foo() {
    var foo = bar;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
