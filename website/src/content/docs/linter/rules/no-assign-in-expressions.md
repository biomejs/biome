---
title: noAssignInExpressions (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noAssignInExpressions`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Inspired from: <a href="https://eslint.org/docs/latest/rules/no-cond-assign" target="_blank"><code>no-cond-assign</code></a>

Disallow assignments in expressions.

In expressions, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
Moreover, the use of assignments in expressions is confusing.
Indeed, expressions are often considered as side-effect free.

## Examples

### Invalid

```ts
let a, b;
a = (b = 1) + 1;
```

<pre class="language-text"><code class="language-text">suspicious/noAssignInExpressions.js:2:6 <a href="https://biomejs.dev/linter/rules/no-assign-in-expressions">lint/suspicious/noAssignInExpressions</a> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>assignment</strong></span><span style="color: Tomato;"> should not be in an </span><span style="color: Tomato;"><strong>expression</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>let a, b;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = (b = 1) + 1;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The use of assignments in expressions is confusing.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Expressions are often considered as side-effect free.</span>
  
</code></pre>

```ts
let a;
if (a = 1) {
}
```

<pre class="language-text"><code class="language-text">suspicious/noAssignInExpressions.js:2:5 <a href="https://biomejs.dev/linter/rules/no-assign-in-expressions">lint/suspicious/noAssignInExpressions</a> ━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>assignment</strong></span><span style="color: Tomato;"> should not be in an </span><span style="color: Tomato;"><strong>expression</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>let a;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>if (a = 1) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The use of assignments in expressions is confusing.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Expressions are often considered as side-effect free.</span>
  
</code></pre>

```ts
function f(a) {
    return a = 1;
}
```

<pre class="language-text"><code class="language-text">suspicious/noAssignInExpressions.js:2:12 <a href="https://biomejs.dev/linter/rules/no-assign-in-expressions">lint/suspicious/noAssignInExpressions</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>assignment</strong></span><span style="color: Tomato;"> should not be in an </span><span style="color: Tomato;"><strong>expression</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>function f(a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    return a = 1;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The use of assignments in expressions is confusing.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Expressions are often considered as side-effect free.</span>
  
</code></pre>

### Valid

```ts
let a;
a = 1;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
