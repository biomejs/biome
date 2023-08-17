---
title: Lint Rule noImportAssign
parent: lint/rules/index
---

# noImportAssign (since v0.9.0)

> This rule is recommended by Rome.

Disallow assigning to imported bindings

## Examples

### Invalid

```jsx
import x from "y";
x = 1;
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:1 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import x from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>x = 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import x from &quot;y&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>x = 1;
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

```jsx
import y from "y";
[y] = 1;
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:2 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import y from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>[y] = 1;
   <strong>   │ </strong> <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import y from &quot;y&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>[y] = 1;
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

```jsx
import z from "y";
({ z } = 1); /// ```
```js,expect_diagnostic
import a from "y";
[...a] = 1;
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:3:4 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">unterminated template literal</span>
  
    <strong>1 │ </strong>import z from &quot;y&quot;;
    <strong>2 │ </strong>({ z } = 1); /// ```
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>```js,expect_diagnostic
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>import a from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>[...a] = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>
   <strong>   │ </strong>
  
</code></pre>

```jsx
import b from "y";
({ ...b } = 1);
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:7 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import b from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>({ ...b } = 1);
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import b from &quot;y&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>({ ...b } = 1);
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

```jsx
import c from "y";
for (c in y) {};
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:6 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import c from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>for (c in y) {};
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import c from &quot;y&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>for (c in y) {};
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

```jsx
import d from "y";
d += 1;
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:1 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import d from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>d += 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import d from &quot;y&quot;;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>d += 1;
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

```jsx
import * as e from "y";
e = 1;
```

<pre class="language-text"><code class="language-text">suspicious/noImportAssign.js:2:1 <a href="https://biomejs.dev/lint/rules/noImportAssign">lint/suspicious/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <strong>1 │ </strong>import * as e from &quot;y&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>e = 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import * as e from &quot;y&quot;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>e = 1;
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
