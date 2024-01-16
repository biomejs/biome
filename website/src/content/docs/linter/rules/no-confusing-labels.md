---
title: noConfusingLabels (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noConfusingLabels`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Inspired from: <a href="https://eslint.org/docs/latest/rules/no-labels" target="_blank"><code>no-labels</code></a>

Disallow labeled statements that are not loops.

Labeled statements in JavaScript are used in conjunction with `break` and `continue` to control flow around multiple loops.
Their use for other statements is suspicious and unfamiliar.

## Examples

### Invalid

```jsx
label: f();
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://biomejs.dev/linter/rules/no-confusing-labels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: f();
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only loops should be labeled.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: {
    f();
    break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://biomejs.dev/linter/rules/no-confusing-labels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f();
    <strong>3 │ </strong>    break label;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only loops should be labeled.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: if (a) {
    f()
    break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://biomejs.dev/linter/rules/no-confusing-labels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: if (a) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    f()
    <strong>3 │ </strong>    break label;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only loops should be labeled.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

```jsx
label: switch (a) {
    case 0:
        break label;
}
```

<pre class="language-text"><code class="language-text">suspicious/noConfusingLabels.js:1:1 <a href="https://biomejs.dev/linter/rules/no-confusing-labels">lint/suspicious/noConfusingLabels</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>label: switch (a) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    case 0:
    <strong>3 │ </strong>        break label;
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Only loops should be labeled.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">The use of labels for other statements is suspicious and unfamiliar.</span>
  
</code></pre>

### Valid

```jsx
outer: while (a) {
    while(b) {
        break outer;
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
