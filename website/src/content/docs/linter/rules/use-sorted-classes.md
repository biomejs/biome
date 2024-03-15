---
title: useSortedClasses (since v1.6.0)
---

**Diagnostic Category: `lint/nursery/useSortedClasses`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce the sorting of CSS utility classes.

This rule implements the same sorting algorithm as [Tailwind CSS](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted), but supports any utility class framework including [UnoCSS](https://unocss.dev/).

It is analogous to [`prettier-plugin-tailwindcss`](https://github.com/tailwindlabs/prettier-plugin-tailwindcss).

:::caution

## Important notes

This rule is a work in progress, and is only partially implemented. Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/1274

Currently, utility class sorting is **not part of the formatter**, and is implemented as a linter rule instead, with an automatic fix. The fix is, at this stage, classified as unsafe. This means that **it won't be applied automatically** as part of IDE actions such as "fix on save".

We appreciate any feedback on this rule, and encourage you to try it out and report any issues you find.

**Please read this entire documentation page before reporting an issue.**

Notably, keep in mind that the following features are not supported yet:

- Variant sorting.
- Custom utilitites and variants (such as ones introduced by Tailwind CSS plugins). Only the default Tailwind CSS configuration is supported.
- Options such as `prefix` and `separator`.
- Tagged template literals.
- Object properties (e.g. in `clsx` calls).

Please don't report issues about these features.
:::

## Examples

### Invalid

```jsx
<div class="px-2 foo p-4 bar" />;
```

<pre class="language-text"><code class="language-text">nursery/useSortedClasses.js:1:12 <a href="https://biomejs.dev/linter/rules/use-sorted-classes">lint/nursery/useSortedClasses</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">These CSS classes should be sorted.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;div class=&quot;px-2 foo p-4 bar&quot; /&gt;;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: lightgreen;">  </span></strong><strong><span style="color: lightgreen;">ℹ</span></strong> <span style="color: lightgreen;">Unsafe fix</span><span style="color: lightgreen;">: </span><span style="color: lightgreen;">Sort the classes.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">v</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">c</span><span style="color: Tomato;">l</span><span style="color: Tomato;">a</span><span style="color: Tomato;">s</span><span style="color: Tomato;">s</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><strong>p</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>-</strong></span><span style="color: Tomato;"><strong>2</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>f</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">p</span><span style="color: Tomato;">-</span><span style="color: Tomato;">4</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">&lt;</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">v</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><strong>f</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>r</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">-</span><span style="color: MediumSeaGreen;">4</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>p</strong></span><span style="color: MediumSeaGreen;"><strong>x</strong></span><span style="color: MediumSeaGreen;"><strong>-</strong></span><span style="color: MediumSeaGreen;"><strong>2</strong></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">&gt;</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Options

### Code-related

```json
{
    "options": {
        "attributes": ["classList"],
        "functions": ["clsx", "cva", "tw"]
    }
}
```

#### attributes

Classes in the `class` and `className` JSX attributes are always sorted. Use this option to add more attributes that should be sorted.

#### functions

If specified, strings in the indicated functions will be sorted. This is useful when working with libraries like [`clsx`](https://github.com/lukeed/clsx) or [`cva`](https://cva.style/).

```jsx
clsx("px-2 foo p-4 bar", {
    "block mx-4": condition,
});
```

Tagged template literals are also supported, for example:

```jsx
tw`px-2`;
tw.div`px-2`;
```

:::caution
Tagged template literal support has not been implemented yet.
:::

### Sort-related

:::caution
At the moment, this rule does not support customizing the sort options. Instead, the default Tailwind CSS configuration is hard-coded.
:::

## Differences with [Prettier](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)

The main key difference is that Tailwind CSS and its Prettier plugin read and execute the `tailwind.config.js` JavaScript file, which Biome can't do. Instead, Biome implements a simpler version of the configuration. The trade-offs are explained below.

### Values are not known

The rule has no knowledge of values such as colors, font sizes, or spacing values, which are normally defined in a configuration file like `tailwind.config.js`. Instead, the rule matches utilities that support values in a simpler way: if they start with a known utility prefix, such as `px-` or `text-`, they're considered valid.

This has two implications:

- False positives: classes can be wrongly recognized as utilities even though their values are incorrect. For example, if there's a `px-` utility defined in the configuration, it will match all of the following classes: `px-2`, `px-1337`, `px-[not-actually-valid]`, `px-literally-anything`.
- No distinction between different utilities that share the same prefix: for example, `text-red-500` and `text-lg` are both interpreted as the same type of utility by this rule, even though the former refers to a color and the latter to a font size. This results in all utilities that share the same prefix being sorted together, regardless of their actual values.

### Custom additions must be specified

The built-in Tailwind CSS preset (enabled by default) contains the set of utilities and variants that are available with the default configuration. More utilities and variants can be added through Tailwind CSS plugins. In Biome, these need to be manually specified in the Biome configuration file in order to "extend" the preset.

### Presets can't be modified

In Tailwind CSS, core plugins (which provide the default utilities and variants) can be disabled. In Biome, however, there is no way to disable parts of a preset: it's all or nothing. A work-around is to, instead of using a preset, manually specify all utilities and variants in the Biome configuration file.

### Whitespace is collapsed

The Tailwind CSS Prettier plugin preserves all original whitespace. This rule, however, collapses all whitespace (including newlines) into single spaces.

This is a deliberate decision. We're unsure about this behavior, and would appreciate feedback on it. If this is a problem for you, please share a detailed explanation of your use case in [the GitHub issue](https://github.com/biomejs/biome/issues/1274).

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
