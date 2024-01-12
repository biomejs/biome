---
title: noUselessLabel (since v1.0.0)
---

**Diagnostic Category: `lint/complexity/noUselessLabel`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-extra-label" target="_blank"><code>no-extra-label</code></a>

Disallow unnecessary labels.

If a loop contains no nested loops or switches, labeling the loop is unnecessary.

## Examples

### Invalid

```jsx
loop: while(a) {
    break loop;
}
```

<pre class="language-text"><code class="language-text">complexity/noUselessLabel.js:2:11 <a href="https://biomejs.dev/linter/rules/no-useless-label">lint/complexity/noUselessLabel</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unnecessary </span><span style="color: Tomato;"><strong>label</strong></span><span style="color: Tomato;">.</span>
  
    <strong>1 │ </strong>loop: while(a) {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    break loop;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the unnecessary </span><span style="color: lightgreen;"><strong>label</strong></span><span style="color: lightgreen;">.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">You can achieve the same result without the label.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>break<span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">l</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">p</span>;
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> 
</code></pre>

### Valid

```jsx
outer: while(a) {
    while(b) {
        break outer;
    }
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
