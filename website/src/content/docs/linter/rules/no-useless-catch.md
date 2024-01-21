---
title: noUselessCatch (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessCatch`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-useless-catch" target="_blank"><code>no-useless-catch</code></a>

Disallow unnecessary `catch` clauses.

A `catch` clause that only rethrows the original error is redundant,
and has no effect on the runtime behavior of the program.
These redundant clauses can be a source of confusion and code bloat,
so it’s better to disallow these unnecessary `catch` clauses.

## Examples

### Invalid

```jsx
try {
    doSomething();
} catch(e) {
    throw e;
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessCatch.js:4:5 <a href="https://biomejs.dev/linter/rules/no-useless-catch">lint/complexity/noUselessCatch</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>catch</strong></span><span style="color: Tomato;"> clause that only rethrows the original error is redundant.</span>
  
    <strong>2 │ </strong>    doSomething();
    <strong>3 │ </strong>} catch(e) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    throw e;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>}
    <strong>6 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">These unnecessary </span><span style="color: lightgreen;"><strong>catch</strong></span><span style="color: lightgreen;"> clauses can be confusing. It is recommended to remove them.</span>
  
</code></pre>

```jsx
try {
    doSomething();
} catch(e) {
    throw e;
} finally {
    doCleanUp();
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessCatch.js:4:5 <a href="https://biomejs.dev/linter/rules/no-useless-catch">lint/complexity/noUselessCatch</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>catch</strong></span><span style="color: Tomato;"> clause that only rethrows the original error is redundant.</span>
  
    <strong>2 │ </strong>    doSomething();
    <strong>3 │ </strong>} catch(e) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    throw e;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>} finally {
    <strong>6 │ </strong>    doCleanUp();
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">These unnecessary </span><span style="color: lightgreen;"><strong>catch</strong></span><span style="color: lightgreen;"> clauses can be confusing. It is recommended to remove them.</span>
  
</code></pre>

### Valid

```jsx
try {
    doSomething();
} catch(e) {
    doSomethingWhenCatch();
    throw e;
}
```

```jsx
try {
    doSomething();
} catch(e) {
    handleError(e);
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
