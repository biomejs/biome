---
"@biomejs/biome": minor
---

Added configurable options to the [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators/) rule. Users can now customize the minimum number of digits required before adding separators and the group length for each type of numeric literal (`binary`, `octal`, `decimal`, `hexadecimal`).

```json
{
    "linter": {
        "rules": {
            "style": {
                "useNumericSeparators": {
                    "level": "error",
                    "options": {
                        "decimal": {
                            "minimumDigits": 7,
                            "groupLength": 3
                        },
                        "hexadecimal": {
                            "minimumDigits": 4,
                            "groupLength": 2
                        }
                    }
                }
            }
        }
    }
}
```
