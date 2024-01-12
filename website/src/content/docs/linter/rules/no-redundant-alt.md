---
title: noRedundantAlt (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noRedundantAlt`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md" target="_blank"><code>no-redundant-roles</code></a>

Enforce `img` alt prop does not contain the word "image", "picture", or "photo".

The rule will first check if `aria-hidden` is truthy to determine whether to enforce the rule. If the image is
hidden, then the rule will always succeed.

## Examples

### Invalid

```jsx
<img src="src" alt="photo content" />;
```

<pre class="language-text"><code class="language-text">a11y/noRedundantAlt.js:1:20 <a href="https://biomejs.dev/linter/rules/no-redundant-alt">lint/a11y/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img src=&quot;src&quot; alt=&quot;photo content&quot; /&gt;;
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

```jsx
<img alt={`picture doing ${things}`} {...this.props} />;
```

<pre class="language-text"><code class="language-text">a11y/noRedundantAlt.js:1:10 <a href="https://biomejs.dev/linter/rules/no-redundant-alt">lint/a11y/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img alt={`picture doing ${things}`} {...this.props} /&gt;;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

```jsx
<img alt="picture of cool person" aria-hidden={false} />;
```

<pre class="language-text"><code class="language-text">a11y/noRedundantAlt.js:1:10 <a href="https://biomejs.dev/linter/rules/no-redundant-alt">lint/a11y/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in </span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img alt=&quot;picture of cool person&quot; aria-hidden={false} /&gt;;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

### Valid

```jsx
<>
	<img src="src" alt="alt" />
	<img src="src" alt={photo} />
	<img src="bar" aria-hidden alt="Picture of me taking a photo of an image" />
</>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
