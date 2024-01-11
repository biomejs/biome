---
title: noControlCharactersInRegex (since v1.0.0)
---

**Diagnostic Category: `lint/suspicious/noControlCharactersInRegex`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://eslint.org/docs/latest/rules/no-control-regex" target="_blank"><code>no-control-regex</code></a>

Prevents from having control characters and some escape sequences that match control characters in regular expressions.

Control characters are hidden special characters that are numbered from 0 to 31 in the ASCII system.
They're not commonly used in JavaScript text. So, if you see them in a pattern (called a regular expression), it's probably a mistake.

The following elements of regular expression patterns are considered possible errors in typing and are therefore disallowed by this rule:

- Hexadecimal character escapes from `\x00` to `\x1F`
- Unicode character escapes from `\u0000` to `\u001F`
- Unicode code point escapes from `\u{0}` to `\u{1F}`
- Unescaped raw characters from U+0000 to U+001F

Control escapes such as `\t` and `\n` are allowed by this rule.

## Examples

### Invalid

```jsx
 var pattern1 = /\x00/;
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\x00</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern1 = /\x00/;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern2 = /\x0C/;
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\x0C</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern2 = /\x0C/;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern3 = /\x1F/;
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\x1F</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern3 = /\x1F/;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern4 = /\u000C/;
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\u000C</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern4 = /\u000C/;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern5 = /\u{C}/u;
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\u{C}</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern5 = /\u{C}/u;
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern7 = new RegExp("\x0C");
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\x0C</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern7 = new RegExp(&quot;\x0C&quot;);
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

```jsx
 var pattern7 = new RegExp("\\x0C");
```

<pre class="language-text"><code class="language-text">suspicious/noControlCharactersInRegex.js:1:17 <a href="https://biomejs.dev/linter/rules/no-control-characters-in-regex">lint/suspicious/noControlCharactersInRegex</a> ━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected control character(s) in regular expression: </span><span style="color: Tomato;"><strong>\x0C</strong></span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong> var pattern7 = new RegExp(&quot;\\x0C&quot;);
   <strong>   │ </strong>                <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Control characters are unusual and potentially incorrect inputs, so they are disallowed.</span>
  
</code></pre>

### Valid

```jsx
var pattern1 = /\x20/;
var pattern2 = /\u0020/;
var pattern3 = /\u{20}/u;
var pattern4 = /\t/;
var pattern5 = /\n/;
var pattern6 = new RegExp("\x20");
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
