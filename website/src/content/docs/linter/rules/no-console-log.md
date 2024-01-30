---
title: noConsoleLog (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noConsoleLog`**

Inspired from: <a href="https://eslint.org/docs/latest/rules/no-console" target="_blank"><code>no-console</code></a>

Disallow the use of `console.log`

## Examples

### Invalid

```jsx
console.log()
```

<pre class="language-text"><code class="language-text">suspicious/noConsoleLog.js:1:1 <a href="https://biomejs.dev/linter/rules/no-console-log">lint/suspicious/noConsoleLog</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't use </span><span style="color: Orange;"><strong>console.log</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>console.log()
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;"><strong>console.log</strong></span><span style="color: lightgreen;"> is usually a tool for debugging and you don't want to have that in production.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If it is not for debugging purpose then using </span><span style="color: lightgreen;"><strong>console.info</strong></span><span style="color: lightgreen;"> might be more appropriate.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove console.log</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">.</span><span style="color: Tomato;">l</span><span style="color: Tomato;">o</span><span style="color: Tomato;">g</span><span style="color: Tomato;">(</span><span style="color: Tomato;">)</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

### Valid

```jsx
console.info("info");
console.warn("warn");
console.error("error");
console.assert(true);
console.table(["foo", "bar"]);
const console = { log() {} };
console.log();
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
