---
title: noMisleadingCharacterClass (since v1.3.0)
---

**Diagnostic Category: `lint/nursery/noMisleadingCharacterClass`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Disallow characters which are made with multiple code points in character class syntax

Unicode includes the characters which are made with multiple code points. RegExp character class syntax (/[abc]/) cannot handle characters which are made by multiple code points as
expected. This rule reports the regular expressions which include multiple code point characters in character class syntax.

Source: https://eslint.org/docs/latest/rules/no-misleading-character-class

## Examples

### Invalid

```jsx
/^[Á]$/u;
/^[❇️]$/u;
/^[👶🏻]$/u;
/^[🇯🇵]$/u;
/^[👨‍👩‍👦]$/u;
/^[👍]$/;
```

<pre class="language-text"><code class="language-text">nursery/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/nursery/noMisleadingCharacterClass</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected combined character in character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[Á]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>/^[❇️]$/u;
    <strong>3 │ </strong>/^[👶🏻]$/u;
  
nursery/noMisleadingCharacterClass.js:1:11 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/nursery/noMisleadingCharacterClass</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Unexpected combined character in character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[Á]$/u;
   <strong>   │ </strong>         
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>/^[❇️]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>/^[👶🏻]$/u;
    <strong>4 │ </strong>/^[🇯🇵]$/u;
  
</code></pre>

## Valid

```jsx
/^[abc]$/;
/^[👍]$/u;
/^[\q{👶🏻}]$/v;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
