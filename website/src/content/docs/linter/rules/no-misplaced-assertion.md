---
title: noMisplacedAssertion (not released)
---

**Diagnostic Category: `lint/nursery/noMisplacedAssertion`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Succinct description of the rule.

Put context and details about the rule.
As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).

Try to stay consistent with the descriptions of implemented rules.

Add a link to the corresponding ESLint rule (if any):

## Examples

### Invalid

```jsx
var a = 1;
a = 2;
```

<pre class="language-text"><code class="language-text">nursery/noMisplacedAssertion.js:1:11 <a href="https://biomejs.dev/linter/rules/no-misplaced-assertion">lint/nursery/noMisplacedAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable is read here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a = 1;
   <strong>   │ </strong>          
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = 2;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This note will give you more information.</span>
  
</code></pre>

### Valid

```jsx
var a = 1;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
