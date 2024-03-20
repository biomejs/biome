---
title: noColorInvalidHex (not released)
---

**Diagnostic Category: `lint/nursery/noColorInvalidHex`**

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

Add a link to the corresponding stylelint rule (if any):

## Examples

### Invalid

```css
p {}
```

<pre class="language-text"><code class="language-text">nursery/noColorInvalidHex.js:1:3 <a href="https://biomejs.dev/linter/rules/no-color-invalid-hex">lint/nursery/noColorInvalidHex</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected empty block is not allowed</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>p {}
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">This note will give you more information.</span>
  
</code></pre>

### Valid

```css
p {
  color: red;
}
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
