---
title: useAsConstAssertion (since vnext)
---

**Diagnostic Category: `lint/nursery/useAsConstAssertion`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Succinct description of the rule.

Put context and details about the rule.
As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).

Try to stay consistent with the descriptions of implemented rules.

Add a link to the corresponding ESLint rule (if any):

Source: https://eslint.org/docs/latest/rules/rule-name

## Examples

### Invalid

```jsx
var a = 1;
a = 2;
```

<pre class="language-text"><code class="language-text">nursery/useAsConstAssertion.js:1:11 <a href="https://biomejs.dev/lint/rules/use-as-const-assertion">lint/nursery/useAsConstAssertion</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Variable is read here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var a = 1;
   <strong>   │ </strong>          
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = 2;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This note will give you more information.</span>
  
</code></pre>

## Valid

```jsx
var a = 1;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
