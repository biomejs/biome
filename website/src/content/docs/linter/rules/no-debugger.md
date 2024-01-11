---
title: noDebugger (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noDebugger`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-debugger" target="_blank"><code>no-debugger</code></a>

Disallow the use of `debugger`

## Examples

### Invalid

```jsx
debugger;
```

<pre class="language-text"><code class="language-text">suspicious/noDebugger.js:1:1 <a href="https://biomejs.dev/linter/rules/no-debugger">lint/suspicious/noDebugger</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>debugger</strong></span><span style="color: Tomato;"> statement.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>debugger;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove debugger statement</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">g</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

### Valid

```jsx
const test = { debugger: 1 };
test.debugger;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
