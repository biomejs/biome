---
title: noFontFamilyDuplicateNames (not released)
---

**Diagnostic Category: `lint/nursery/noFontFamilyDuplicateNames`**

:::danger
This rule hasn't been released yet.
:::

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Source: <a href="https://github.com/uhyo/eslint-plugin-import-access" target="_blank"><code>font-family-no-duplicate-names</code></a>

Disallow duplicate names within font families.

This rule checks the `font` and `font-family` properties for duplicate font names.

This rule ignores var(--custom-property) variable syntaxes.

## Examples

### Invalid

```css
a { font-family: "Lucida Grande", 'Arial', sans-serif, sans-serif; }
```

<pre class="language-text"><code class="language-text">nursery/noFontFamilyDuplicateNames.js:1:56 <a href="https://biomejs.dev/linter/rules/no-font-family-duplicate-names">lint/nursery/noFontFamilyDuplicateNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected duplicate font name: </span><span style="color: Tomato;"><strong>sans-serif</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a { font-family: &quot;Lucida Grande&quot;, 'Arial', sans-serif, sans-serif; }
   <strong>   │ </strong>                                                       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```css
a { font-family: 'Arial', "Lucida Grande", Arial, sans-serif; }
```

<pre class="language-text"><code class="language-text">nursery/noFontFamilyDuplicateNames.js:1:44 <a href="https://biomejs.dev/linter/rules/no-font-family-duplicate-names">lint/nursery/noFontFamilyDuplicateNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected duplicate font name: </span><span style="color: Tomato;"><strong>Arial</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a { font-family: 'Arial', &quot;Lucida Grande&quot;, Arial, sans-serif; }
   <strong>   │ </strong>                                           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```css
a { FONT: italic 300 16px/30px Arial, " Arial", serif; }
```

<pre class="language-text"><code class="language-text">nursery/noFontFamilyDuplicateNames.js:1:39 <a href="https://biomejs.dev/linter/rules/no-font-family-duplicate-names">lint/nursery/noFontFamilyDuplicateNames</a> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected duplicate font name: </span><span style="color: Tomato;"><strong>Arial</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a { FONT: italic 300 16px/30px Arial, &quot; Arial&quot;, serif; }
   <strong>   │ </strong>                                      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```css
a { font-family: "Lucida Grande", "Arial", sans-serif; }
```

```css
b { font: normal 14px/32px -apple-system, BlinkMacSystemFont, sans-serif; }
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
