---
"@biomejs/biome": major
---

`noUndeclaredVariables` no longer reports TypeScript types.

In TypeScript projects, developers often use global declaration files to declare global types.
Biome is currently unable to detect these global types.
This creates many false positives for `noUndeclaredVariables`.

TypeScript is better suited to perform this kind of check.
As proof of this, TypeScript ESLint doesn't provide any rule that extends the `no-undef` ESLint rule.

This is why Biome 1.9 introduced a new option `checkTypes` which, when it is set to false, ignores undeclared type references.
The option was set to `true` by default.

This option is now set to `false` by default.
To get the previous behavior, you have to set `checkTypes` to `true`:

```json
{
    "linter": {
        "rules": {
            "correctness": {
                "noUndeclaredVariables": {
                    "level": "on",
                    "options": { "checkTypes": true }
                }
            }
        }
    }
}
```
