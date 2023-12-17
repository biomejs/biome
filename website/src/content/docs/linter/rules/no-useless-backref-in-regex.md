---
title: noUselessBackrefInRegex (since v1.5.0)
---

**Diagnostic Category: `lint/nursery/noUselessBackrefInRegex`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Detects and warns about unnecessary backreferences in regular expressions.

Regular expressions in JavaScript allow backreferences using \1, \2, etc., to match the same text as previously matched by a capturing group.
However, misusing or overusing backreferences can make regular expressions hard to read and inefficient.
This rule identifies such unnecessary backreferences.

## Examples

### Invalid

```jsx
var regex = /(a)\1/;
```

<pre class="language-text"><code class="language-text">nursery/noUselessBackrefInRegex.js:1:13 <a href="https://biomejs.dev/linter/rules/no-useless-backreference-in-regex">lint/nursery/noUselessBackrefInRegex</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This regular expression contains an unnecessary backreference.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var regex = /(a)\1/;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

## Valid

```jsx
var regex = /(a)\1b\2/; // Valid if there's a corresponding second group
var regex = /(a)b\1/;   // Valid use of backreference
```

nursery/noUselessBackrefInRegex.js:1:13 <a href="https://biomejs.dev/linter/rules/no-useless-backreference-in-regex">lint/nursery/noUselessBackrefInRegex</a> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This regular expression contains an unnecessary backreference.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var regex = /(a)\1b\2/; // Valid if there's a corresponding second group
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>var regex = /(a)b\1/;   // Valid use of backreference
    <strong>3 │ </strong>
  
## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
