---
title: useMediaCaption (since v1.0.0)
---

**Diagnostic Category: `lint/a11y/useMediaCaption`**

:::note
This rule is recommended by Biome. A diagnostic error will appear when linting your code.
:::

Source: <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/media-has-caption.md" target="_blank"><code>media-has-caption</code></a>

Enforces that `audio` and `video` elements must have a `track` for captions.

## Examples

### Invalid

```jsx
	<video />
```

<pre class="language-text"><code class="language-text">a11y/useMediaCaption.js:1:2 <a href="https://biomejs.dev/linter/rules/use-media-caption">lint/a11y/useMediaCaption</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>track</strong></span><span style="color: Tomato;"> for captions when using </span><span style="color: Tomato;"><strong>audio</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>video</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>	&lt;video /&gt;
   <strong>   │ </strong>	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.</span>
  
</code></pre>

```jsx
	<audio>child</audio>
```

<pre class="language-text"><code class="language-text">a11y/useMediaCaption.js:1:2 <a href="https://biomejs.dev/linter/rules/use-media-caption">lint/a11y/useMediaCaption</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a </span><span style="color: Tomato;"><strong>track</strong></span><span style="color: Tomato;"> for captions when using </span><span style="color: Tomato;"><strong>audio</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>video</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>	&lt;audio&gt;child&lt;/audio&gt;
   <strong>   │ </strong>	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.</span>
  
</code></pre>

### Valid

```jsx
	<audio>
		<track kind="captions" {...props} />
	</audio>
```

```jsx
	<video muted {...props}></video>
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
