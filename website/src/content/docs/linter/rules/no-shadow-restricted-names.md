---
title: noShadowRestrictedNames (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noShadowRestrictedNames`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-shadow-restricted-names" target="_blank"><code>no-shadow-restricted-names</code></a>

Disallow identifiers from shadowing restricted names.

## Examples

### Invalid

```jsx
function NaN() {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:10 <a href="https://biomejs.dev/linter/rules/no-shadow-restricted-names">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;NaN&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function NaN() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
let Set;
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:5 <a href="https://biomejs.dev/linter/rules/no-shadow-restricted-names">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Set&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let Set;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
try {	} catch(Object) {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:15 <a href="https://biomejs.dev/linter/rules/no-shadow-restricted-names">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Object&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>try {	} catch(Object) {}
   <strong>   │ </strong>     	        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
function Array() {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:10 <a href="https://biomejs.dev/linter/rules/no-shadow-restricted-names">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Array&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function Array() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
function test(JSON) {console.log(JSON)}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:15 <a href="https://biomejs.dev/linter/rules/no-shadow-restricted-names">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;JSON&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function test(JSON) {console.log(JSON)}
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
