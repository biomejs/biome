---
cli: minor
---

# Code actions via IDE/editor

Biome can now apply code actions of linter rules or assist actions directly from the settings of their IDE/editor.

For example, let's consider the lint rule [`noSwitchDeclarations`](https://biomejs.dev/linter/rules/no-switch-declarations/), which has an unsafe fix.
Nowadays, if you want to use this rule, you're "forced" to enable it via configuration, and if you want to apply its fix when you save the file, you're forced to make its fix safe, which could be a hazard for the project and team you work with:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noSwitchDeclarations": {
          "level": "error",
          "fix": "safe"
        }
      }
    }
  }
}
```

Now, you can benefit from the code action without making the fix safe for the entire project or for the entire team. IDEs and editor that are compatible LSP allow to list a series of "filters" or code actions that can be applied on save. In the case of VSCode, you will need to add the following snippet in the `settings.json`:

```json
{
  "editor.codeActionsOnSave": {
    "quickfix.biome.correctness.noSwitchDeclarations": "explicit"
  }
}
```

Upon save, the Biome LSP will tell the editor the apply the code action of the rule `noSwitchDeclarations`.
