---
"@biomejs/biome": minor
---

Enable `.editorconfig` by default, default value of [`formatter.useEditorconfig`](https://biomejs.dev/reference/configuration/#formatteruseeditorconfig) set to `true`.

It will follow the following rules:

- When `biome.json` and `.editorconfig` in the same directory, formatter settings from `biome.json` take precedence.
- If `biome.json` is not present or it exists without formatter settings, and an `.editorconfig` file exists in a directory higher up in the hierarchy, then default formatter settings should be used and the `.editorconfig` file should be ignored.