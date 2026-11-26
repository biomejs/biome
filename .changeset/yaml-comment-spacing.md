---
"@biomejs/biome": minor
---

Added two YAML formatter options that help the output pass yamllint's default rules and match ryl's formatter.

[`yaml.formatter.commentSpacing`](https://biomejs.dev/reference/configuration/#yamlformattercommentspacing) sets the exact number of spaces between content and an inline comment, from `1` to `4`. It defaults to `2`, which yamllint's `comments` rule expects. Set it to `1` to keep the previous output.

```diff
- - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
+ - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
```

[`yaml.formatter.trailingCommas`](https://biomejs.dev/reference/configuration/#yamlformattertrailingcommas) controls the comma after the last entry of a multiline flow mapping or sequence. It defaults to `all`, which keeps the current output. Set it to `none` to remove the comma, as ryl's formatter does.

```diff
  key:
    [
      first_value,
-     second_value,
+     second_value
    ]
```
