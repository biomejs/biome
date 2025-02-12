---
"@biomejs/biome": minor
---

Biome users can now configure code actions from linter rules as well as assist actions directly in the settings of their IDE/editor.

For example, let's consider the lint rule [`noSwitchDeclarations`](https://biomejs.dev/linter/rules/no-switch-declarations/), which has an unsafe fix.
Previously, if you wanted to use this rule, you were "forced" to enable it via configuration, and if you wanted to apply its fix when you saved a file, you were forced to mark the fix as safe:

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

Now, you can benefit from the code action without making the fix safe for the entire project. IDEs and editors that are LSP compatible allow to list a series of "filters" or code actions that can be applied on save. In the case of VS Code, you will need to add the following snippet in the `settings.json`:

```json
{
  "editor.codeActionsOnSave": {
    "quickfix.biome.correctness.noSwitchDeclarations": "explicit"
  }
}
```

Upon save, Biome will inform the editor the apply the code action of the rule `noSwitchDeclarations`.
