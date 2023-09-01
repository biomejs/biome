---
title: useHtmlLang (since v1.0.0)
---


:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Enforce that `html` element has `lang` attribute.

## Examples

### Invalid

```jsx
<html></html>
```

<pre class="language-text"><code class="language-text">a11y/useHtmlLang.js:1:1 <a href="https://biomejs.dev/linter/rules/use-html-lang">lint/a11y/useHtmlLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute when using the </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html&gt;&lt;/html&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Setting a </span><span style="color: rgb(38, 148, 255);"><strong>lang</strong></span><span style="color: rgb(38, 148, 255);"> attribute on HTML document elements configures the languageused by screen readers when no user default is specified.</span>
  
</code></pre>

```jsx
<html lang={""}></html>
```

<pre class="language-text"><code class="language-text">a11y/useHtmlLang.js:1:1 <a href="https://biomejs.dev/linter/rules/use-html-lang">lint/a11y/useHtmlLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute when using the </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang={&quot;&quot;}&gt;&lt;/html&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Setting a </span><span style="color: rgb(38, 148, 255);"><strong>lang</strong></span><span style="color: rgb(38, 148, 255);"> attribute on HTML document elements configures the languageused by screen readers when no user default is specified.</span>
  
</code></pre>

```jsx
<html lang={null}></html>
```

<pre class="language-text"><code class="language-text">a11y/useHtmlLang.js:1:1 <a href="https://biomejs.dev/linter/rules/use-html-lang">lint/a11y/useHtmlLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute when using the </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang={null}&gt;&lt;/html&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Setting a </span><span style="color: rgb(38, 148, 255);"><strong>lang</strong></span><span style="color: rgb(38, 148, 255);"> attribute on HTML document elements configures the languageused by screen readers when no user default is specified.</span>
  
</code></pre>

```jsx
<html lang={undefined}></html>
```

<pre class="language-text"><code class="language-text">a11y/useHtmlLang.js:1:1 <a href="https://biomejs.dev/linter/rules/use-html-lang">lint/a11y/useHtmlLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute when using the </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang={undefined}&gt;&lt;/html&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Setting a </span><span style="color: rgb(38, 148, 255);"><strong>lang</strong></span><span style="color: rgb(38, 148, 255);"> attribute on HTML document elements configures the languageused by screen readers when no user default is specified.</span>
  
</code></pre>

```jsx
<html lang={true}></html>
```

<pre class="language-text"><code class="language-text">a11y/useHtmlLang.js:1:1 <a href="https://biomejs.dev/linter/rules/use-html-lang">lint/a11y/useHtmlLang</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>lang</strong></span><span style="color: Tomato;"> attribute when using the </span><span style="color: Tomato;"><strong>html</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;html lang={true}&gt;&lt;/html&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Setting a </span><span style="color: rgb(38, 148, 255);"><strong>lang</strong></span><span style="color: rgb(38, 148, 255);"> attribute on HTML document elements configures the languageused by screen readers when no user default is specified.</span>
  
</code></pre>

### Valid

```jsx
<html lang="en"></html>
```

```jsx
<html lang={language}></html>
```

```jsx
<html {...props}></html>
```

```jsx
<html lang={""} {...props}></html>
```

## Accessibility guidelines

- [WCAG 3.1.1](https://www.w3.org/WAI/WCAG21/Understanding/language-of-page)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
