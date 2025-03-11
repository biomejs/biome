---
"@biomejs/biome": major
---

Enable `.editorconfig` by default, default value of [`formatter.useEditorconfig`](https://biomejs.dev/reference/configuration/#formatteruseeditorconfig) set to `true`.

It will follow the following rules:

- Formatting settings in `biome.json` always take precedence over `.editorconfig` files.
- `.editorconfig` files that exist higher up in the hierarchy than a `biome.json` file are already ignored. This is to avoid loading formatting settings from someone's home directory into a project with a `biome.json` file.
- Nested `.editorconfig` files aren't supported.