---
title: Lint Rule noShadowRestrictedNames
parent: lint/rules/index
---

# noShadowRestrictedNames (since v0.9.0)

> This rule is recommended by Biome.

Disallow identifiers from shadowing restricted names.

## Examples

### Invalid

```jsx
function NaN() {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:10 <a href="https://biomejs.dev/lint/rules/noShadowRestrictedNames">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;NaN&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function NaN() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
let Set;
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:5 <a href="https://biomejs.dev/lint/rules/noShadowRestrictedNames">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Set&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let Set;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
try {	} catch(Object) {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:15 <a href="https://biomejs.dev/lint/rules/noShadowRestrictedNames">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Object&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>try {	} catch(Object) {}
   <strong>   │ </strong>     	        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
function Array() {}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:10 <a href="https://biomejs.dev/lint/rules/noShadowRestrictedNames">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;Array&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function Array() {}
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

```jsx
function test(JSON) {console.log(JSON)}
```

<pre class="language-text"><code class="language-text">suspicious/noShadowRestrictedNames.js:1:15 <a href="https://biomejs.dev/lint/rules/noShadowRestrictedNames">lint/suspicious/noShadowRestrictedNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not shadow the global &quot;JSON&quot; property.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function test(JSON) {console.log(JSON)}
   <strong>   │ </strong>              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.</span>
  
</code></pre>

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
