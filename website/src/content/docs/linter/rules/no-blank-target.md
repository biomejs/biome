---
title: noBlankTarget (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noBlankTarget`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-target-blank.md" target="_blank"><code>jsx-no-target-blank</code></a>

Disallow `target="_blank"` attribute without `rel="noreferrer"`

When creating anchor `a` element, there are times when its link has to be opened in a new browser tab
via `target="_blank"` attribute. This attribute has to paired with `rel="noreferrer"` or you're incur
in a security issue.

Refer to [the noreferrer documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer)
and the [the noopener documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noopener)

## Examples

### Invalid

```jsx
<a href='http://external.link' target='_blank'>child</a>
```

<pre class="language-text"><code class="language-text">a11y/noBlankTarget.js:1:32 <a href="https://biomejs.dev/linter/rules/no-blank-target">lint/a11y/noBlankTarget</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;"> without </span><span style="color: Tomato;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href='http://external.link' target='_blank'&gt;child&lt;/a&gt;
   <strong>   │ </strong>                               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Opening external links in new tabs without rel=&quot;noreferrer&quot; is a security risk. See </span><span style="color: lightgreen;"><a href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">the explanation</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add the </span><span style="color: lightgreen;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span>href='http://external.link'<span style="opacity: 0.8;">·</span>target='_blank'<span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">&quot;</span>&gt;child&lt;/a&gt;
<strong>  </strong><strong>    │ </strong>                                              <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>          
</code></pre>

```jsx
<a href='http://external.link' target='_blank' rel="noopener">child</a>
```

<pre class="language-text"><code class="language-text">a11y/noBlankTarget.js:1:32 <a href="https://biomejs.dev/linter/rules/no-blank-target">lint/a11y/noBlankTarget</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;"> without </span><span style="color: Tomato;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href='http://external.link' target='_blank' rel=&quot;noopener&quot;&gt;child&lt;/a&gt;
   <strong>   │ </strong>                               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Opening external links in new tabs without rel=&quot;noreferrer&quot; is a security risk. See </span><span style="color: lightgreen;"><a href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">the explanation</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add the </span><span style="color: lightgreen;"><strong>&quot;noreferrer&quot;</strong></span><span style="color: lightgreen;"> to the existing attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span>href='http://external.link'<span style="opacity: 0.8;">·</span>target='_blank'<span style="opacity: 0.8;">·</span>rel=&quot;<span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>noopener&quot;&gt;child&lt;/a&gt;
<strong>  </strong><strong>    │ </strong>                                                    <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>                   
</code></pre>

```jsx
<a {...props} href='http://external.link' target='_blank' rel="noopener">child</a>
```

<pre class="language-text"><code class="language-text">a11y/noBlankTarget.js:1:43 <a href="https://biomejs.dev/linter/rules/no-blank-target">lint/a11y/noBlankTarget</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid using </span><span style="color: Tomato;"><strong>target=&quot;_blank&quot;</strong></span><span style="color: Tomato;"> without </span><span style="color: Tomato;"><strong>rel=&quot;noreferrer&quot;</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a {...props} href='http://external.link' target='_blank' rel=&quot;noopener&quot;&gt;child&lt;/a&gt;
   <strong>   │ </strong>                                          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Opening external links in new tabs without rel=&quot;noreferrer&quot; is a security risk. See </span><span style="color: lightgreen;"><a href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">the explanation</a></span><span style="color: lightgreen;"> for more details.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Safe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Add the </span><span style="color: lightgreen;"><strong>&quot;noreferrer&quot;</strong></span><span style="color: lightgreen;"> to the existing attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span>{...props}<span style="opacity: 0.8;">·</span>href='http://external.link'<span style="opacity: 0.8;">·</span>target='_blank'<span style="opacity: 0.8;">·</span>rel=&quot;<span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">r</span><span style="opacity: 0.8;"><span style="color: MediumSeaGreen;">·</span></span>noopener&quot;&gt;child&lt;/a&gt;
<strong>  </strong><strong>    │ </strong>                                                               <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>                   
</code></pre>

### Valid

```jsx
<a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
```

```jsx
<a href='http://external.link' target='_blank' rel="noopener" {...props}>child</a>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
