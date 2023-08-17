---
title: Lint Rule noDuplicateParameters
parent: lint/rules/index
---

# noDuplicateParameters (since v0.9.0)

> This rule is recommended by Rome.

Disallow duplicate function parameter name.

If more than one parameter has the same name in a function definition,
the last occurrence overrides the preceding occurrences.
A duplicated name might be a typing error.

Source: https://eslint.org/docs/latest/rules/no-dupe-args

## Examples

### Invalid

```jsx
var f = function(a, b, b) {}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateParameters.js:1:24 <a href="https://biomejs.dev/lint/rules/noDuplicateParameters">lint/suspicious/noDuplicateParameters</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate parameter name.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var f = function(a, b, b) {}
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The parameter overrides a preceding parameter by using the same name.</span>
  
</code></pre>

```jsx
function b(a, b, b) {}
```

<pre class="language-text"><code class="language-text">suspicious/noDuplicateParameters.js:1:18 <a href="https://biomejs.dev/lint/rules/noDuplicateParameters">lint/suspicious/noDuplicateParameters</a> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate parameter name.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function b(a, b, b) {}
   <strong>   │ </strong>                 <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The parameter overrides a preceding parameter by using the same name.</span>
  
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
