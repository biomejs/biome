---
title: Lint Rule noRestrictedGlobals
parent: lint/rules/index
---

# noRestrictedGlobals (since v0.10.0)

This rule allows you to specify global variable names that you don’t want to use in your application.

>Disallowing usage of specific global variables can be useful if you want to allow a set of
global variables by enabling an environment, but still want to disallow some of those.


## Examples

### Invalid

```jsx
console.log(event)
```

<pre class="language-text"><code class="language-text">style/noRestrictedGlobals.js:1:13 <a href="https://docs.rome.tools/lint/rules/noRestrictedGlobals">lint/style/noRestrictedGlobals</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Do not use the global variable </span><span style="color: Orange;"><strong>event</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>console.log(event)
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>
  
</code></pre>

### Valid

```jsx
function f(event) {
    console.log(event)
}
```

## Options

Use the options to specify additional globals that you want to restrict in your
source code.

```json
{
    "//": "...",
    "options": {
        "deniedGlobals": ["$", "MooTools"]
    }
}
```

In the example above, the rule will emit a diagnostics if tried to use `$` or `MooTools` without
creating a local variable.

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
