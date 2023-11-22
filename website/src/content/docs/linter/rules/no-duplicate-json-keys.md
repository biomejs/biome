---
title: noDuplicateJsonKeys (since v1.0.0)
---

**Diagnostic Category: `lint/nursery/noDuplicateJsonKeys`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow two keys with the same name inside a JSON object.

## Examples

### Invalid

```json
{
  "title": "New title",
  "title": "Second title"
}
```

<pre class="language-text"><code class="language-text">nursery/noDuplicateJsonKeys.js:2:3 <a href="https://biomejs.dev/linter/rules/no-duplicate-json-keys">lint/nursery/noDuplicateJsonKeys</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The key </span><span style="color: Tomato;"><strong>title</strong></span><span style="color: Tomato;"> was already declared.</span>
  
    <strong>1 │ </strong>{
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>  &quot;title&quot;: &quot;New title&quot;,
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>  &quot;title&quot;: &quot;Second title&quot;
    <strong>4 │ </strong>}
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This where a duplicated key was declared again.</span>
  
    <strong>1 │ </strong>{
    <strong>2 │ </strong>  &quot;title&quot;: &quot;New title&quot;,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>  &quot;title&quot;: &quot;Second title&quot;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">If a key is defined multiple times, only the last definition takes effect. Previous definitions are ignored.</span>
  
</code></pre>

### Valid

```json
{
  "title": "New title",
  "secondTitle": "Second title"
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
