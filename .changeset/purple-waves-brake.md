---
"@biomejs/biome": minor
---

Added a feature that allows editors to inject a Biome configuration to the Biome Language Server without affecting the configuration of the project.

If you have a Biome extension that is compatible with your preferred LSP-ready editor, you can map `inlineConfig`. The configuration will be merged with the configuration of the project (or the default configuration):

For example, with the Zed editor, you would have the following configuration, which will format all files using four spaces as indentation style:

```json5
// .zed/settings.json
{
  "lsp": {
    "biome": {
      "settings": {
        "inline_config": {
          "formatter": {
            "indentStyle": "space",
            "indentWidth":  4
          }
        }
      }
    }
  }
}
```
