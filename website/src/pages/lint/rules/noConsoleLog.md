---
title: Lint Rule noConsoleLog
parent: lint/rules/index
---

# noConsoleLog (since v1.0.0)

Disallow the use of `console.log`

## Examples

### Invalid

```jsx
console.log()
```

<pre class="language-text"><code class="language-text">suspicious/noConsoleLog.js:1:1 <a href="https://biomejs.dev/lint/rules/noConsoleLog">lint/suspicious/noConsoleLog</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Don't use </span><span style="color: Orange;"><strong>console.log</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>console.log()
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>console.log</strong></span><span style="color: rgb(38, 148, 255);"> is usually a tool for debugging and you don't want to have that in production.</span>
  
</code></pre>

## Valid

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
