---
title: noDuplicateElseIf (not released)
---

**Diagnostic Category: `lint/nursery/noDuplicateElseIf`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-dupe-else-if" target="_blank"><code>no-dupe-else-if</code></a>

Disallow duplicate conditions in if-else-if chains

if-else-if chains are commonly used when there is a need to execute only one branch
(or at most one branch) out of several possible branches, based on certain conditions.

Two identical test conditions in the same chain are almost always a mistake in the code.
Unless there are side effects in the expressions,
a duplicate will evaluate to the same true or false value as the identical expression earlier in the chain,
meaning that its branch can never execute.

Please note that this rule does not compare conditions from the chain with conditions inside statements

## Examples

### Invalid

```jsx
if (a) {
    foo();
} else if (b) {
    bar();
} else if (b) {
    baz();
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateElseIf.js:5:12 <a href="https://biomejs.dev/linter/rules/no-duplicate-else-if">lint/nursery/noDuplicateElseIf</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This branch can never execute. Its condition is a duplicate or covered by previous conditions in the if-else-if chain.</span>
  
    <strong>3 │ </strong>} else if (b) {
    <strong>4 │ </strong>    bar();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>} else if (b) {
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>    baz();
    <strong>7 │ </strong>}
  
</code></pre>

### Valid

```jsx
if (a) {
    foo();
} else if (b) {
    bar();
} else if (c) {
    baz();
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
