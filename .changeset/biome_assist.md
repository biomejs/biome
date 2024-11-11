---
cli: minor
---

# Biome assist

Biome assist a new product of the Biome analyzer. The assist is meant to provide "rules", called **actions**, that aren't meant to signal errors, like in the linter rules.

The assist, with time, will provide code actions that users can opt in via configuration or via IDEs/editors, using the Language Server Protocol.

The assist **is enabled by default**.  However, you can turn if off via configuration:

```json
{
  "assist": {
    "enabled": false
  }
}
```

You can turn on the actions that you want to use in your configuration. For example, to you can enable the `useSortedKeys` action:

```json
{
  "assist": {
    "actions": {
      "source": {
        "useSortedKeys": "on"
      }
    }
  }
}
```

Alternatively, IDE/editor users can decide which action to apply on save *straight from the editor settings*, as long as the assist is enabled:

For example, in VSCode you would apply the `useSortedKeys` action when saving a file by adding the following snippet in `settings.json`:

```json
{
  "editor.codeActionsOnSave": {
    "source.biome.useSortedKeys": "explicit"
  }
}
```

In Zed, instead, you would need to add the following snippet in `~/.config/zed/settings.json`:

```json
{
  "code_actions_on_format": {
    "source.biome.useSortedKeys": true
  }
}
```
