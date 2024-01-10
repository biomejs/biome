---
title: noDuplicateParameters (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noDuplicateParameters`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-dupe-args" target="_blank"><code>no-dupe-args</code></a>

Disallow duplicate function parameter name.

If more than one parameter has the same name in a function definition,
the last occurrence overrides the preceding occurrences.
A duplicated name might be a typing error.

## Examples

### Invalid

```jsx
var f = function(a, b, b) {}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateParameters.js:1:24 <a href="https://biomejs.dev/linter/rules/no-duplicate-parameters">lint/suspicious/noDuplicateParameters</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate parameter name.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var f = function(a, b, b) {}
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The parameter overrides a preceding parameter by using the same name.</span>
  
</code></pre>

```jsx
function b(a, b, b) {}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateParameters.js:1:18 <a href="https://biomejs.dev/linter/rules/no-duplicate-parameters">lint/suspicious/noDuplicateParameters</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate parameter name.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function b(a, b, b) {}
   <strong>   │ </strong>                 <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">The parameter overrides a preceding parameter by using the same name.</span>
  
</code></pre>

### Valid

```jsx
function i(i, b, c) {}
var j = function (j, b, c) {};
function k({ k, b }, { c, d }) {}
function l([, l]) {}
function foo([[a, b], [c, d]]) {}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
