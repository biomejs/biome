---
title: useValidLang (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useValidLang`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/lang.md" target="_blank"><code>lang</code></a>

Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.

## Examples

### Invalid

```jsx
<html lang="lorem" />
```

<pre class="language-text"><code class="language-text">a11y/useValidLang.js:1:12 <a href="https://biomejs.dev/linter/rules/use-valid-lang">lint/a11y/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;lorem&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Some of valid languages:</span>
  
  - ab
  - aa
  - af
  - sq
  - am
  - ar
  - an
  - hy
  - as
  - ay
  - az
  - ba
  - eu
  - bn
  - dz
  
</code></pre>

```jsx
<html lang="en-babab" />
```

<pre class="language-text"><code class="language-text">a11y/useValidLang.js:1:12 <a href="https://biomejs.dev/linter/rules/use-valid-lang">lint/a11y/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;en-babab&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Some of valid countries:</span>
  
  - AF
  - AL
  - DZ
  - AS
  - AD
  - AO
  - AI
  - AQ
  - AG
  - AR
  - AM
  - AW
  - AU
  - AT
  - AZ
  
</code></pre>

```jsx
<html lang="en-GB-typo" />
```

<pre class="language-text"><code class="language-text">a11y/useValidLang.js:1:12 <a href="https://biomejs.dev/linter/rules/use-valid-lang">lint/a11y/useValidLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang=&quot;en-GB-typo&quot; /&gt;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

### Valid

```jsx
<Html lang="en-babab" />
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
