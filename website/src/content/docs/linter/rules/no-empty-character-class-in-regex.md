---
title: noEmptyCharacterClassInRegex (since v1.3.0)
---

**Diagnostic Category: `lint/correctness/noEmptyCharacterClassInRegex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-empty-character-class" target="_blank"><code>no-empty-character-class</code></a>

Disallow empty character classes in regular expression literals.

Empty character classes don't match anything.
In contrast, negated empty classes match any character.
They are often the result of a typing mistake.

## Examples

### Invalid

```jsx
/^a[]/.test("a"); // false
```

<pre class="language-text"><code class="language-text">correctness/noEmptyCharacterClassInRegex.js:1:4 <a href="https://biomejs.dev/linter/rules/no-empty-character-class-in-regex">lint/correctness/noEmptyCharacterClassInRegex</a> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The regular expression includes this </span><span style="color: Tomato;"><strong>empty character class</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^a[]/.test(&quot;a&quot;); // false
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Empty character classes don't match anything.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">If you want to match against </span><span style="color: lightgreen;"><strong>[</strong></span><span style="color: lightgreen;">, escape it </span><span style="color: lightgreen;"><strong>\[</strong></span><span style="color: lightgreen;">.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Otherwise, remove the character class or fill it.</span>
  
</code></pre>

```jsx
/^a[^]/.test("ax"); // true
```

<pre class="language-text"><code class="language-text">correctness/noEmptyCharacterClassInRegex.js:1:4 <a href="https://biomejs.dev/linter/rules/no-empty-character-class-in-regex">lint/correctness/noEmptyCharacterClassInRegex</a> ━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The regular expression includes this </span><span style="color: Tomato;"><strong>negated empty character class</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>/^a[^]/.test(&quot;ax&quot;); // true
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Negated empty character classes match anything.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">If you want to match against </span><span style="color: lightgreen;"><strong>[</strong></span><span style="color: lightgreen;">, escape it </span><span style="color: lightgreen;"><strong>\[</strong></span><span style="color: lightgreen;">.
</span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">  </span><span style="color: lightgreen;">Otherwise, remove the character class or fill it.</span>
  
</code></pre>

### Valid

```jsx
/^a[xy]/.test("ay"); // true
```

```jsx
/^a[^xy]/.test("ab"); // true
```

```jsx
/^a\[]/.test("a[]"); // true
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
