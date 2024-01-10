---
title: noAccessKey (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/noAccessKey`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-access-key.md" target="_blank"><code>no-access-key</code></a>

Enforce that the `accessKey` attribute is not used on any HTML element.

The `accessKey` assigns a keyboard shortcut to the current element. However, the `accessKey` value
can conflict with keyboard commands used by screen readers and keyboard-only users, which leads to
inconsistent keyboard actions across applications. To avoid accessibility complications,
this rule suggests users remove the `accessKey` attribute on elements.

## Examples

### Invalid

```jsx
<input type="submit" accessKey="s" value="Submit" />
```

<pre class="language-text"><code class="language-text">a11y/noAccessKey.js:1:22 <a href="https://biomejs.dev/linter/rules/no-access-key">lint/a11y/noAccessKey</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input type=&quot;submit&quot; accessKey=&quot;s&quot; value=&quot;Submit&quot; /&gt;
   <strong>   │ </strong>                     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Assigning keyboard shortcuts using the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute leads to inconsistent keyboard actions across applications.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span>type=&quot;submit&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">c</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;">K</span><span style="color: Tomato;">e</span><span style="color: Tomato;">y</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">&quot;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>value=&quot;Submit&quot;<span style="opacity: 0.8;">·</span>/&gt;
<strong>  </strong><strong>    │ </strong>                     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>                 
</code></pre>

```jsx
<a href="https://webaim.org/" accessKey="w">WebAIM.org</a>
```

<pre class="language-text"><code class="language-text">a11y/noAccessKey.js:1:31 <a href="https://biomejs.dev/linter/rules/no-access-key">lint/a11y/noAccessKey</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href=&quot;https://webaim.org/&quot; accessKey=&quot;w&quot;&gt;WebAIM.org&lt;/a&gt;
   <strong>   │ </strong>                              <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Assigning keyboard shortcuts using the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute leads to inconsistent keyboard actions across applications.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;a<span style="opacity: 0.8;">·</span>href=&quot;https://webaim.org/&quot;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">c</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;">K</span><span style="color: Tomato;">e</span><span style="color: Tomato;">y</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">w</span><span style="color: Tomato;">&quot;</span>&gt;WebAIM.org&lt;/a&gt;
<strong>  </strong><strong>    │ </strong>                              <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>               
</code></pre>

```jsx
<button accessKey="n">Next</button>
```

<pre class="language-text"><code class="language-text">a11y/noAccessKey.js:1:9 <a href="https://biomejs.dev/linter/rules/no-access-key">lint/a11y/noAccessKey</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the </span><span style="color: Tomato;"><strong>accessKey</strong></span><span style="color: Tomato;"> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;button accessKey=&quot;n&quot;&gt;Next&lt;/button&gt;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Assigning keyboard shortcuts using the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute leads to inconsistent keyboard actions across applications.</span>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Remove the </span><span style="color: lightgreen;"><strong>accessKey</strong></span><span style="color: lightgreen;"> attribute.</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;button<span style="opacity: 0.8;">·</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">c</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;">K</span><span style="color: Tomato;">e</span><span style="color: Tomato;">y</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">n</span><span style="color: Tomato;">&quot;</span>&gt;Next&lt;/button&gt;
<strong>  </strong><strong>    │ </strong>        <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>              
</code></pre>

## Resources

- [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)
- [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
