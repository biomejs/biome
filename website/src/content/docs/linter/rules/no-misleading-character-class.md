---
title: noMisleadingCharacterClass (since v1.5.0)
---

**Diagnostic Category: `lint/suspicious/noMisleadingCharacterClass`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-misleading-character-class" target="_blank"><code>no-misleading-character-class</code></a>

Disallow characters made with multiple code points in character class syntax.

Unicode includes the characters which are made with multiple code points. e.g. Á, 🇯🇵, 👨‍👩‍👦.
A RegExp character class `/[abc]/` cannot handle characters with multiple code points.
For example, the character `❇️` consists of two code points: `❇` (U+2747) and `VARIATION SELECTOR-16` (U+FE0F).
If this character is in a RegExp character class, it will match to either `❇` or `VARIATION SELECTOR-16` rather than `❇️`.
This rule reports the regular expressions which include multiple code point characters in character class syntax.

## Examples

### Invalid

```jsx
/^[Á]$/u;
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected combined character in the character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[Á]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
/^[❇️]$/u;
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected combined character in the character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[❇️]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
/^[👶🏻]$/u;
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected modified Emoji in the character class. </span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[👶🏻]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
/^[🇯🇵]$/u;
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Regional indicator symbol characters should not be used in the character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[🇯🇵]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
/^[👨‍👩‍👦]$/u;
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> ━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected joined character sequence in character class.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[👨‍👩‍👦]$/u;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
/^[👍]$/; // surrogate pair without u flag
```

<pre class="language-text"><code class="language-text">suspicious/noMisleadingCharacterClass.js:1:1 <a href="https://biomejs.dev/linter/rules/no-misleading-character-class">lint/suspicious/noMisleadingCharacterClass</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected surrogate pair in character class. Use the 'u' flag.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^[👍]$/; // surrogate pair without u flag
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add unicode </span><span style="color: lightgreen;"><strong>u</strong></span><span style="color: lightgreen;"> flag to regex</span>
  
<strong>  </strong><strong>  1 │ </strong>/^[👍]$/<span style="color: MediumSeaGreen;">u</span>;<span style="opacity: 0.8;">·</span>//<span style="opacity: 0.8;">·</span>surrogate<span style="opacity: 0.8;">·</span>pair<span style="opacity: 0.8;">·</span>without<span style="opacity: 0.8;">·</span>u<span style="opacity: 0.8;">·</span>flag
<strong>  </strong><strong>    │ </strong>        <span style="color: MediumSeaGreen;">+</span>                                  
</code></pre>

### Valid

```jsx
/^[abc]$/;
/^[👍]$/u;
/^[\q{👶🏻}]$/v;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
