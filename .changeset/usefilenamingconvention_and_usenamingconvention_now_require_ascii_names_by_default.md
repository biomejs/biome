---
"@biomejs/biome": major
---

The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer accepts non-ASCII characters by default.

Prior to Biome 2.0, non-ASCII names were accepted by default. They are now rejected.

For example, the following code is now reported as invalid by the `useNamingConvention` rule.

```js
let johnCafé;
```

If you want to allow non ASCII filenames and non-ASCII identifiers, you need to set the `requireAscii` options in your Biome configuration file to `false`:

```json
{
    "linter": {
        "rules": {
            "style": {
                "useFilenamingConvention": {
                    "level": "on",
                    "options": {
                        "requireAscii": false
                    }
                }
                "useFilenamingConvention": {
                    "level": "on",
                    "options": {
                        "requireAscii": false
                    }
                }
            }
        }
    }
}
```
