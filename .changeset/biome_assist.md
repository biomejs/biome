---
"@biomejs/biome": minor
---

Biome assist is a new feature of the Biome analyzer. The assist is meant to provide **actions**. Actions differ from linter rules in that they aren't meant to signal errors.

The assist will provide code actions that users can opt into via configuration or via IDEs/editors, using the Language Server Protocol.

The assist **is enabled by default**.  However, you can turn if off via configuration:

```json
{
  "assist": {
    "enabled": false
  }
}
```

You can turn on the actions that you want to use in your configuration. For example, you can enable the `useSortedKeys` action like this:

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

Alternatively, IDE/editor users can decide which action to apply on save *directly from the editor settings*, as long as the assist is enabled.

For example, in VS Code you can apply the `useSortedKeys` action when saving a file by adding the following snippet in `settings.json`:

```json
{
  "editor.codeActionsOnSave": {
    "source.biome.useSortedKeys": "explicit"
  }
}
```

In Zed, you can achieve the same by adding the following snippet in `~/.config/zed/settings.json`:

```json
{
  "code_actions_on_format": {
    "source.biome.useSortedKeys": true
  }
}
```
