---
title: noDuplicateCase (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noDuplicateCase`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-duplicate-case" target="_blank"><code>no-duplicate-case</code></a>

Disallow duplicate case labels.

If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.

## Examples

### Invalid

```jsx
switch (a) {
    case 1:
        break;
    case 1:
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateCase.js:4:10 <a href="https://biomejs.dev/linter/rules/no-duplicate-case">lint/suspicious/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case 1:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case 1:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The first similar label is here:</span>
  
    <strong>1 │ </strong>switch (a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    case 1:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>        break;
    <strong>4 │ </strong>    case 1:
  
</code></pre>

```jsx
switch (a) {
    case one:
        break;
    case one:
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateCase.js:4:10 <a href="https://biomejs.dev/linter/rules/no-duplicate-case">lint/suspicious/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case one:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case one:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The first similar label is here:</span>
  
    <strong>1 │ </strong>switch (a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    case one:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>        break;
    <strong>4 │ </strong>    case one:
  
</code></pre>

```jsx
switch (a) {
    case "1":
        break;
    case "1":
        break;
    default:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateCase.js:4:10 <a href="https://biomejs.dev/linter/rules/no-duplicate-case">lint/suspicious/noDuplicateCase</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate case label.</span>
  
    <strong>2 │ </strong>    case &quot;1&quot;:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case &quot;1&quot;:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>        break;
    <strong>6 │ </strong>    default:
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The first similar label is here:</span>
  
    <strong>1 │ </strong>switch (a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    case &quot;1&quot;:
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>        break;
    <strong>4 │ </strong>    case &quot;1&quot;:
  
</code></pre>

### Valid

```jsx
switch (a) {
    case 1:
        break;
    case 2:
        break;
    default:
        break;
}
```

```jsx
switch (a) {
    case one:
        break;
    case two:
        break;
    default:
        break;
}
```

```jsx
switch (a) {
    case "1":
        break;
    case "2":
        break;
    default:
        break;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
