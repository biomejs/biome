---
title: useDefaultSwitchClauseLast (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/useDefaultSwitchClauseLast`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/default-case-last" target="_blank"><code>default-case-last</code></a>

Enforce default clauses in switch statements to be last

A switch statement can optionally have a default clause.

If present, it’s usually the last clause, but it doesn’t need to be. It is also allowed to put the default clause before all case clauses, or anywhere between.
The behavior is mostly the same as if it was the last clause.

The default block will be still executed only if there is no match in the case clauses (including those defined after the default),
but there is also the ability to “fall through” from the default clause to the following clause in the list.
However, such flow is not common and it would be confusing to the readers.

Even if there is no "fall through" logic, it’s still unexpected to see the default clause before or between the case clauses. By convention, it is expected to be the last clause.

## Examples

### Invalid

```jsx
switch (foo) {
    default:
        break;
    case 0:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/useDefaultSwitchClauseLast.js:2:5 <a href="https://biomejs.dev/linter/rules/use-default-switch-clause-last">lint/suspicious/useDefaultSwitchClauseLast</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default</strong></span><span style="color: Tomato;"> clause should be the last </span><span style="color: Tomato;"><strong>switch</strong></span><span style="color: Tomato;"> clause.</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    case 0:
    <strong>5 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The following </span><span style="color: lightgreen;"><strong>case</strong></span><span style="color: lightgreen;"> clause is here:</span>
  
    <strong>2 │ </strong>    default:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Regardless its position, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause is always executed when there is no match. To avoid confusion, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause should be the last </span><span style="color: lightgreen;"><strong>switch</strong></span><span style="color: lightgreen;"> clause.</span>
  
</code></pre>

```jsx
switch (foo) {
    default:
        f();
    case 0:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/useDefaultSwitchClauseLast.js:2:5 <a href="https://biomejs.dev/linter/rules/use-default-switch-clause-last">lint/suspicious/useDefaultSwitchClauseLast</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default</strong></span><span style="color: Tomato;"> clause should be the last </span><span style="color: Tomato;"><strong>switch</strong></span><span style="color: Tomato;"> clause.</span>
  
    <strong>1 │ </strong>switch (foo) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>        f();
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>    case 0:
    <strong>5 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The following </span><span style="color: lightgreen;"><strong>case</strong></span><span style="color: lightgreen;"> clause is here:</span>
  
    <strong>2 │ </strong>    default:
    <strong>3 │ </strong>        f();
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    case 0:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Regardless its position, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause is always executed when there is no match. To avoid confusion, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause should be the last </span><span style="color: lightgreen;"><strong>switch</strong></span><span style="color: lightgreen;"> clause.</span>
  
</code></pre>

```jsx
switch (foo) {
    case 0:
        break;
    default:
    case 1:
        break;
}
```

<pre class="language-text"><code class="language-text">suspicious/useDefaultSwitchClauseLast.js:4:5 <a href="https://biomejs.dev/linter/rules/use-default-switch-clause-last">lint/suspicious/useDefaultSwitchClauseLast</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The </span><span style="color: Tomato;"><strong>default</strong></span><span style="color: Tomato;"> clause should be the last </span><span style="color: Tomato;"><strong>switch</strong></span><span style="color: Tomato;"> clause.</span>
  
    <strong>2 │ </strong>    case 0:
    <strong>3 │ </strong>        break;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>    default:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>    case 1:
    <strong>6 │ </strong>        break;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The following </span><span style="color: lightgreen;"><strong>case</strong></span><span style="color: lightgreen;"> clause is here:</span>
  
    <strong>3 │ </strong>        break;
    <strong>4 │ </strong>    default:
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    case 1:
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>6 │ </strong>        break;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>7 │ </strong>}
    <strong>8 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Regardless its position, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause is always executed when there is no match. To avoid confusion, the </span><span style="color: lightgreen;"><strong>default</strong></span><span style="color: lightgreen;"> clause should be the last </span><span style="color: lightgreen;"><strong>switch</strong></span><span style="color: lightgreen;"> clause.</span>
  
</code></pre>

### Valid

```jsx
switch (foo) {
    case 0:
        break;
    case 1:
    default:
        break;
}
```

```jsx
switch (foo) {
    case 0:
        break;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
